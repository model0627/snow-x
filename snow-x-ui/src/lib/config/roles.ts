// Role-based access control configuration

export type UserRole = 'Admin' | 'Manager' | 'Staff';

export interface RolePermissions {
	role: UserRole;
	allowedMenus: string[];
}

// Role definitions with default permissions
export const ROLE_PERMISSIONS: Record<UserRole, RolePermissions> = {
	Admin: {
		role: 'Admin',
		allowedMenus: ['*'] // Admin has access to all menus
	},
	Manager: {
		role: 'Manager',
		allowedMenus: [
			'dashboard',
			'ipam',
			'settings'
		]
	},
	Staff: {
		role: 'Staff',
		allowedMenus: ['dashboard']
	}
};

// Menu item identifiers (matching the sidebar structure)
export const MENU_IDS = {
	DASHBOARD: 'dashboard',
	IPAM: 'ipam',
	IPAM_IP: 'ipam.ip',
	IPAM_DEVICE: 'ipam.device',
	IPAM_DEVICE_LIBRARY: 'ipam.device_library',
	CUSTODIAN: 'custodian',
	CUSTODIAN_POLICIES: 'custodian.policies',
	CUSTODIAN_REPORTS: 'custodian.reports',
	ADMIN_SETTINGS: 'admin.settings',
	SETTINGS: 'settings',
	RELATIONSHIPS: 'relationships',
	CONTACTS: 'contacts'
} as const;

/**
 * Check if a user with given role has access to a specific menu
 */
export function hasMenuAccess(userRole: UserRole, menuId: string): boolean {
	const permissions = ROLE_PERMISSIONS[userRole];

	// Admin has access to everything
	if (permissions.allowedMenus.includes('*')) {
		return true;
	}

	// Check for exact match
	if (permissions.allowedMenus.includes(menuId)) {
		return true;
	}

	// Check for parent menu access (e.g., 'ipam' grants access to 'ipam.ip')
	const parentMenuId = menuId.split('.')[0];
	if (permissions.allowedMenus.includes(parentMenuId)) {
		return true;
	}

	return false;
}

/**
 * Get all accessible menu IDs for a role
 */
export function getAccessibleMenus(userRole: UserRole): string[] {
	const permissions = ROLE_PERMISSIONS[userRole];

	if (permissions.allowedMenus.includes('*')) {
		return Object.values(MENU_IDS);
	}

	return permissions.allowedMenus;
}

/**
 * Role display names (Korean)
 */
export const ROLE_DISPLAY_NAMES: Record<UserRole, string> = {
	Admin: '관리자',
	Manager: '담당자',
	Staff: '직원'
};
