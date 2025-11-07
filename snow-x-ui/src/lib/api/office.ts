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
			return await privateApi
				.put(`v0/ipam/office/${officeId}/server-room/${serverRoomId}`, { json: data })
				.json<ServerRoom>();
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

// IP Range types
export interface IpRange {
	id: string;
	tenant_id: string;
	name: string;
	description?: string;
	network_address: string;
	subnet_mask: number;
	gateway?: string;
	dns_servers?: string[];
	vlan_id?: number;
	ip_version: number;
	total_ips?: number;
	used_ips?: number;
	available_ips?: number;
	usage_percentage?: number;
	allocated_ips?: number;
	reserved_ips?: number;
	unavailable_ips?: number;
	expired_ips?: number;
	other_ips?: number;
	created_by: string;
	created_at: string;
	updated_at: string;
	is_active: boolean;
}

export interface CreateIpRangeRequest {
	tenant_id: string;
	name: string;
	description?: string;
	network_address: string;
	subnet_mask: number;
	gateway?: string;
	dns_servers?: string[];
	vlan_id?: number;
	ip_version?: number;
}

export interface UpdateIpRangeRequest {
	tenant_id?: string;
	name?: string;
	description?: string;
	network_address?: string;
	subnet_mask?: number;
	gateway?: string;
	dns_servers?: string[];
	vlan_id?: number;
	ip_version?: number;
}

export interface IpRangeListResponse {
	ip_ranges: IpRange[];
	total: number;
	page: number;
	limit: number;
}

export interface IpRangeListParams {
	page?: number;
	limit?: number;
}

// IP Range management API calls
export const ipRangeApi = {
	// Create new IP range
	async createIpRange(data: CreateIpRangeRequest): Promise<IpRange> {
		try {
			return await privateApi.post('v0/ipam/ip-range', { json: data }).json<IpRange>();
		} catch (error) {
			console.error('Failed to create IP range:', error);
			throw error;
		}
	},

	// Get list of IP ranges
	async getIpRanges(params?: IpRangeListParams): Promise<IpRangeListResponse> {
		try {
			const searchParams = new URLSearchParams();
			if (params?.page) searchParams.append('page', params.page.toString());
			if (params?.limit) searchParams.append('limit', params.limit.toString());

			const url = `v0/ipam/ip-range${searchParams.toString() ? `?${searchParams.toString()}` : ''}`;
			return await privateApi.get(url).json<IpRangeListResponse>();
		} catch (error) {
			console.error('Failed to get IP ranges:', error);
			throw error;
		}
	},

	// Get single IP range by ID
	async getIpRange(ipRangeId: string): Promise<IpRange> {
		try {
			return await privateApi.get(`v0/ipam/ip-range/${ipRangeId}`).json<IpRange>();
		} catch (error) {
			console.error('Failed to get IP range:', error);
			throw error;
		}
	},

	// Update IP range
	async updateIpRange(ipRangeId: string, data: UpdateIpRangeRequest): Promise<IpRange> {
		try {
			return await privateApi.put(`v0/ipam/ip-range/${ipRangeId}`, { json: data }).json<IpRange>();
		} catch (error) {
			console.error('Failed to update IP range:', error);
			throw error;
		}
	},

	// Delete IP range
	async deleteIpRange(ipRangeId: string): Promise<void> {
		try {
			await privateApi.delete(`v0/ipam/ip-range/${ipRangeId}`);
		} catch (error) {
			console.error('Failed to delete IP range:', error);
			throw error;
		}
	}
};

// IP Address types
export interface IpAddress {
	id: string;
	ip_range_id: string;
	ip_address?: string;
	mac_address?: string;
	hostname?: string;
	status: string;
	description?: string;
	created_by: string;
	created_at: string;
	updated_at: string;
	is_active: boolean;
}

export interface CreateBulkIpAddressesRequest {
	ip_range_id: string;
	start_ip: string;
	end_ip: string;
	status: string;
	description?: string;
}

export interface IpAddressListResponse {
	ip_addresses: IpAddress[];
	total: number;
	page: number;
	limit: number;
}

