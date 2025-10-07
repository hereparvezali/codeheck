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
    const base: string = import.meta.env.VITE_BASE;

    // Login function expects exactly one of username or email + password
    const signin = async (params: LoginPayload): Promise<string | null> => {
        const res = await fetch(base + "/user/signin", {
            method: "POST",
            credentials: "include",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(params),
        });

        if (!res.ok) {
            setUser(null);
            throw new Error(await res.json());
        }

        const data: User = await res.json();
        setUser(data);
        return data.access_token;
    };

    const refresh = async (): Promise<string | null> => {
        try {
            const res = await fetch(base + "/user/refresh", {
                method: "GET",
                credentials: "include",
            });

            if (!res.ok) throw new Error(res.statusText);
            const data: User = await res.json();
            console.log(data);
            setUser(data);
            return data.access_token;
        } catch (e) {
            setUser(null);
            console.error("Refresh failed:", e);
            return null;
        } finally {
            setLoading(false);
        }
    };

    const signout = async (): Promise<void> => {
        try {
            setUser(null);
            await fetch(base + "/user/signout", {
                method: "GET",
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
        if (!token) {
            token = await refresh();
        }

        let res = await fetch(base + url, {
            ...options,
            headers: {
                ...options.headers,
                Authorization: `Bearer ${token}`,
            },
        });

        if (res.status === 401) {
            token = await refresh();
            res = await fetch(base + url, {
                ...options,
                headers: {
                    ...options.headers,
                    Authorization: `Bearer ${token}`,
                },
            });
        }
        return res;
    };

    // Optional: on mount, refresh user to persist signin after reload
    useEffect(() => {
        const stored = localStorage.getItem("user");
        if (!stored) {
            refresh();
        } else {
            const parsed: User = JSON.parse(stored);
            setUser(parsed);
            setLoading(false);
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    useEffect(() => {
        if (!user) return;
        localStorage.setItem("user", JSON.stringify(user));
    }, [user]);

    return (
        <AuthContext.Provider
            value={{ user, signin, refresh, signout, authfetch, setUser, base }}
        >
            {!loading && children}
        </AuthContext.Provider>
    );
};

// eslint-disable-next-line react-refresh/only-export-components
export const useAuth = (): AuthContextType => {
    const ctx = useContext(AuthContext);
    if (!ctx) throw new Error("useAuth must be used within AuthProvider");
    return ctx;
};
