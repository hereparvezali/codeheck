import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { useAuth } from "../utils/contexts/authcontext";
import { parseUtcDate } from "../utils/helpers";
import ProfileOverview, { type UserStats } from "./ProfileOverview";
import ProfileSolved from "./ProfileSolved";
import ProfileSubmissions from "./ProfileSubmissions";

export default function Profile() {
    const { username } = useParams<{ username?: string }>();
    const navigate = useNavigate();
    const { authfetch, signout, user } = useAuth();
    const [loading, setLoading] = useState(true);
    const [stats, setStats] = useState<UserStats | null>(null);
    const [error, setError] = useState<string | null>(null);
    const [activeTab, setActiveTab] = useState<"overview" | "solved" | "submissions">("overview");

    const isOwnProfile = !username || (user && user.username === username);

    useEffect(() => {
        const fetchStats = async () => {
            setLoading(true);
            setError(null);
            try {
                let url = "/user/stats";
                if (username) {
                    url += `?username=${username}`;
                } else if (!user) {
                    navigate("/signin");
                    return;
                }

                const res = await authfetch(url);
                if (!res.ok) {
                    if (res.status === 401) {
                        navigate("/signin");
                        return;
                    }
                    throw new Error(await res.text() || "Failed to load user profile");
                }

                const data: UserStats = await res.json();
                setStats(data);
            } catch (err) {
                setError((err as Error).message);
            } finally {
                setLoading(false);
            }
        };

        fetchStats();
    }, [username, user, authfetch, navigate]);

    const handleSignout = async () => {
        await signout();
        navigate("/");
    };

    if (loading) {
        return (
            <div className="min-h-screen bg-black flex items-center justify-center text-white">
                <div className="flex flex-col items-center gap-3">
                    <div className="w-7 h-7 border-2 border-zinc-400 border-t-transparent rounded-full animate-spin" />
                    <p className="text-zinc-400 text-sm">Loading profile data...</p>
                </div>
            </div>
        );
    }

    if (error || !stats) {
        return (
            <div className="min-h-screen bg-black flex items-center justify-center p-6">
                <div className="bg-zinc-950 border border-zinc-900 p-8 rounded-2xl max-w-md w-full text-center">
                    <h2 className="text-xl font-bold text-zinc-200 mb-2">User Not Found</h2>
                    <p className="text-zinc-400 text-sm mb-6">{error || "Could not load user stats."}</p>
                    <button
                        onClick={() => navigate(-1)}
                        className="px-6 py-2.5 bg-zinc-100 hover:bg-white text-zinc-950 rounded-xl font-semibold transition shadow-sm"
                    >
                        &larr; Go Back
                    </button>
                </div>
            </div>
        );
    }

    return (
        <div className="min-h-screen bg-black text-zinc-100 py-10 px-4 sm:px-6 lg:px-8">
            <div className="max-w-5xl mx-auto space-y-6">
                {}
                <div className="bg-zinc-950 border border-zinc-900 rounded-2xl p-6 shadow-lg flex flex-col sm:flex-row justify-between items-start sm:items-center gap-6">
                    <div className="flex items-center gap-5">
                        <div className="w-16 h-16 rounded-2xl bg-zinc-900 border border-zinc-800 flex items-center justify-center text-2xl font-bold text-white shadow-sm">
                            {stats.username.charAt(0).toUpperCase()}
                        </div>

                        <div>
                            <div className="flex items-center gap-3">
                                <h1 className="text-2xl sm:text-3xl font-extrabold text-white">{stats.username}</h1>
                                <span className="px-2.5 py-0.5 rounded-full text-xs font-semibold bg-zinc-900 text-zinc-300 border border-zinc-800">
                                    ★ {stats.rating}
                                </span>
                            </div>
                            <p className="text-xs text-zinc-400 mt-1">{stats.email}</p>
                            <p className="text-xs text-zinc-500 mt-1">
                                Member since {parseUtcDate(stats.created_at).toLocaleDateString()}
                            </p>
                        </div>
                    </div>

                    {isOwnProfile && (
                        <button
                            onClick={handleSignout}
                            className="px-4 py-2 bg-zinc-900 hover:bg-zinc-800 text-zinc-300 border border-zinc-800 rounded-xl text-xs font-semibold transition"
                        >
                            Sign Out
                        </button>
                    )}
                </div>

                {}
                <div className="bg-zinc-950 border border-zinc-900 rounded-2xl overflow-hidden shadow-lg">
                    <div className="flex border-b border-zinc-900 bg-zinc-950">
                        <button
                            onClick={() => setActiveTab("overview")}
                            className={`flex-1 py-3 text-xs sm:text-sm font-semibold transition ${
                                activeTab === "overview"
                                    ? "bg-zinc-900 text-white border-b-2 border-zinc-300"
                                    : "text-zinc-400 hover:text-zinc-200"
                            }`}
                        >
                            Overview
                        </button>
                        <button
                            onClick={() => setActiveTab("solved")}
                            className={`flex-1 py-3 text-xs sm:text-sm font-semibold transition ${
                                activeTab === "solved"
                                    ? "bg-zinc-900 text-white border-b-2 border-zinc-300"
                                    : "text-zinc-400 hover:text-zinc-200"
                            }`}
                        >
                            Solved ({stats.total_solved})
                        </button>
                        <button
                            onClick={() => setActiveTab("submissions")}
                            className={`flex-1 py-3 text-xs sm:text-sm font-semibold transition ${
                                activeTab === "submissions"
                                    ? "bg-zinc-900 text-white border-b-2 border-zinc-300"
                                    : "text-zinc-400 hover:text-zinc-200"
                            }`}
                        >
                            Submissions ({stats.total_submissions})
                        </button>
                    </div>

                    <div className="p-6">
                        {activeTab === "overview" && <ProfileOverview stats={stats} />}
                        {activeTab === "solved" && <ProfileSolved userId={stats.user_id} />}
                        {activeTab === "submissions" && <ProfileSubmissions userId={stats.user_id} />}
                    </div>
                </div>
            </div>
        </div>
    );
}
