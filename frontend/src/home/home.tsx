import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { useAuth } from "../utils/contexts/authcontext";
import { formatUtcToLocal, getStatusColor, parseUtcDate } from "../utils/helpers";
import type { Problem } from "../problems/problems";
import type { Contest } from "../contests/contests";

export default function Home() {
    const { user, authfetch } = useAuth();
    const [recentProblems, setRecentProblems] = useState<Problem[]>([]);
    const [upcomingContests, setUpcomingContests] = useState<Contest[]>([]);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const loadHomeData = async () => {
            try {
                const [probRes, contestRes] = await Promise.all([
                    authfetch("/problems?limit=8"),
                    authfetch("/contests?limit=5"),
                ]);

                if (probRes.ok) {
                    const data = await probRes.json();
                    setRecentProblems(data.problems || []);
                }
                if (contestRes.ok) {
                    const data = await contestRes.json();
                    setUpcomingContests(data.contests || []);
                }
            } catch {

            } finally {
                setLoading(false);
            }
        };

        loadHomeData();
    }, [authfetch]);

    const difficultyBadge = (diff?: string) => {
        const d = diff?.toLowerCase();
        if (d === "easy") return "bg-zinc-900 text-zinc-300 border-zinc-800";
        if (d === "medium") return "bg-zinc-800 text-zinc-200 border-zinc-700 font-medium";
        if (d === "hard") return "bg-zinc-700 text-white border-zinc-600 font-semibold";
        return "bg-zinc-900 text-zinc-400 border-zinc-800";
    };

    return (
        <div className="min-h-screen bg-black text-zinc-100 flex flex-col justify-between">
            <div className="max-w-6xl mx-auto w-full px-6 py-12 space-y-12">
                <section className="text-center space-y-4 pt-4">
                    <h1 className="text-3xl sm:text-5xl font-extrabold tracking-tight text-white">
                        CodeHeck
                    </h1>
                    <p className="text-sm sm:text-base text-zinc-400 max-w-xl mx-auto">
                        Competitive programming judge and real-time contests.
                    </p>

                    <div className="flex flex-wrap justify-center gap-3 pt-2">
                        <Link
                            to="/problems"
                            className="px-5 py-2 bg-zinc-100 hover:bg-white text-zinc-950 text-xs font-semibold rounded-xl transition shadow-sm"
                        >
                            Browse Problems
                        </Link>
                        <Link
                            to="/contests"
                            className="px-5 py-2 bg-zinc-900 hover:bg-zinc-800 text-zinc-200 text-xs font-semibold rounded-xl border border-zinc-800 transition"
                        >
                            Contests Arena
                        </Link>
                        {!user && (
                            <Link
                                to="/signin"
                                className="px-5 py-2 bg-zinc-900 hover:bg-zinc-800 text-zinc-400 hover:text-white text-xs font-semibold rounded-xl border border-zinc-900 transition"
                            >
                                Sign In
                            </Link>
                        )}
                    </div>
                </section>

                <section className="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <div className="bg-zinc-950 border border-zinc-900 rounded-2xl p-6 space-y-4">
                        <div className="flex justify-between items-center pb-2 border-b border-zinc-900">
                            <h2 className="text-sm font-bold text-white uppercase tracking-wider">Problems</h2>
                            <Link to="/problems" className="text-xs text-zinc-500 hover:text-zinc-300 transition">
                                View all &rarr;
                            </Link>
                        </div>

                        {loading ? (
                            <div className="space-y-2">
                                {[1, 2, 3, 4].map((i) => (
                                    <div key={i} className="h-10 bg-zinc-900 rounded-xl animate-pulse" />
                                ))}
                            </div>
                        ) : recentProblems.length === 0 ? (
                            <div className="py-8 text-center text-xs text-zinc-500">
                                No problems available.
                            </div>
                        ) : (
                            <div className="space-y-1.5">
                                {recentProblems.map((prob) => (
                                    <Link
                                        key={prob.id}
                                        to={`/problems/${prob.slug || prob.id}`}
                                        className="flex items-center justify-between p-2.5 bg-zinc-900/40 hover:bg-zinc-900 border border-zinc-900 hover:border-zinc-800 rounded-xl transition"
                                    >
                                        <div className="flex items-center gap-2.5 min-w-0">
                                            <span className="text-zinc-600 text-xs font-mono">#{prob.id}</span>
                                            <span className="text-xs font-medium text-zinc-200 hover:text-white truncate">
                                                {prob.title}
                                            </span>
                                        </div>
                                        <div className="flex items-center gap-2 shrink-0">
                                            {prob.status && (
                                                <span className={`px-2 py-0.5 text-[11px] rounded ${getStatusColor(prob.status)}`}>
                                                    {prob.status}
                                                </span>
                                            )}
                                            <span
                                                className={`px-2 py-0.5 text-[11px] rounded border ${difficultyBadge(
                                                    prob.difficulty
                                                )}`}
                                            >
                                                {prob.difficulty || "Unrated"}
                                            </span>
                                        </div>
                                    </Link>
                                ))}
                            </div>
                        )}
                    </div>

                    <div className="bg-zinc-950 border border-zinc-900 rounded-2xl p-6 space-y-4">
                        <div className="flex justify-between items-center pb-2 border-b border-zinc-900">
                            <h2 className="text-sm font-bold text-white uppercase tracking-wider">Contests</h2>
                            <Link to="/contests" className="text-xs text-zinc-500 hover:text-zinc-300 transition">
                                View all &rarr;
                            </Link>
                        </div>

                        {loading ? (
                            <div className="space-y-2">
                                {[1, 2, 3].map((i) => (
                                    <div key={i} className="h-14 bg-zinc-900 rounded-xl animate-pulse" />
                                ))}
                            </div>
                        ) : upcomingContests.length === 0 ? (
                            <div className="py-8 text-center text-xs text-zinc-500">
                                No active contests scheduled.
                            </div>
                        ) : (
                            <div className="space-y-2">
                                {upcomingContests.map((c) => {
                                    const now = new Date();
                                    const start = parseUtcDate(c.start_time);
                                    const end = parseUtcDate(c.end_time);
                                    const isOngoing = now >= start && now <= end;
                                    const isUpcoming = now < start;

                                    return (
                                        <Link
                                            key={c.id}
                                            to={`/contests/${c.id}`}
                                            className="block p-3 bg-zinc-900/40 hover:bg-zinc-900 border border-zinc-900 hover:border-zinc-800 rounded-xl transition"
                                        >
                                            <div className="flex justify-between items-start mb-1">
                                                <h3 className="text-xs font-semibold text-zinc-100 hover:text-white truncate">
                                                    {c.title}
                                                </h3>
                                                <span
                                                    className={`px-2 py-0.5 text-[11px] rounded border shrink-0 ${
                                                        isOngoing
                                                            ? "bg-zinc-800 text-white border-zinc-600 font-semibold"
                                                            : isUpcoming
                                                            ? "bg-zinc-900 text-zinc-300 border-zinc-800"
                                                            : "bg-zinc-950 text-zinc-600 border-zinc-900"
                                                    }`}
                                                >
                                                    {isOngoing ? "Live" : isUpcoming ? "Upcoming" : "Ended"}
                                                </span>
                                            </div>
                                            <div className="text-[11px] text-zinc-500 font-mono">
                                                {formatUtcToLocal(c.start_time)} &rarr; {formatUtcToLocal(c.end_time)}
                                            </div>
                                        </Link>
                                    );
                                })}
                            </div>
                        )}
                    </div>
                </section>
            </div>

            <footer className="border-t border-zinc-900 py-6 px-6 text-center text-xs text-zinc-600">
                <div className="max-w-6xl mx-auto flex flex-col sm:flex-row justify-between items-center gap-2">
                    <span className="font-semibold text-zinc-400">CodeHeck</span>
                    <div className="flex gap-4">
                        <Link to="/problems" className="hover:text-zinc-400 transition">Problems</Link>
                        <Link to="/contests" className="hover:text-zinc-400 transition">Contests</Link>
                        <Link to="/profile" className="hover:text-zinc-400 transition">Profile</Link>
                    </div>
                </div>
            </footer>
        </div>
    );
}

