"""
External API client for fetching data from various external services
"""
import asyncio
import json
from typing import Dict, List, Any, Optional, Tuple
import httpx
from datetime import datetime, timezone

from ..utils import get_logger

logger = get_logger(__name__)


class ExternalApiClient:
    """Client for interacting with external APIs"""
    
    def __init__(self, base_url: str, headers: Optional[Dict[str, str]] = None, 
                 auth_config: Optional[Dict[str, Any]] = None, timeout: int = 30):
        self.base_url = base_url.rstrip('/')
        self.headers = headers or {}
        self.auth_config = auth_config or {}
        self.timeout = timeout
        
        # Setup authentication headers
        self._setup_auth_headers()
    
    def _setup_auth_headers(self):
        """Setup authentication headers based on auth_config"""
        if not self.auth_config:
            return
            
        auth_type = self.auth_config.get('type', '').lower()
        
        if auth_type == 'bearer':
            token = self.auth_config.get('token')
            if token:
                self.headers['Authorization'] = f'Bearer {token}'
        
        elif auth_type == 'api_key':
            key = self.auth_config.get('key')
            header_name = self.auth_config.get('header_name', 'X-API-Key')
            if key:
                self.headers[header_name] = key
        
        elif auth_type == 'basic':
            username = self.auth_config.get('username')
            password = self.auth_config.get('password')
            if username and password:
                import base64
                credentials = base64.b64encode(f'{username}:{password}'.encode()).decode()
                self.headers['Authorization'] = f'Basic {credentials}'
    
    async def test_connection(self) -> Dict[str, Any]:
        """Test the API connection without fetching data"""
        try:
            async with httpx.AsyncClient(timeout=self.timeout) as client:
                start_time = datetime.now(timezone.utc)
                
                response = await client.get(
                    self.base_url,
                    headers=self.headers,
                    follow_redirects=True
                )
                
                end_time = datetime.now(timezone.utc)
                duration = int((end_time - start_time).total_seconds() * 1000)
                
                logger.info(f"Connection test completed: {response.status_code} in {duration}ms")
                
                # 샘플 데이터 추출 시도 (성공 시에만)
                sample_data = None
                if response.status_code < 400:
                    try:
                        content_type = response.headers.get("content-type", "")
                        if "application/json" in content_type:
                            json_data = response.json()
                            # 배열인 경우 첫 번째 항목을 샘플로 사용
                            if isinstance(json_data, list) and len(json_data) > 0:
                                sample_data = json_data[0]
                            elif isinstance(json_data, dict):
                                sample_data = json_data
                        else:
                            # JSON이 아닌 경우 샘플 응답 텍스트를 제공
                            response_text = response.text[:500]  # 처음 500자만
                            sample_data = {"response_text": response_text, "content_type": content_type}
                    except Exception as e:
                        logger.warning(f"Failed to parse sample data: {str(e)}")
                        sample_data = None
                
                result = {
                    "status": "success" if response.status_code < 400 else "error",
                    "status_code": response.status_code,
                    "response_time_ms": duration,
                    "content_type": response.headers.get("content-type", ""),
                    "message": f"Connected successfully (HTTP {response.status_code})" if response.status_code < 400 
                              else f"Connection failed (HTTP {response.status_code})"
                }
                
                if sample_data is not None:
                    result["sample_data"] = sample_data
                    
                return result
                
        except httpx.TimeoutException:
            logger.warning(f"Connection test timed out for {self.base_url}")
            return {
                "status": "error",
                "message": "Connection timeout"
            }
        except httpx.ConnectError as e:
            logger.error(f"Connection error for {self.base_url}: {str(e)}")
            return {
                "status": "error",
                "message": f"Connection error: {str(e)}"
            }
        except Exception as e:
            logger.error(f"Unexpected error during connection test: {str(e)}")
            return {
                "status": "error",
                "message": f"Unexpected error: {str(e)}"
            }
    
    async def fetch_data(self, endpoint: str = "", method: str = "GET", 
                        body: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        """Fetch data from the external API"""
        url = f"{self.base_url}/{endpoint.lstrip('/')}" if endpoint else self.base_url
        
        try:
            async with httpx.AsyncClient(timeout=self.timeout) as client:
                start_time = datetime.now(timezone.utc)
                
                request_data = {
                    "url": url,
                    "method": method,
                    "headers": self.headers,
                    "follow_redirects": True
                }
                
                if body and method.upper() in ['POST', 'PUT', 'PATCH']:
                    request_data["json"] = body
                
                response = await client.request(**request_data)
                
                end_time = datetime.now(timezone.utc)
                duration = int((end_time - start_time).total_seconds() * 1000)
                
                logger.info(f"API request completed: {method} {url} -> {response.status_code} in {duration}ms")
                
                if response.status_code >= 400:
                    error_msg = f"API request failed: HTTP {response.status_code}"
                    try:
                        error_content = response.json()
                        error_msg += f" - {error_content}"
                    except:
                        error_msg += f" - {response.text}"
                    
                    return {
                        "status": "error",
                        "error": error_msg,
                        "status_code": response.status_code,
                        "duration_ms": duration
                    }
                
                # Parse response data
                content_type = response.headers.get("content-type", "").lower()
                
                if "application/json" in content_type:
                    data = response.json()
                elif "text/" in content_type:
                    # Try to parse as JSON first, fallback to text
                    try:
                        data = response.json()
                    except:
                        data = {"text_content": response.text}
                else:
                    data = {"raw_content": response.content.decode('utf-8', errors='ignore')}
                
                # Normalize data to list format for consistent processing
                if isinstance(data, dict):
                    # If it's a single object, wrap in list
                    if self._looks_like_data_object(data):
                        data = [data]
                    # If it has a data/results/items field, extract it
                    elif 'data' in data and isinstance(data['data'], list):
                        data = data['data']
                    elif 'results' in data and isinstance(data['results'], list):
                        data = data['results']
                    elif 'items' in data and isinstance(data['items'], list):
                        data = data['items']
                    else:
                        data = [data]
                elif not isinstance(data, list):
                    data = [{"content": data}]
                
                return {
                    "status": "success",
                    "data": data,
                    "status_code": response.status_code,
                    "duration_ms": duration,
                    "records_count": len(data) if isinstance(data, list) else 1
                }
                
        except httpx.TimeoutException:
            error_msg = f"Request timeout for {url}"
            logger.warning(error_msg)
            return {
                "status": "error",
                "error": error_msg
            }
        except httpx.ConnectError as e:
            error_msg = f"Connection error for {url}: {str(e)}"
            logger.error(error_msg)
            return {
                "status": "error",
                "error": error_msg
            }
        except json.JSONDecodeError as e:
            error_msg = f"Invalid JSON response from {url}: {str(e)}"
            logger.error(error_msg)
            return {
                "status": "error",
                "error": error_msg
            }
        except Exception as e:
            error_msg = f"Unexpected error fetching data from {url}: {str(e)}"
            logger.error(error_msg)
            return {
                "status": "error",
                "error": error_msg
            }
    
    def _looks_like_data_object(self, obj: Dict[str, Any]) -> bool:
        """Determine if a dict looks like a data object rather than a wrapper"""
        wrapper_keys = {'data', 'results', 'items', 'records', 'response'}
        obj_keys = set(obj.keys())
        
        # If it has common wrapper keys, it's probably a wrapper
        if any(key in obj_keys for key in wrapper_keys):
            return False
        
        # If it has ID-like fields, it's probably a data object
        id_keys = {'id', '_id', 'uuid', 'key', 'identifier'}
        if any(key in obj_keys for key in id_keys):
            return True
        
        # If it has many fields, it's probably a data object
        if len(obj_keys) > 3:
            return True
        
        return True