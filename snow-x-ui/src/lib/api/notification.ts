import { privateApi } from './private';

export interface NotificationDiffEntry {
	field: string;
	before?: string | number | boolean | null;
	after?: string | number | boolean | null;
}

export interface NotificationPayload {
	rack_id?: string;
	server_room_id?: string;
	server_room_name?: string;
	rack_height?: number;
	created_by?: string;
	actor_id?: string;
	actor_name?: string;
	actor_handle?: string;
	actor_email?: string;
	link?: string;
	diff?: NotificationDiffEntry[];
	[key: string]: unknown;
}

export interface Notification {
	id: string;
	tenant_id?: string;
	channel: string;
	category?: string;
	title?: string;
	message?: string;
	payload?: NotificationPayload | null;
	status: string;
	retry_count: number;
	max_retries: number;
	last_error?: string;
	scheduled_at: string;
	processing_started_at?: string;
	processed_at?: string;
	created_at: string;
	updated_at: string;
}

export interface NotificationListResponse {
	notifications: Notification[];
	total: number;
	page: number;
	limit: number;
}

export interface NotificationListParams {
	page?: number;
	limit?: number;
	status?: string;
	channel?: string;
}

export interface CreateNotificationRequest {
	channel: string;
	category?: string;
	title?: string;
	message?: string;
	payload?: Record<string, unknown>;
	tenant_id?: string;
	scheduled_at?: string;
	max_retries?: number;
}

export interface UpdateNotificationStatusRequest {
	status: string;
	last_error?: string;
}

export const notificationApi = {
	async getNotifications(params?: NotificationListParams): Promise<NotificationListResponse> {
		const searchParams = new URLSearchParams();
		if (params?.page) searchParams.append('page', params.page.toString());
		if (params?.limit) searchParams.append('limit', params.limit.toString());
		if (params?.status) searchParams.append('status', params.status);
		if (params?.channel) searchParams.append('channel', params.channel);

		const url = `v0/notifications${searchParams.toString() ? `?${searchParams.toString()}` : ''}`;
		return await privateApi.get(url).json<NotificationListResponse>();
	},

	async createNotification(data: CreateNotificationRequest): Promise<Notification> {
		return await privateApi.post('v0/notifications', { json: data }).json<Notification>();
	},

	async updateNotificationStatus(id: string, data: UpdateNotificationStatusRequest): Promise<Notification> {
		return await privateApi.patch(`v0/notifications/${id}`, { json: data }).json<Notification>();
	}
};
