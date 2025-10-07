import type { KyInstance } from 'ky';
import { privateApi } from './private.ts';

export interface FieldMapping {
	mappings: {
		source_field: string;
		target_field: string;
		data_type: 'string' | 'number' | 'boolean' | 'date';
		default_value?: string;
		transformation?: 'uppercase' | 'lowercase' | 'trim' | 'title';
	}[];
	filter_conditions: {
		field: string;
		operator: 'equals' | 'contains' | 'not_equals';
		value: string;
	}[];
}

export interface ExternalApiConnection {
	id: number;
	name: string;
	base_url: string;
	description?: string;
	headers?: Record<string, string>;
	auth_config?: Record<string, any>;
	field_mapping?: FieldMapping;
	sync_interval: number;
	is_active: boolean;
	auto_sync: boolean;
	last_sync_at?: string;
	next_sync_at?: number;
	sync_count: number;
	last_error_message?: string;
	created_at: string;
	updated_at: string;
}

export interface CreateConnectionRequest {
	name: string;
	base_url: string;
	description?: string;
	headers?: Record<string, string>;
	auth_config?: Record<string, any>;
	field_mapping?: FieldMapping;
	sync_interval?: number;
	is_active?: boolean;
	auto_sync?: boolean;
}

export interface TestConnectionRequest {
	base_url: string;
	headers?: Record<string, string>;
	auth_config?: Record<string, any>;
}

export interface ExternalApiSyncLog {
	id: number;
	connection_id: number;
	status: 'success' | 'error' | 'in_progress';
	request_url: string;
	request_method: string;
	response_status?: number;
	records_processed: number;
	error_message?: string;
	duration?: number;
	started_at: string;
	completed_at?: string;
}

export interface ExternalApiData {
	id: number;
	connection_id: number;
	external_id?: string;
	data_type: string;
	raw_data: any;
	processed_data?: any;
	hash: string;
	status: string;
	last_sync_at: string;
	created_at: string;
	updated_at: string;
}

export interface ApiStats {
	total_connections: number;
	active_connections: number;
	today_syncs: number;
	today_failed_syncs: number;
	total_active_records: number;
}

class ExternalApiService {
	constructor(private client: KyInstance) {}

	// Get external API task service URL - port 7000 for Python tasks API
	private get tasksUrl() {
		return 'http://localhost:7000';
	}

	private async tasksRequest(endpoint: string, options?: any) {
		// Direct fetch to tasks API since ky might be configured for main API
		const url = `${this.tasksUrl}${endpoint}`;
		const response = await fetch(url, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				...options?.headers
			}
		});

		if (!response.ok) {
			const error = await response.json().catch(() => ({ message: 'API Error' }));
			throw new Error(error.message || `HTTP ${response.status}`);
		}

		return response.json();
	}

	async listConnections(skip = 0, limit = 100): Promise<ExternalApiConnection[]> {
		return this.tasksRequest(`/tasks/external-api/connections?skip=${skip}&limit=${limit}`);
	}

	async getConnection(connectionId: number): Promise<ExternalApiConnection> {
		return this.tasksRequest(`/tasks/external-api/connections/${connectionId}`);
	}

	async createConnection(request: CreateConnectionRequest): Promise<ExternalApiConnection> {
		return this.tasksRequest('/tasks/external-api/connections', {
			method: 'POST',
			body: JSON.stringify(request)
		});
	}

	async updateConnection(
		connectionId: number,
		request: Partial<CreateConnectionRequest>
	): Promise<ExternalApiConnection> {
		return this.tasksRequest(`/tasks/external-api/connections/${connectionId}`, {
			method: 'PUT',
			body: JSON.stringify(request)
		});
	}

	async deleteConnection(connectionId: number): Promise<{ message: string }> {
		return this.tasksRequest(`/tasks/external-api/connections/${connectionId}`, {
			method: 'DELETE'
		});
	}

	async testConnection(
		request: TestConnectionRequest
	): Promise<{ status: string; message: string; status_code?: number; response_time_ms?: number }> {
		return this.tasksRequest('/tasks/external-api/test-connection', {
			method: 'POST',
			body: JSON.stringify(request)
		});
	}

	async syncConnection(connectionId: number): Promise<{ message: string; task_id: string; connection_id: number }> {
		return this.tasksRequest(`/tasks/external-api/connections/${connectionId}/sync`, {
			method: 'POST'
		});
	}

	async syncAllConnections(): Promise<{ message: string; task_id: string }> {
		return this.tasksRequest('/tasks/external-api/sync-all', {
			method: 'POST'
		});
	}

	async getSyncLogs(connectionId: number, skip = 0, limit = 50): Promise<ExternalApiSyncLog[]> {
		return this.tasksRequest(`/tasks/external-api/connections/${connectionId}/logs?skip=${skip}&limit=${limit}`);
	}

	async getApiData(connectionId: number, skip = 0, limit = 100, dataType?: string): Promise<ExternalApiData[]> {
		const params = new URLSearchParams({ skip: skip.toString(), limit: limit.toString() });
		if (dataType) params.set('data_type', dataType);
		return this.tasksRequest(`/tasks/external-api/connections/${connectionId}/data?${params}`);
	}

	async getStats(): Promise<ApiStats> {
		return this.tasksRequest('/tasks/external-api/stats');
	}
}

// Create the service instance with private client
export const externalApiService = new ExternalApiService(privateApi);
