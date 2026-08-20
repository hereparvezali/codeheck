import React, {
  createContext,
  useState,
  type ReactNode,
  useEffect,
  useContext,
} from "react";

interface User {
  id: number;
  username: string;
  email: string;
  access_token: string;
}

interface LoginPayload {
  username_or_email: string;
  password: string;
}

interface AuthContextType {
  user: User | null;
  loading: boolean;
  signin: (params: LoginPayload) => Promise<string | null>;
  refresh: () => Promise<string | null>;
  signout: () => Promise<void>;
  authfetch: (url: string, options?: RequestInit) => Promise<Response>;
  setUser: React.Dispatch<React.SetStateAction<User | null>>;
  base: string;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

interface AuthProviderProps {
  children: ReactNode;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [loading, setLoading] = useState(true);
  const [user, setUser] = useState<User | null>(null);
  const base: string = import.meta.env.VITE_BASE || "/api";


  const signin = async (params: LoginPayload): Promise<string | null> => {
    const res = await fetch(base + "/user/signin", {
      method: "POST",
      credentials: "include",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(params),
    });

    if (!res.ok) {
      setUser(null);
      throw new Error(await res.text());
    }

    const data: User = await res.json();
    setUser(data);
    return data.access_token;
  };

  const refresh = async (): Promise<string | null> => {
    try {
      const res = await fetch(base + "/user/refresh", {
        method: "POST",
        credentials: "include",
      });

      if (!res.ok) throw new Error(res.statusText);
      const data: User = await res.json();
      setUser(data);
      return data.access_token;
    } catch (e) {
      setUser(null);
      localStorage.removeItem("user");
      console.error("Refresh failed:", e);
      return null;
    } finally {
      setLoading(false);
    }
  };

  const signout = async (): Promise<void> => {
    setUser(null);
    localStorage.removeItem("user");
    try {
      await fetch(base + "/user/signout", {
        method: "POST",
        credentials: "include",
      });
    } catch (e) {
      console.error("Logout error:", e);
    }
  };

  const authfetch = async (
    url: string,
    options: RequestInit = {},
  ): Promise<Response> => {
    let token = user?.access_token ?? null;

    const headers: Record<string, string> = {
      ...(options.headers as Record<string, string> || {}),
    };
    if (token) {
      headers["Authorization"] = `Bearer ${token}`;
    }

    let res = await fetch(base + url, {
      ...options,
      headers,
    });

    if (res.status === 401 && user) {
      token = await refresh();
      if (!token) return res;
      headers["Authorization"] = `Bearer ${token}`;
      res = await fetch(base + url, {
        ...options,
        headers,
      });
    }
    return res;
  };

  useEffect(() => {
    const stored = localStorage.getItem("user");
    if (!stored) {
      refresh();
    } else {
      try {
        const parsed: User = JSON.parse(stored);
        if (parsed && typeof parsed.id === "number" && parsed.access_token) {
          setUser(parsed);
          setLoading(false);
        } else {
          refresh();
        }
      } catch {
        localStorage.removeItem("user");
        refresh();
      }
    }
  }, []);

  useEffect(() => {
    if (!user) return;
    localStorage.setItem("user", JSON.stringify(user));
  }, [user]);

  return (
    <AuthContext.Provider
      value={{ user, loading, signin, refresh, signout, authfetch, setUser, base }}
    >
      {children}
    </AuthContext.Provider>
  );
};


export const useAuth = (): AuthContextType => {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error("useAuth must be used within AuthProvider");
  return ctx;
};