export interface IpAddressListParams {
	ip_range_id?: string;
	status?: string;
	search?: string;
	page?: number;
	limit?: number;
}

// Device contact mapping types
export interface DeviceContact {
	id: string;
	contact_id: string;
	contact_name: string;
	resource_type: string;
	resource_id: string;
	role?: string;
	created_at: string;
}

export interface DeviceContactListResponse {
	mappings: DeviceContact[];
	total: number;
}

// IP Address management API calls
export const ipAddressApi = {
	// Create bulk IP addresses
	async createBulkIpAddresses(data: CreateBulkIpAddressesRequest): Promise<IpAddress[]> {
		try {
			return await privateApi.post('v0/ipam/ip-address/bulk', { json: data }).json<IpAddress[]>();
		} catch (error) {
			console.error('Failed to create bulk IP addresses:', error);
			throw error;
		}
	},

	// Get list of IP addresses
	async getIpAddresses(params?: IpAddressListParams): Promise<IpAddressListResponse> {
		try {
			const searchParams = new URLSearchParams();
			if (params?.ip_range_id) searchParams.append('ip_range_id', params.ip_range_id);
			if (params?.status) searchParams.append('status', params.status);
			if (params?.search) searchParams.append('search', params.search);
			if (params?.page) searchParams.append('page', params.page.toString());
			if (params?.limit) searchParams.append('limit', params.limit.toString());

			const url = `v0/ipam/ip-address${searchParams.toString() ? `?${searchParams.toString()}` : ''}`;
			return await privateApi.get(url).json<IpAddressListResponse>();
		} catch (error) {
			console.error('Failed to get IP addresses:', error);
			throw error;
		}
	}
};

// Device types
export interface Device {
	id: string;
	rack_id?: string;
	name: string;
	description?: string;
	device_type: string;
	manufacturer?: string;
	model?: string;
	serial_number?: string;
	rack_position?: number;
	rack_size: number;
	power_consumption?: number;
	status: string;
	purchase_date?: string;
	warranty_end?: string;
	created_by: string;
	created_at: string;
	updated_at: string;
	is_active: boolean;
	source_type: string; // 'manual' or 'api_sync'
	external_api_connection_id?: number;
}

export interface CreateDeviceRequest {
	rack_id?: string;
	name: string;
	description?: string;
	device_type: string;
	manufacturer?: string;
	model?: string;
	serial_number?: string;
	rack_position?: number;
	rack_size?: number;
	power_consumption?: number;
	status?: string;
	purchase_date?: string;
	warranty_end?: string;
}

export interface UpdateDeviceRequest {
	rack_id?: string;
	name?: string;
	description?: string;
	device_type?: string;
	manufacturer?: string;
	model?: string;
	serial_number?: string;
	rack_position?: number;
	rack_size?: number;
	power_consumption?: number;
	status?: string;
	purchase_date?: string;
	warranty_end?: string;
}

export interface DeviceListResponse {
	devices: Device[];
	total: number;
	page: number;
	limit: number;
}

export interface DeviceListParams {
	page?: number;
	limit?: number;
	search?: string;
	device_type?: string;
	status?: string;
	rack_id?: string;
}

