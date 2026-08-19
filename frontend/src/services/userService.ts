import { apiRequest } from "./apiClient";

export interface UserProfile {
  id: number;
  username: string;
  email: string;
  rating: number;
  created_at: string;
}

export interface UserStats extends UserProfile {
  total_solved: number;
  easy_solved: number;
  medium_solved: number;
  hard_solved: number;
  total_submissions: number;
  accepted_submissions: number;
}

export const UserService = {
  getMe: () => apiRequest<UserProfile>("/user"),

  getUserInfo: (params: { id?: number; username?: string; email?: string }) =>
    apiRequest<UserProfile>("/user/info", { params }),

  getUserStats: (params?: { user_id?: number; username?: string }) =>
    apiRequest<UserStats>("/user/stats", { params }),
};
