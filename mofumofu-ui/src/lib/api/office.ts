import type { HTTPError } from 'ky';
import { privateApi } from './private';

export interface Office {
    id: string;
    name: string;
    description?: string;
    address: string;
    contact_person?: string;
    phone?: string;
    email?: string;
    created_by: string;
    created_at: string;
    updated_at: string;
    is_active: boolean;
}

export interface CreateOfficeRequest {
    name: string;
    description?: string;
    address: string;
    contact_person?: string;
    phone?: string;
    email?: string;
}

export interface UpdateOfficeRequest {
    name?: string;
    description?: string;
    address?: string;
    contact_person?: string;
    phone?: string;
    email?: string;
}

export interface OfficeListResponse {
    offices: Office[];
    total: number;
    page: number;
    limit: number;
}

export interface OfficeListParams {
    page?: number;
    limit?: number;
    search?: string;
}

// Office management API calls
export const officeApi = {
    // Create new office
    async createOffice(data: CreateOfficeRequest): Promise<Office> {
        try {
            return await privateApi.post('v0/ipam/office', { json: data }).json<Office>();
        } catch (error) {
            console.error('Failed to create office:', error);
            throw error;
        }
    },

    // Get list of offices
    async getOffices(params?: OfficeListParams): Promise<OfficeListResponse> {
        try {
            const searchParams = new URLSearchParams();
            if (params?.page) searchParams.append('page', params.page.toString());
            if (params?.limit) searchParams.append('limit', params.limit.toString());
            if (params?.search) searchParams.append('search', params.search);

            const url = `v0/ipam/office${searchParams.toString() ? `?${searchParams.toString()}` : ''}`;
            return await privateApi.get(url).json<OfficeListResponse>();
        } catch (error) {
            console.error('Failed to get offices:', error);
            throw error;
        }
    },

    // Get single office by ID
    async getOffice(id: string): Promise<Office> {
        try {
            return await privateApi.get(`v0/ipam/office/${id}`).json<Office>();
        } catch (error) {
            console.error('Failed to get office:', error);
            throw error;
        }
    },

    // Update office
    async updateOffice(id: string, data: UpdateOfficeRequest): Promise<Office> {
        try {
            return await privateApi.put(`v0/ipam/office/${id}`, { json: data }).json<Office>();
        } catch (error) {
            console.error('Failed to update office:', error);
            throw error;
        }
    },

    // Delete office
    async deleteOffice(id: string): Promise<void> {
        try {
            await privateApi.delete(`v0/ipam/office/${id}`);
        } catch (error) {
            console.error('Failed to delete office:', error);
            throw error;
        }
    }
};

// Server Room types
export interface ServerRoom {
    id: string;
    office_id: string;
    name: string;
    description?: string;
    floor_level?: string;
    room_number?: string;
    temperature_monitoring: boolean;
    humidity_monitoring: boolean;
    access_control: boolean;
    created_by: string;
    created_at: string;
    updated_at: string;
    is_active: boolean;
}

export interface CreateServerRoomRequest {
    name: string;
    description?: string;
    floor_level?: string;
    room_number?: string;
    temperature_monitoring: boolean;
    humidity_monitoring: boolean;
    access_control: boolean;
}

export interface UpdateServerRoomRequest {
    name?: string;
    description?: string;
    floor_level?: string;
    room_number?: string;
    temperature_monitoring?: boolean;
    humidity_monitoring?: boolean;
    access_control?: boolean;
}

export interface ServerRoomListResponse {
    server_rooms: ServerRoom[];
    total: number;
    page: number;
    limit: number;
}

export interface ServerRoomListParams {
    page?: number;
    limit?: number;
}

// Server Room management API calls
export const serverRoomApi = {
    // Create new server room
    async createServerRoom(officeId: string, data: CreateServerRoomRequest): Promise<ServerRoom> {
        try {
            console.log('Creating server room:', { officeId, data });
            return await privateApi.post(`v0/ipam/office/${officeId}/server-room`, { json: data }).json<ServerRoom>();
        } catch (error) {
            console.error('Failed to create server room:', error);
            console.error('Request details:', { officeId, data });
            throw error;
        }
    },

    // Get list of server rooms for an office
    async getServerRooms(officeId: string, params?: ServerRoomListParams): Promise<ServerRoomListResponse> {
        try {
            const searchParams = new URLSearchParams();
            if (params?.page) searchParams.append('page', params.page.toString());
            if (params?.limit) searchParams.append('limit', params.limit.toString());

            const url = `v0/ipam/office/${officeId}/server-room${searchParams.toString() ? `?${searchParams.toString()}` : ''}`;
            return await privateApi.get(url).json<ServerRoomListResponse>();
        } catch (error) {
            console.error('Failed to get server rooms:', error);
            throw error;
        }
    },

    // Get single server room by ID
    async getServerRoom(officeId: string, serverRoomId: string): Promise<ServerRoom> {
        try {
            return await privateApi.get(`v0/ipam/office/${officeId}/server-room/${serverRoomId}`).json<ServerRoom>();
        } catch (error) {
            console.error('Failed to get server room:', error);
            throw error;
        }
    },

    // Update server room
    async updateServerRoom(officeId: string, serverRoomId: string, data: UpdateServerRoomRequest): Promise<ServerRoom> {
        try {
            return await privateApi.put(`v0/ipam/office/${officeId}/server-room/${serverRoomId}`, { json: data }).json<ServerRoom>();
        } catch (error) {
            console.error('Failed to update server room:', error);
            throw error;
        }
    },

    // Delete server room
    async deleteServerRoom(officeId: string, serverRoomId: string): Promise<void> {
        try {
            await privateApi.delete(`v0/ipam/office/${officeId}/server-room/${serverRoomId}`);
        } catch (error) {
            console.error('Failed to delete server room:', error);
            throw error;
        }
    }
};