// Device management API calls
export const deviceApi = {
	// Create new device
	async createDevice(data: CreateDeviceRequest): Promise<Device> {
		try {
			return await privateApi.post('v0/ipam/device', { json: data }).json<Device>();
		} catch (error) {
			console.error('Failed to create device:', error);
			throw error;
		}
	},

	// Get list of devices
	async getDevices(params?: DeviceListParams): Promise<DeviceListResponse> {
		try {
			const searchParams = new URLSearchParams();
			if (params?.page) searchParams.append('page', params.page.toString());
			if (params?.limit) searchParams.append('limit', params.limit.toString());
			if (params?.search) searchParams.append('search', params.search);
			if (params?.device_type) searchParams.append('device_type', params.device_type);
			if (params?.status) searchParams.append('status', params.status);
			if (params?.rack_id) searchParams.append('rack_id', params.rack_id);

			const url = `v0/ipam/device${searchParams.toString() ? `?${searchParams.toString()}` : ''}`;
			return await privateApi.get(url).json<DeviceListResponse>();
		} catch (error) {
			console.error('Failed to get devices:', error);
			throw error;
		}
	},

	// Get single device by ID
	async getDevice(deviceId: string): Promise<Device> {
		try {
			return await privateApi.get(`v0/ipam/device/${deviceId}`).json<Device>();
		} catch (error) {
			console.error('Failed to get device:', error);
			throw error;
		}
	},

	// Update device
	async updateDevice(deviceId: string, data: UpdateDeviceRequest): Promise<Device> {
		try {
			return await privateApi.put(`v0/ipam/device/${deviceId}`, { json: data }).json<Device>();
		} catch (error) {
			console.error('Failed to update device:', error);
			throw error;
		}
	},

	// Delete device
	async deleteDevice(deviceId: string): Promise<void> {
		try {
			await privateApi.delete(`v0/ipam/device/${deviceId}`);
		} catch (error) {
			console.error('Failed to delete device:', error);
			throw error;
		}
	},

	// Get assigned IP addresses for a device
	async getAssignedIpAddresses(deviceId: string): Promise<IpAddress[]> {
		try {
			return await privateApi.get(`v0/ipam/device/${deviceId}/ip-addresses`).json<IpAddress[]>();
		} catch (error) {
			console.error('Failed to get assigned IP addresses:', error);
			throw error;
		}
	},

	// Get assigned contacts for a device
	async getAssignedContacts(deviceId: string): Promise<DeviceContactListResponse> {
		try {
			return await privateApi.get(`v0/ipam/device/${deviceId}/contacts`).json<DeviceContactListResponse>();
		} catch (error) {
			console.error('Failed to get assigned contacts:', error);
			throw error;
		}
	},

	// Assign IP address to a device
	async assignIpAddress(deviceId: string, ipAddressId: string): Promise<void> {
		try {
			await privateApi.post(`v0/ipam/device/${deviceId}/ip-address`, {
				json: { ip_address_id: ipAddressId }
			});
		} catch (error) {
			console.error('Failed to assign IP address:', error);
			throw error;
		}
	},

	// Assign contact to a device
	async assignContact(deviceId: string, contactId: string, role?: string): Promise<void> {
		try {
			await privateApi.post(`v0/ipam/device/${deviceId}/contact`, {
				json: { contact_id: contactId, role }
			});
		} catch (error) {
			console.error('Failed to assign contact:', error);
			throw error;
		}
	},

	// Unassign IP address from a device
	async unassignIpAddress(deviceId: string, ipAddressId: string): Promise<void> {
		try {
			await privateApi.delete(`v0/ipam/device/${deviceId}/ip-address/${ipAddressId}`);
		} catch (error) {
			console.error('Failed to unassign IP address:', error);
			throw error;
		}
	},

	// Unassign contact from a device
	async unassignContact(deviceId: string, contactId: string): Promise<void> {
		try {
			await privateApi.delete(`v0/ipam/device/${deviceId}/contact/${contactId}`);
		} catch (error) {
			console.error('Failed to unassign contact:', error);
			throw error;
		}
	}
};

// Rack types
export interface RackDeviceSummary {
	id: string;
	name: string;
	device_type?: string;
	status?: string;
	manufacturer?: string;
	model?: string;
	serial_number?: string;
	rack_position?: number | null;
	rack_size: number;
}

export interface Rack {
	id: string;
	server_room_id: string;
	name: string;
	description?: string;
	rack_height: number;
	power_capacity?: number;
	cooling_type?: string;
	location_x?: number;
	location_y?: number;
	created_at: string;
	updated_at: string;
	is_active: boolean;
	server_room_name?: string;
	office_name?: string;
	device_count?: number;
	usage_percentage?: number;
	used_units?: number;
	devices?: RackDeviceSummary[];
}

export interface RackListResponse {
	racks: Rack[];
	total: number;
	page: number;
	limit: number;
}

export interface RackListParams {
	page?: number;
	limit?: number;
}

