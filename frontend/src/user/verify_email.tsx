import React, { useEffect, useState } from "react";
import { useSearchParams, Link } from "react-router-dom";
import { UserService } from "../services/userService";

export default function VerifyEmail() {
    const [searchParams] = useSearchParams();
    const token = searchParams.get("token");

    const [status, setStatus] = useState<"loading" | "success" | "error" | "idle">(
        token ? "loading" : "idle"
    );
    const [message, setMessage] = useState<string>("");
    const [email, setEmail] = useState<string>("");
    const [resendLoading, setResendLoading] = useState<boolean>(false);
    const [resendSuccess, setResendSuccess] = useState<string | null>(null);
    const [resendError, setResendError] = useState<string | null>(null);

    useEffect(() => {
        if (!token) {
            setStatus("idle");
            return;
        }

        let isMounted = true;

        UserService.verifyEmail(token)
            .then((res) => {
                if (isMounted) {
                    setStatus("success");
                    setMessage(res.message || "Email verified successfully! You can now sign in.");
                }
            })
            .catch((err) => {
                if (isMounted) {
                    setStatus("error");
                    setMessage(err.message || "Invalid or expired verification token.");
                }
            });

        return () => {
            isMounted = false;
        };
    }, [token]);

    const handleResend = async (e: React.FormEvent) => {
        e.preventDefault();
        setResendError(null);
        setResendSuccess(null);
        setResendLoading(true);

        try {
            const res = await UserService.resendVerification(email);
            setResendSuccess(res.message || "Verification email resent successfully! Please check your inbox.");
        } catch (err: any) {
            setResendError(err.message || "Failed to resend verification email.");
        } finally {
            setResendLoading(false);
        }
    };

    return (
        <div className="min-h-[calc(100vh-60px)] flex items-center justify-center bg-black px-4 py-12">
            <div className="w-full max-w-md p-8 bg-zinc-950 border border-zinc-900 rounded-3xl shadow-xl space-y-6">
                <div className="text-center space-y-2">
                    <h1 className="text-2xl font-extrabold text-white">Email Verification</h1>
                </div>

                {status === "loading" && (
                    <div className="text-center py-6 space-y-3">
                        <div className="inline-block w-8 h-8 border-2 border-zinc-500 border-t-white rounded-full animate-spin"></div>
                        <p className="text-xs text-zinc-400">Verifying your email address...</p>
                    </div>
                )}

                {status === "success" && (
                    <div className="space-y-4">
                        <div className="p-4 bg-emerald-950/30 border border-emerald-800/50 rounded-xl text-xs text-emerald-300 text-center font-medium">
                            {message}
                        </div>
                        <Link
                            to="/signin"
                            className="block w-full text-center py-2.5 bg-zinc-100 hover:bg-white text-zinc-950 font-semibold rounded-xl shadow-sm transition text-xs"
                        >
                            Proceed to Sign In
                        </Link>
                    </div>
                )}

                {(status === "error" || status === "idle") && (
                    <div className="space-y-5">
                        {status === "error" && (
                            <div className="p-3 bg-red-950/30 border border-red-800/50 rounded-xl text-xs text-red-300 text-center font-medium">
                                {message}
                            </div>
                        )}

                        <div className="space-y-2 text-center">
                            <h2 className="text-sm font-semibold text-zinc-200">Resend Verification Link</h2>
                            <p className="text-xs text-zinc-400">
                                Enter your account email address below to receive a new verification link.
                            </p>
                        </div>

                        <form onSubmit={handleResend} className="space-y-4">
                            <div>
                                <label className="block text-xs font-medium text-zinc-400 mb-1.5 uppercase tracking-wider">
                                    Email Address
                                </label>
                                <input
                                    type="email"
                                    placeholder="your-email@example.com"
                                    value={email}
                                    onChange={(e) => setEmail(e.target.value)}
                                    className="w-full px-3.5 py-2 bg-zinc-900 border border-zinc-800 rounded-xl text-white placeholder-zinc-600 text-xs focus:border-zinc-500 outline-none transition"
                                    required
                                />
                            </div>

                            {resendSuccess && (
                                <div className="p-3 bg-emerald-950/30 border border-emerald-800/50 rounded-xl text-xs text-emerald-300 text-center font-medium">
                                    {resendSuccess}
                                </div>
                            )}

                            {resendError && (
                                <div className="p-3 bg-red-950/30 border border-red-800/50 rounded-xl text-xs text-red-300 text-center font-medium">
                                    {resendError}
                                </div>
                            )}

                            <button
                                type="submit"
                                disabled={resendLoading}
                                className="w-full py-2.5 bg-zinc-100 hover:bg-white text-zinc-950 font-semibold rounded-xl shadow-sm transition disabled:opacity-50 disabled:cursor-not-allowed text-xs"
                            >
                                {resendLoading ? "Sending..." : "Resend Verification Email"}
                            </button>
                        </form>

                        <div className="text-center pt-2">
                            <Link to="/signin" className="text-xs text-zinc-400 hover:text-white underline">
                                Back to Sign In
                            </Link>
                        </div>
                    </div>
                )}
            </div>
        </div>
    );
}
