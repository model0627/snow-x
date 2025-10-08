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

export interface CustodianExecution {
	id: string;
	policy_id: string;
	status: 'pending' | 'running' | 'completed' | 'failed';
	dry_run: boolean;
	task_id?: string;
	output?: string;
	error?: string;
	created_at: string;
	started_at?: string;
	completed_at?: string;
}

export interface TaskStatusResponse {
	task_id: string;
	status: string;
	result?: {
		status: string;
		output: string;
		return_code: number;
	};
	error?: string;
}

export interface ExecutionResult {
	id: string;
	policy_id: string;
	status: 'pending' | 'running' | 'completed' | 'failed';
	dry_run: boolean;
	output?: string;
	error?: string;
	created_at: string;
	started_at?: string;
	completed_at?: string;
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
 * Get execution result by ID
 */
export async function getExecutionResult(executionId: string): Promise<CustodianExecution> {
	const response = await privateApi.get(`v0/custodian/executions/${executionId}`).json<CustodianExecution>();
	return response;
}

/**
 * Get all executions for a policy
 */
export async function getPolicyExecutions(policyId: string): Promise<CustodianExecution[]> {
	const response = await privateApi.get(`v0/custodian/policies/${policyId}/executions`).json<CustodianExecution[]>();
	return response;
}

/**
 * Get task status from Task API
 */
export async function getTaskStatus(taskId: string): Promise<TaskStatusResponse> {
	// Task API는 별도 포트(7000)에서 실행됨
	const taskApiUrl = import.meta.env.PUBLIC_TASK_API_URL || 'http://localhost:7000';
	const response = await fetch(`${taskApiUrl}/tasks/custodian/task-status/${taskId}`);
	if (!response.ok) {
		throw new Error(`Failed to get task status: ${response.statusText}`);
	}
	return response.json();
}

/**
 * Validate YAML syntax
 */
export async function validateYaml(content: string): Promise<{ valid: boolean; error?: string }> {
	const response = await privateApi.post('v0/custodian/validate', { json: { content } }).json<{ valid: boolean; error?: string }>();
	return response;
}