// Rack management API calls
export interface CreateRackRequest {
	server_room_id: string;
	name: string;
	description?: string;
	rack_height: number;
	power_capacity?: number;
	cooling_type?: string;
	location_x?: number;
	location_y?: number;
}

export interface UpdateRackRequest {
	name?: string;
	description?: string;
	rack_height?: number;
	power_capacity?: number;
	cooling_type?: string;
	location_x?: number;
	location_y?: number;
}

export const rackApi = {
	// Get list of racks
	async getRacks(params?: RackListParams): Promise<RackListResponse> {
		try {
			const searchParams = new URLSearchParams();
			if (params?.page) searchParams.append('page', params.page.toString());
			if (params?.limit) searchParams.append('limit', params.limit.toString());

			const url = `v0/ipam/racks${searchParams.toString() ? `?${searchParams.toString()}` : ''}`;
			return await privateApi.get(url).json<RackListResponse>();
		} catch (error) {
			console.error('Failed to get racks:', error);
			throw error;
		}
	},

	async createRack(data: CreateRackRequest): Promise<Rack> {
		try {
			return await privateApi.post('v0/ipam/racks', { json: data }).json<Rack>();
		} catch (error) {
			console.error('Failed to create rack:', error);
			throw error;
		}
	},

	async updateRack(id: string, data: UpdateRackRequest): Promise<Rack> {
		try {
			return await privateApi.put(`v0/ipam/racks/${id}`, { json: data }).json<Rack>();
		} catch (error) {
			console.error('Failed to update rack:', error);
			throw error;
		}
	}
};

// Device Library types
export interface DeviceLibrary {
	id: string;
	name: string;
	description?: string;
	device_type: string;
	manufacturer?: string;
	model?: string;
	default_rack_size?: number;
	default_power_consumption?: number;
	default_config?: any;
	device_id?: string;
	device_name?: string;
	created_by: string;
	created_at: string;
	updated_at: string;
	is_active: boolean;
	source_type: string; // 'manual' or 'api_sync'
	external_api_connection_id?: number;
}

export interface CreateDeviceLibraryRequest {
	name: string;
	description?: string;
	device_type: string;
	manufacturer?: string;
	model?: string;
	default_rack_size?: number;
	default_power_consumption?: number;
	default_config?: any;
	device_id?: string | null;
	device_name?: string | null;
}

export interface UpdateDeviceLibraryRequest {
	name?: string;
	description?: string;
	device_type?: string;
	manufacturer?: string;
	model?: string;
	default_rack_size?: number;
	default_power_consumption?: number;
	default_config?: any;
	device_id?: string | null;
	device_name?: string | null;
	is_active?: boolean;
	remove_device_link?: boolean;
}

export interface DeviceLibraryListResponse {
	libraries: DeviceLibrary[];
	total: number;
	page: number;
	limit: number;
}

export interface DeviceLibraryListParams {
	page?: number;
	limit?: number;
	search?: string;
	device_type?: string;
}

// Device Library management API calls
export const deviceLibraryApi = {
	// Create new device library
	async createDeviceLibrary(data: CreateDeviceLibraryRequest): Promise<DeviceLibrary> {
		try {
			return await privateApi.post('v0/ipam/device-library', { json: data }).json<DeviceLibrary>();
		} catch (error) {
			console.error('Failed to create device library:', error);
			throw error;
		}
	},

	// Get list of device libraries
	async getDeviceLibraries(params?: DeviceLibraryListParams): Promise<DeviceLibraryListResponse> {
		try {
			const searchParams = new URLSearchParams();
			if (params?.page) searchParams.append('page', params.page.toString());
			if (params?.limit) searchParams.append('limit', params.limit.toString());
			if (params?.search) searchParams.append('search', params.search);
			if (params?.device_type) searchParams.append('device_type', params.device_type);

			const url = `v0/ipam/device-library${searchParams.toString() ? `?${searchParams.toString()}` : ''}`;
			return await privateApi.get(url).json<DeviceLibraryListResponse>();
		} catch (error) {
			console.error('Failed to get device libraries:', error);
			throw error;
		}
	},

	// Get single device library by ID
	async getDeviceLibrary(libraryId: string): Promise<DeviceLibrary> {
		try {
			return await privateApi.get(`v0/ipam/device-library/${libraryId}`).json<DeviceLibrary>();
		} catch (error) {
			console.error('Failed to get device library:', error);
			throw error;
		}
	},

	// Update device library
	async updateDeviceLibrary(libraryId: string, data: UpdateDeviceLibraryRequest): Promise<DeviceLibrary> {
		try {
			return await privateApi.put(`v0/ipam/device-library/${libraryId}`, { json: data }).json<DeviceLibrary>();
		} catch (error) {
			console.error('Failed to update device library:', error);
			throw error;
		}
	},

	// Delete device library
	async deleteDeviceLibrary(libraryId: string): Promise<void> {
		try {
			await privateApi.delete(`v0/ipam/device-library/${libraryId}`);
		} catch (error) {
			console.error('Failed to delete device library:', error);
			throw error;
		}
	}
};

