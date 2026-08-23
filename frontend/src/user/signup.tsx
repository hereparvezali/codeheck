import React, { useState } from "react";
import { Link } from "react-router-dom";
import { useAuth } from "../utils/contexts/authcontext";
import { UserService } from "../services/userService";

export default function Signup() {
    const [username, setUsername] = useState("");
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const [error, setError] = useState<string | null>(null);
    const [loading, setLoading] = useState(false);
    const [isSuccess, setIsSuccess] = useState(false);
    const [resendLoading, setResendLoading] = useState(false);
    const [resendMessage, setResendMessage] = useState<string | null>(null);
    const [resendError, setResendError] = useState<string | null>(null);
    const { base } = useAuth();

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setError(null);
        setLoading(true);

        try {
            const res = await fetch(base + "/user/signup", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
                    username,
                    email,
                    password,
                }),
            });

            if (!res.ok) {
                let errorMsg = "Signup failed";
                try {
                    const data = await res.json();
                    errorMsg = data.message || data.error || errorMsg;
                } catch {
                    try {
                        errorMsg = await res.text() || errorMsg;
                    } catch {
                        // fallback
                    }
                }
                throw new Error(errorMsg);
            }
            setIsSuccess(true);
        } catch (err) {
            setError((err as Error).message);
        } finally {
            setLoading(false);
        }
    };

    const handleResend = async () => {
        setResendLoading(true);
        setResendMessage(null);
        setResendError(null);
        try {
            const res = await UserService.resendVerification(email);
            setResendMessage(res.message || "Verification link resent successfully! Check your inbox.");
        } catch (err: any) {
            setResendError(err.message || "Failed to resend verification email.");
        } finally {
            setResendLoading(false);
        }
    };

    if (isSuccess) {
        return (
            <div className="min-h-[calc(100vh-60px)] flex items-center justify-center bg-black px-4 py-12">
                <div className="w-full max-w-md p-8 bg-zinc-950 border border-zinc-900 rounded-3xl shadow-xl space-y-6 text-center">
                    <div className="mx-auto w-12 h-12 bg-zinc-900 border border-zinc-800 rounded-2xl flex items-center justify-center text-xl">
                        ✉️
                    </div>

                    <div className="space-y-2">
                        <h1 className="text-2xl font-extrabold text-white">Check your email</h1>
                        <p className="text-xs text-zinc-400">
                            We've sent a verification link to{" "}
                            <span className="text-zinc-200 font-semibold">{email}</span>
                        </p>
                    </div>

                    <div className="p-4 bg-zinc-900/60 border border-zinc-800/80 rounded-2xl text-xs text-zinc-300 text-left space-y-2">
                        <p className="font-medium text-zinc-200">Next steps:</p>
                        <ul className="list-disc list-inside space-y-1 text-zinc-400">
                            <li>Check your inbox and click the verification link.</li>
                            <li>If you don't see it within a minute, check your <span className="text-zinc-200">spam/junk</span> folder.</li>
                        </ul>
                    </div>

                    {resendMessage && (
                        <div className="p-3 bg-emerald-950/30 border border-emerald-800/50 rounded-xl text-xs text-emerald-300 text-center font-medium">
                            {resendMessage}
                        </div>
                    )}

                    {resendError && (
                        <div className="p-3 bg-red-950/30 border border-red-800/50 rounded-xl text-xs text-red-300 text-center font-medium">
                            {resendError}
                        </div>
                    )}

                    <div className="space-y-3 pt-2">
                        <Link
                            to="/signin"
                            className="block w-full py-2.5 bg-zinc-100 hover:bg-white text-zinc-950 font-semibold rounded-xl shadow-sm transition text-xs"
                        >
                            Proceed to Sign In
                        </Link>

                        <button
                            type="button"
                            onClick={handleResend}
                            disabled={resendLoading}
                            className="w-full py-2.5 bg-zinc-900 hover:bg-zinc-800 text-zinc-300 hover:text-white font-medium rounded-xl border border-zinc-800 transition disabled:opacity-50 text-xs"
                        >
                            {resendLoading ? "Resending..." : "Didn't receive email? Resend"}
                        </button>
                    </div>
                </div>
            </div>
        );
    }

    return (
        <div className="min-h-[calc(100vh-60px)] flex items-center justify-center bg-black px-4 py-12">
            <div className="w-full max-w-md p-8 bg-zinc-950 border border-zinc-900 rounded-3xl shadow-xl space-y-6">
                <div className="text-center space-y-2">
                    <h1 className="text-2xl font-extrabold text-white">Sign Up</h1>
                </div>

                <form onSubmit={handleSubmit} className="space-y-4">
                    <div>
                        <label className="block text-xs font-medium text-zinc-400 mb-1.5 uppercase tracking-wider">
                            Username
                        </label>
                        <input
                            type="text"
                            placeholder="Username"
                            value={username}
                            onChange={(e) => setUsername(e.target.value)}
                            className="w-full px-3.5 py-2 bg-zinc-900 border border-zinc-800 rounded-xl text-white placeholder-zinc-600 text-xs focus:border-zinc-500 outline-none transition"
                            required
                        />
                    </div>

                    <div>
                        <label className="block text-xs font-medium text-zinc-400 mb-1.5 uppercase tracking-wider">
                            Email
                        </label>
                        <input
                            type="email"
                            placeholder="Email"
                            value={email}
                            onChange={(e) => setEmail(e.target.value)}
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
                        <div className="p-3 bg-red-950/30 border border-red-800/50 rounded-xl text-xs text-red-300 text-center font-medium space-y-1.5">
                            <div>{error}</div>
                            {error.toLowerCase().includes("already exists") && (
                                <div>
                                    <Link
                                        to="/signin"
                                        className="text-white font-semibold underline hover:text-zinc-200"
                                    >
                                        Sign In with this account
                                    </Link>
                                </div>
                            )}
                        </div>
                    )}

                    <button
                        type="submit"
                        disabled={loading}
                        className="w-full py-2.5 bg-zinc-100 hover:bg-white text-zinc-950 font-semibold rounded-xl shadow-sm transition disabled:opacity-50 disabled:cursor-not-allowed text-xs flex items-center justify-center gap-2"
                    >
                        {loading ? (
                            <>
                                <span className="inline-block w-3.5 h-3.5 border-2 border-zinc-950 border-t-transparent rounded-full animate-spin"></span>
                                Sending verification email...
                            </>
                        ) : (
                            "Sign Up"
                        )}
                    </button>
                </form>

                <p className="text-center text-xs text-zinc-400">
                    Already have an account?{" "}
                    <Link to="/signin" className="text-zinc-200 font-semibold hover:text-white underline">
                        Sign In
                    </Link>
                </p>
            </div>
        </div>
    );
}

