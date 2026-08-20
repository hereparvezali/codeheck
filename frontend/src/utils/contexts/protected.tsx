import { Navigate } from "react-router-dom";
import { useAuth } from "../contexts/authcontext";
import type { JSX } from "react";

interface Props {
    children: JSX.Element;
}

export default function ProtectedRoute({ children }: Props) {
    const { user, loading } = useAuth();

    if (loading) {
        return (
            <div className="min-h-[calc(100vh-60px)] bg-black flex items-center justify-center">
                <div className="w-7 h-7 border-2 border-zinc-500 border-t-transparent rounded-full animate-spin" />
            </div>
        );
    }

    if (!user) {
        return <Navigate to="/signin" replace />;
    }
    return children;
}

