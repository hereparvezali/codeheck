import { apiRequest } from "./apiClient";

export interface SubmissionItem {
  id: number;
  user_id: number;
  problem_id: number;
  language: string;
  code?: string;
  status: string;
  verdict?: string | null;
  time?: number | null;
  memory?: number | null;
  submitted_at: string;
  contest_id?: number | null;
}

export interface SubmissionsResponse {
  cursor?: number;
  submissions: SubmissionItem[];
}

export interface CreateSubmissionPayload {
  problem_id: number;
  language: string;
  code: string;
  contest_id?: number | null;
}

export const SubmissionService = {
  createSubmission: (payload: CreateSubmissionPayload) =>
    apiRequest<SubmissionItem>("/submission", {
      method: "POST",
      body: JSON.stringify(payload),
    }),

  getSubmission: (id: number) =>
    apiRequest<SubmissionItem>("/submission", {
      params: { id },
    }),

  getSubmissions: (params?: {
    cursor?: number;
    limit?: number;
    offset?: number;
    id?: number;
    user_id?: number;
    problem_id?: number;
    contest_id?: number;
    status?: string;
    language?: string;
  }) => apiRequest<SubmissionsResponse>("/submissions", { params }),
};
