// src/lib/api/config.ts
import { env } from '$env/dynamic/public';

const FALLBACK_API_URL = 'http://localhost:8000';

export const API_URL = env.PUBLIC_API_URL || FALLBACK_API_URL;
