import { privateApi } from '../private';

export interface CustodianPolicy {
	id: string;
	name: string;
	description?: string;
	content: string; // YAML content
	created_at: string;
	updated_at: string;
}

export interface CreatePolicyRequest {
	name: string;
	description?: string;
	content: string;
}

export interface UpdatePolicyRequest {
	name?: string;
	description?: string;
	content?: string;
}

export interface ExecutePolicyRequest {
	policy_id: string;
	dry_run?: boolean;
}

export interface ExecutionResult {
	execution_id: string;
	policy_id: string;
	status: 'running' | 'completed' | 'failed';
	started_at: string;
	completed_at?: string;
	output?: string;
	error?: string;
}

/**
 * Get all custodian policies
 */
export async function getCustodianPolicies(): Promise<CustodianPolicy[]> {
	const response = await privateApi.get('v0/custodian/policies').json<CustodianPolicy[]>();
	return response;
}

/**
 * Get a single custodian policy by ID
 */
export async function getCustodianPolicy(id: string): Promise<CustodianPolicy> {
	const response = await privateApi.get(`v0/custodian/policies/${id}`).json<CustodianPolicy>();
	return response;
}

/**
 * Create a new custodian policy
 */
export async function createCustodianPolicy(data: CreatePolicyRequest): Promise<CustodianPolicy> {
	const response = await privateApi.post('v0/custodian/policies', { json: data }).json<CustodianPolicy>();
	return response;
}

/**
 * Update an existing custodian policy
 */
export async function updateCustodianPolicy(id: string, data: UpdatePolicyRequest): Promise<CustodianPolicy> {
	const response = await privateApi.put(`v0/custodian/policies/${id}`, { json: data }).json<CustodianPolicy>();
	return response;
}

/**
 * Delete a custodian policy
 */
export async function deleteCustodianPolicy(id: string): Promise<void> {
	await privateApi.delete(`v0/custodian/policies/${id}`);
}

/**
 * Execute a custodian policy
 */
export async function executeCustodianPolicy(data: ExecutePolicyRequest): Promise<ExecutionResult> {
	const response = await privateApi.post('v0/custodian/execute', { json: data }).json<ExecutionResult>();
	return response;
}

/**
 * Get execution result
 */
export async function getExecutionResult(executionId: string): Promise<ExecutionResult> {
	const response = await privateApi.get(`v0/custodian/executions/${executionId}`).json<ExecutionResult>();
	return response;
}

/**
 * Validate YAML syntax
 */
export async function validateYaml(content: string): Promise<{ valid: boolean; error?: string }> {
	const response = await privateApi.post('v0/custodian/validate', { json: { content } }).json<{ valid: boolean; error?: string }>();
	return response;
}
