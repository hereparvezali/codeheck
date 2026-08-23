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

export interface MessageResponse {
  message: string;
}

export const UserService = {
  getMe: () => apiRequest<UserProfile>("/user"),

  getUserInfo: (params: { id?: number; username?: string; email?: string }) =>
    apiRequest<UserProfile>("/user/info", { params }),

  getUserStats: (params?: { user_id?: number; username?: string }) =>
    apiRequest<UserStats>("/user/stats", { params }),

  verifyEmail: (token: string) =>
    apiRequest<MessageResponse>("/user/verify", {
      method: "POST",
      body: JSON.stringify({ token }),
    }),

  resendVerification: (email: string) =>
    apiRequest<MessageResponse>("/user/resend-verification", {
      method: "POST",
      body: JSON.stringify({ email }),
    }),
};
