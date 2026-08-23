import { useState } from "react";
import { useAuth } from "../utils/contexts/authcontext";
import { useNavigate, Link } from "react-router-dom";

export default function Signin() {
    const { signin } = useAuth();
    const navigate = useNavigate();
    const [usernameEmail, setUsernameEmail] = useState("");
    const [password, setPassword] = useState("");
    const [error, setError] = useState<string | null>(null);
    const [loading, setLoading] = useState(false);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setError(null);
        setLoading(true);

        try {
            const resp = await signin({
                username_or_email: usernameEmail,
                password,
            });
            if (!resp) throw new Error("Invalid credentials");
            navigate("/profile");
        } catch (err) {
            setError((err as Error).message);
        } finally {
            setLoading(false);
        }
    };

    return (
        <div className="min-h-[calc(100vh-60px)] flex items-center justify-center bg-black px-4 py-12">
            <div className="w-full max-w-md p-8 bg-zinc-950 border border-zinc-900 rounded-3xl shadow-xl space-y-6">
                <div className="text-center space-y-2">
                    <h1 className="text-2xl font-extrabold text-white">Sign In</h1>
                </div>

                <form onSubmit={handleSubmit} className="space-y-4">
                    <div>
                        <label className="block text-xs font-medium text-zinc-400 mb-1.5 uppercase tracking-wider">
                            Username or Email
                        </label>
                        <input
                            type="text"
                            placeholder="Username or email"
                            value={usernameEmail}
                            onChange={(e) => setUsernameEmail(e.target.value)}
                            className="w-full px-3.5 py-2 bg-zinc-900 border border-zinc-800 rounded-xl text-white placeholder-zinc-600 text-xs focus:border-zinc-500 outline-none transition"
                            required
                        />
                    </div>

                    <div>
                        <label className="block text-xs font-medium text-zinc-400 mb-1.5 uppercase tracking-wider">
                            Password
                        </label>
                        <input
                            type="password"
                            placeholder="Password"
                            value={password}
                            onChange={(e) => setPassword(e.target.value)}
                            className="w-full px-3.5 py-2 bg-zinc-900 border border-zinc-800 rounded-xl text-white placeholder-zinc-600 text-xs focus:border-zinc-500 outline-none transition"
                            required
                        />
                    </div>

                    {error && (
                        <div className="p-3 bg-zinc-900 border border-zinc-800 rounded-xl text-xs text-zinc-300 text-center font-medium space-y-2">
                            <div>{error}</div>
                            {error.toLowerCase().includes("verify") && (
                                <div>
                                    <Link
                                        to="/verify-email"
                                        className="text-zinc-100 font-semibold underline hover:text-white"
                                    >
                                        Resend verification link
                                    </Link>
                                </div>
                            )}
                        </div>
                    )}

                    <button
                        type="submit"
                        disabled={loading}
                        className="w-full py-2.5 bg-zinc-100 hover:bg-white text-zinc-950 font-semibold rounded-xl shadow-sm transition disabled:opacity-50 disabled:cursor-not-allowed text-xs"
                    >
                        {loading ? "Signing in..." : "Sign In"}
                    </button>
                </form>

                <p className="text-center text-xs text-zinc-400">
                    Don't have an account?{" "}
                    <Link to="/signup" className="text-zinc-200 font-semibold hover:text-white underline">
                        Sign up
                    </Link>
                </p>
            </div>
        </div>
    );
}

