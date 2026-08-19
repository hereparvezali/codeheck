import { apiRequest } from "./apiClient";

export interface ContestItem {
  id: number;
  title: string;
  slug: string;
  description?: string;
  start_time: string;
  end_time: string;
  is_public: boolean;
  author_id?: number;
  registration_id?: number;
  registered_at?: string;
}

export interface ContestsResponse {
  cursor?: number;
  contests: ContestItem[];
}

export interface ContestProblem {
  id: number;
  title: string;
  slug: string;
  difficulty?: string;
  label?: string;
}

export interface LeaderboardProblemStatus {
  solved: boolean;
  attempts: number;
  time?: number;
}

export interface LeaderboardEntry {
  user_id: number;
  username: string;
  solved: number;
  penalty: number;
  problems: Record<string, LeaderboardProblemStatus>;
}

export interface LeaderboardResponse {
  standings: LeaderboardEntry[];
}

export interface CreateContestPayload {
  title: string;
  slug: string;
  description?: string;
  start_time: string;
  end_time: string;
  is_public: boolean;
}

export const ContestService = {
  getContests: (params?: {
    cursor?: number;
    limit?: number;
    offset?: number;
    author_id?: number;
  }) => apiRequest<ContestsResponse>("/contests", { params }),

  getContest: (params: { id?: number; slug?: string }) =>
    apiRequest<ContestItem>("/contest", { params }),

  createContest: (payload: CreateContestPayload) =>
    apiRequest<ContestItem>("/contest", {
      method: "POST",
      body: JSON.stringify(payload),
    }),

  updateContest: (payload: Partial<CreateContestPayload> & { id: number }) =>
    apiRequest<{ status: string; rows_affected: number }>("/contest", {
      method: "PUT",
      body: JSON.stringify(payload),
    }),

  deleteContest: (id: number) =>
    apiRequest<{ status: string; rows_affected: number }>("/contest", {
      method: "DELETE",
      params: { id },
    }),

  getContestProblems: (contest_id: number) =>
    apiRequest<ContestProblem[]>("/contest/problems", {
      params: { id: contest_id },
    }),

  addContestProblems: (contest_id: number, problems: { problem_id: number; label?: string }[]) =>
    apiRequest<{ status: string; count: number }>("/contest/problems", {
      method: "POST",
      body: JSON.stringify({ id: contest_id, problems }),
    }),

  deleteContestProblem: (contest_id: number, problem_id: number) =>
    apiRequest<{ status: string; rows_affected: number }>("/contest/problems", {
      method: "DELETE",
      params: { contest_id, problem_id },
    }),

  register: (contest_id: number) =>
    apiRequest<{ id: number; contest_id: number; registered_at: string }>("/contest/registration", {
      method: "POST",
      body: JSON.stringify({ contest_id }),
    }),

  unregister: (registration_id: number) =>
    apiRequest<{ status: string; rows_affected: number }>("/contest/registration", {
      method: "DELETE",
      params: { registration_id },
    }),

  getLeaderboard: (contest_id: number) =>
    apiRequest<LeaderboardResponse>("/contest/leaderboard", {
      params: { contest_id },
    }),
};
