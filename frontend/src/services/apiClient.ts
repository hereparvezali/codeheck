import { getToken } from "../utils/storage";

const BASE_URL = import.meta.env.VITE_BASE || "/api";

export interface RequestOptions extends RequestInit {
  params?: Record<string, string | number | boolean | undefined | null>;
}

export class ApiError extends Error {
  status: number;
  data?: any;

  constructor(status: number, message: string, data?: any) {
    super(message);
    this.name = "ApiError";
    this.status = status;
    this.data = data;
  }
}

export async function apiRequest<T>(endpoint: string, options: RequestOptions = {}): Promise<T> {
  const { params, headers = {}, ...customConfig } = options;

  let url = `${BASE_URL}${endpoint.startsWith("/") ? endpoint : `/${endpoint}`}`;

  if (params) {
    const queryParams = new URLSearchParams();
    Object.entries(params).forEach(([key, value]) => {
      if (value !== undefined && value !== null) {
        queryParams.append(key, String(value));
      }
    });
    const queryString = queryParams.toString();
    if (queryString) {
      url += `${url.includes("?") ? "&" : "?"}${queryString}`;
    }
  }

  const token = getToken();
  const defaultHeaders: Record<string, string> = {
    "Content-Type": "application/json",
  };

  if (token) {
    defaultHeaders["Authorization"] = `Bearer ${token}`;
  }

  const config: RequestInit = {
    credentials: "include",
    headers: {
      ...defaultHeaders,
      ...(headers as Record<string, string>),
    },
    ...customConfig,
  };

  const response = await fetch(url, config);

  if (!response.ok) {
    let errorMessage = `HTTP Error ${response.status}: ${response.statusText}`;
    let errorData = null;
    try {
      errorData = await response.json();
      if (errorData?.msg || errorData?.message || errorData?.error) {
        errorMessage = errorData.msg || errorData.message || errorData.error;
      }
    } catch {
      try {
        errorMessage = await response.text();
      } catch {

      }
    }
    throw new ApiError(response.status, errorMessage, errorData);
  }

  const contentType = response.headers.get("content-type");
  if (contentType && contentType.includes("application/json")) {
    return response.json();
  }
  return response.text() as unknown as T;
}
