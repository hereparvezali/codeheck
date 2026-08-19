import { apiRequest } from "./apiClient";

export interface ProblemItem {
  id: number;
  slug: string;
  title: string;
  difficulty?: string;
  is_public: boolean;
  created_at: string;
  author_id?: number;
  status?: string | null;
}

export interface ProblemDetail extends ProblemItem {
  statement?: string;
  input_spec?: string;
  output_spec?: string;
  sample_inputs?: string;
  sample_outputs?: string;
  time_limit: number;
  memory_limit: number;
}

export interface ProblemsResponse {
  cursor?: number;
  problems: ProblemItem[];
}

export interface CreateProblemPayload {
  title: string;
  slug: string;
  statement?: string;
  input_spec?: string;
  output_spec?: string;
  sample_inputs?: string;
  sample_outputs?: string;
  time_limit: number;
  memory_limit: number;
  difficulty?: string;
  is_public?: boolean;
}

export interface TestCase {
  input: string;
  output: string;
}

export const ProblemService = {
  getProblems: (params?: {
    cursor?: number;
    limit?: number;
    offset?: number;
    difficulty?: string;
    search?: string;
    author_id?: number;
  }) => apiRequest<ProblemsResponse>("/problems", { params }),

  getProblem: (params: { id?: number; slug?: string }) =>
    apiRequest<ProblemDetail>("/problem", { params }),

  createProblem: (payload: CreateProblemPayload) =>
    apiRequest<ProblemDetail>("/problem", {
      method: "POST",
      body: JSON.stringify(payload),
    }),

  updateProblem: (payload: Partial<CreateProblemPayload> & { id: number }) =>
    apiRequest<{ status: string; rows_affected: number }>("/problem", {
      method: "PUT",
      body: JSON.stringify(payload),
    }),

  deleteProblem: (id: number) =>
    apiRequest<{ status: string; rows_affected: number }>("/problem", {
      method: "DELETE",
      params: { id },
    }),

  getTestCases: (problem_id: number) =>
    apiRequest<TestCase[]>("/problem/testcases", {
      params: { problem_id },
    }),

  createTestCases: (problem_id: number, cases: TestCase[]) =>
    apiRequest<{ status: number; msg: string; count: number }>("/problem/testcases", {
      method: "POST",
      body: JSON.stringify({ problem_id, cases }),
    }),

  deleteTestCases: (problem_id: number) =>
    apiRequest<{ status: string; rows_affected: number }>("/problem/testcases", {
      method: "DELETE",
      params: { problem_id },
    }),
};