// Contact types
export interface Contact {
	id: string;
	name: string;
	title?: string;
	department?: string;
	phone?: string;
	mobile?: string;
	email?: string;
	office_location?: string;
	responsibilities?: string;
	created_by: string;
	created_at: string;
	updated_at: string;
	is_active: boolean;
	source_type: string; // 'manual' or 'api_sync'
	external_api_connection_id?: number;
}

export interface CreateContactRequest {
	name: string;
	title?: string;
	department?: string;
	phone?: string;
	mobile?: string;
	email?: string;
	office_location?: string;
	responsibilities?: string;
}

export interface UpdateContactRequest {
	name?: string;
	title?: string;
	department?: string;
	phone?: string;
	mobile?: string;
	email?: string;
	office_location?: string;
	responsibilities?: string;
	is_active?: boolean;
}

export interface ContactListResponse {
	contacts: Contact[];
	total: number;
	page: number;
	limit: number;
}

export interface ContactListParams {
	page?: number;
	limit?: number;
	search?: string;
	department?: string;
	is_active?: boolean;
}

// Contact management API calls
export const contactApi = {
	// Create new contact
	async createContact(data: CreateContactRequest): Promise<Contact> {
		try {
			return await privateApi.post('v0/ipam/contact', { json: data }).json<Contact>();
		} catch (error) {
			console.error('Failed to create contact:', error);
			throw error;
		}
	},

	// Get list of contacts
	async getContacts(params?: ContactListParams): Promise<ContactListResponse> {
		try {
			const searchParams = new URLSearchParams();
			if (params?.page) searchParams.append('page', params.page.toString());
			if (params?.limit) searchParams.append('limit', params.limit.toString());
			if (params?.search) searchParams.append('search', params.search);
			if (params?.department) searchParams.append('department', params.department);
			if (params?.is_active !== undefined) searchParams.append('is_active', params.is_active.toString());

			const url = `v0/ipam/contact${searchParams.toString() ? `?${searchParams.toString()}` : ''}`;
			return await privateApi.get(url).json<ContactListResponse>();
		} catch (error) {
			console.error('Failed to get contacts:', error);
			throw error;
		}
	},

	// Get single contact by ID
	async getContact(contactId: string): Promise<Contact> {
		try {
			return await privateApi.get(`v0/ipam/contact/${contactId}`).json<Contact>();
		} catch (error) {
			console.error('Failed to get contact:', error);
			throw error;
		}
	},

	// Update contact
	async updateContact(contactId: string, data: UpdateContactRequest): Promise<Contact> {
		try {
			return await privateApi.put(`v0/ipam/contact/${contactId}`, { json: data }).json<Contact>();
		} catch (error) {
			console.error('Failed to update contact:', error);
			throw error;
		}
	},

	// Delete contact
	async deleteContact(contactId: string): Promise<void> {
		try {
			await privateApi.delete(`v0/ipam/contact/${contactId}`);
		} catch (error) {
			console.error('Failed to delete contact:', error);
			throw error;
		}
	}
};
