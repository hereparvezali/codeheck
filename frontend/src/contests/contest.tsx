import { useEffect, useState } from "react";
import { useNavigate, useParams, Link } from "react-router-dom";
import { useAuth } from "../utils/contexts/authcontext";
import { formatUtcToLocal, parseUtcDate } from "../utils/helpers";

interface LeaderboardEntry {
    user_id: number;
    username: string;
    solved: number;
    penalty: number;
    problems: Record<number, ProblemStatus>;
}

interface ProblemStatus {
    solved: boolean;
    attempts: number;
    time: number | null;
}

interface Contest {
    id: number;
    title: string;
    description?: string;
    start_time: string;
    end_time: string;
    is_public: boolean;
    author_id?: number;
    registration_id?: number;
}

interface ProblemResponse {
    id: number;
    title: string;
    slug: string;
    difficulty: string;
    label?: string;
}

export default function ContestDetail() {
    const navigate = useNavigate();
    const { id } = useParams<{ id: string }>();
    const { authfetch, user } = useAuth();

    const [contest, setContest] = useState<Contest | null>(null);
    const [problems, setProblems] = useState<ProblemResponse[]>([]);
    const [problemsLoading, setProblemsLoading] = useState(false);
    const [problemsError, setProblemsError] = useState<string | null>(null);
    const [leaderboard, setLeaderboard] = useState<LeaderboardEntry[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [activeTab, setActiveTab] = useState<"problems" | "leaderboard">("problems");
    const [timeLeft, setTimeLeft] = useState<{ status: "upcoming" | "ongoing" | "ended"; text: string; percent: number }>({
        status: "upcoming",
        text: "",
        percent: 0,
    });
    const [registering, setRegistering] = useState(false);

    const fetchContest = async () => {
        if (!id) return;
        try {
            const res = await authfetch(`/contest?id=${id}`);
            if (!res.ok) {
                throw new Error(await res.text() || "Failed to load contest");
            }
            const data = await res.json();
            setContest(data);
        } catch (err) {
            setError((err as Error).message);
        } finally {
            setLoading(false);
        }
    };

    const fetchProblems = async (contestId: number) => {
        setProblemsLoading(true);
        try {
            const res = await authfetch(`/contest/problems?id=${contestId}`);
            if (res.ok) {
                const data = await res.json();
                setProblems(data || []);
                setProblemsError(null);
            } else {
                setProblems([]);
                let msg = "Problems locked";
                try {
                    const errData = await res.json();
                    if (errData.message) msg = errData.message;
                } catch {

                }
                setProblemsError(msg);
            }
        } catch {

        } finally {
            setProblemsLoading(false);
        }
    };

    const fetchLeaderboard = async (contestId: number) => {
        try {
            const res = await authfetch(`/contest/leaderboard?contest_id=${contestId}`);
            if (res.ok) {
                const data = await res.json();
                setLeaderboard(data.standings || []);
            }
        } catch {

        }
    };

    useEffect(() => {
        fetchContest();
    }, [id, user]);

    useEffect(() => {
        if (!contest) return;
        fetchProblems(contest.id);
        fetchLeaderboard(contest.id);

        const baseUrl = import.meta.env.VITE_BASE || "http://localhost:8000/api";
        const tokenParam = user?.access_token ? `&token=${encodeURIComponent(user.access_token)}` : "";
        const wsUrl = baseUrl.replace(/^http/, "ws") + `/contest/leaderboard/ws?contest_id=${contest.id}${tokenParam}`;

        let ws: WebSocket | null = null;
        let pollTimer: ReturnType<typeof setInterval> | null = null;

        try {
            ws = new WebSocket(wsUrl);
            ws.onmessage = (event) => {
                try {
                    const data = JSON.parse(event.data);
                    if (data.standings) {
                        setLeaderboard(data.standings);
                    }
                } catch {

                }
            };
            ws.onerror = () => {
                if (!pollTimer) {
                    pollTimer = setInterval(() => fetchLeaderboard(contest.id), 8000);
                }
            };
            ws.onclose = () => {
                if (!pollTimer) {
                    pollTimer = setInterval(() => fetchLeaderboard(contest.id), 8000);
                }
            };
        } catch {
            pollTimer = setInterval(() => fetchLeaderboard(contest.id), 8000);
        }

        return () => {
            if (ws) ws.close();
            if (pollTimer) clearInterval(pollTimer);
        };
    }, [contest?.id, contest?.registration_id, timeLeft.status, user?.access_token]);

    useEffect(() => {
        if (!contest) return;

        const updateTimer = () => {
            const now = new Date().getTime();
            const start = parseUtcDate(contest.start_time).getTime();
            const end = parseUtcDate(contest.end_time).getTime();

            if (now < start) {
                const diff = Math.max(0, start - now);
                const hrs = Math.floor(diff / (1000 * 60 * 60));
                const mins = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
                const secs = Math.floor((diff % (1000 * 60)) / 1000);
                setTimeLeft({
                    status: "upcoming",
                    text: `Starts in ${hrs.toString().padStart(2, "0")}:${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`,
                    percent: 0,
                });
            } else if (now >= start && now <= end) {
                const total = end - start;
                const elapsed = now - start;
                const diff = Math.max(0, end - now);
                const hrs = Math.floor(diff / (1000 * 60 * 60));
                const mins = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
                const secs = Math.floor((diff % (1000 * 60)) / 1000);
                setTimeLeft({
                    status: "ongoing",
                    text: `Ends in ${hrs.toString().padStart(2, "0")}:${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`,
                    percent: Math.min(100, (elapsed / total) * 100),
                });
            } else {
                setTimeLeft({
                    status: "ended",
                    text: "Ended",
                    percent: 100,
                });
            }
        };

        updateTimer();
        const timer = setInterval(updateTimer, 1000);
        return () => clearInterval(timer);
    }, [contest]);

    const handleRegisterToggle = async () => {
        if (!user) {
            navigate("/signin");
            return;
        }
        if (!contest || registering) return;

        setRegistering(true);
        try {
            if (contest.registration_id) {
                const res = await authfetch(`/contest/registration?id=${contest.registration_id}`, {
                    method: "DELETE",
                });
                if (res.ok) {
                    setContest({ ...contest, registration_id: undefined });
                    setProblems([]);
                }
            } else {
                const res = await authfetch("/contest/registration", {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({ contest_id: contest.id }),
                });
                if (res.ok) {
                    const data = await res.json();
                    const newRegId = data.id || 1;
                    setContest({ ...contest, registration_id: newRegId });
                    fetchProblems(contest.id);
                }
            }
        } catch {

        } finally {
            setRegistering(false);
        }
    };

    if (loading) {
        return (
            <div className="min-h-screen bg-black flex items-center justify-center text-white">
                <div className="flex flex-col items-center gap-3">
                    <div className="w-8 h-8 border-2 border-zinc-400 border-t-transparent rounded-full animate-spin" />
                    <p className="text-zinc-400 text-sm">Loading contest...</p>
                </div>
            </div>
        );
    }

    if (error || !contest) {
        return (
            <div className="min-h-screen bg-black flex items-center justify-center p-6">
                <div className="bg-zinc-950 border border-zinc-900 p-8 rounded-2xl max-w-md w-full text-center">
                    <h2 className="text-xl font-bold text-zinc-200 mb-2">Contest Not Found</h2>
                    <p className="text-zinc-400 text-sm mb-6">{error || "Could not find contest details."}</p>
                    <Link
                        to="/contests"
                        className="px-6 py-2.5 bg-zinc-100 hover:bg-white text-zinc-950 rounded-xl font-semibold transition shadow-sm"
                    >
                        Back to Contests
                    </Link>
                </div>
            </div>
        );
    }

    return (
        <div className="min-h-screen bg-black text-zinc-100 py-10 px-4 sm:px-6 lg:px-8">
            <div className="max-w-6xl mx-auto space-y-6">
                <div className="bg-zinc-950 border border-zinc-900 rounded-2xl p-6 shadow-lg relative overflow-hidden">
                    {timeLeft.status === "ongoing" && (
                        <div className="absolute top-0 left-0 right-0 h-1 bg-zinc-900">
                            <div
                                className="h-full bg-zinc-400 transition-all duration-1000"
                                style={{ width: `${timeLeft.percent}%` }}
                            />
                        </div>
                    )}


                    <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-6">
                        <div>
                            <div className="flex items-center gap-3 mb-2">
                                <span className="font-mono text-xs text-zinc-500">#{contest.id}</span>
                                <h1 className="text-2xl sm:text-3xl font-extrabold text-white">{contest.title}</h1>
                                <span
                                    className={`px-3 py-1 rounded-full text-xs font-semibold border ${
                                        timeLeft.status === "ongoing"
                                            ? "bg-zinc-800 text-white border-zinc-600"
                                            : timeLeft.status === "upcoming"
                                            ? "bg-zinc-900 text-zinc-300 border-zinc-800"
                                            : "bg-zinc-950 text-zinc-600 border-zinc-900"
                                    }`}
                                >
                                    {timeLeft.text}
                                </span>
                            </div>

                            {contest.description && (
                                <p className="text-sm text-zinc-400 max-w-2xl mt-1 leading-relaxed">
                                    {contest.description}
                                </p>
                            )}

                            <div className="flex flex-wrap gap-4 text-xs text-zinc-500 mt-3">
                                <div>
                                    <span className="text-zinc-400">{formatUtcToLocal(contest.start_time)}</span> &rarr;{" "}
                                    <span className="text-zinc-400">{formatUtcToLocal(contest.end_time)}</span>
                                </div>
                                <div>
                                    <span>{contest.is_public ? "Public" : "Private"}</span>
                                </div>
                            </div>
                        </div>

                        <div className="flex items-center gap-3">
                            {contest.author_id === user?.id && (
                                <Link
                                    to={`/admin/edit_contest/${contest.id}`}
                                    className="px-4 py-2 bg-zinc-900 hover:bg-zinc-800 text-zinc-200 rounded-xl text-sm font-semibold border border-zinc-800 transition"
                                >
                                    Edit Contest
                                </Link>
                            )}

                            {timeLeft.status === "ended" ? (
                                <button
                                    disabled
                                    className="px-5 py-2 rounded-xl text-xs font-semibold bg-zinc-900 text-zinc-600 border border-zinc-800/50 cursor-not-allowed"
                                >
                                    Ended
                                </button>
                            ) : (
                                <button
                                    onClick={handleRegisterToggle}
                                    disabled={registering}
                                    className={`px-5 py-2 rounded-xl text-xs font-semibold transition ${
                                        contest.registration_id
                                            ? "bg-zinc-900 hover:bg-zinc-800 text-zinc-300 border border-zinc-800"
                                            : "bg-zinc-100 hover:bg-white text-zinc-950 shadow-sm"
                                    }`}
                                >
                                    {registering
                                        ? "Updating..."
                                        : contest.registration_id
                                        ? "Registered (Unregister)"
                                        : "Register for Contest"}
                                </button>
                            )}
                        </div>
                    </div>
                </div>

                <div className="flex items-center gap-1.5 p-1 bg-zinc-950 border border-zinc-900 rounded-xl w-fit">
                    <button
                        onClick={() => setActiveTab("problems")}
                        className={`px-4 py-1.5 rounded-lg text-xs font-semibold transition ${
                            activeTab === "problems"
                                ? "bg-zinc-800 text-white shadow-sm border border-zinc-700"
                                : "text-zinc-400 hover:text-zinc-200"
                        }`}
                    >
                        Problems ({problems.length})
                    </button>
                    <button
                        onClick={() => setActiveTab("leaderboard")}
                        className={`px-4 py-1.5 rounded-lg text-xs font-semibold transition ${
                            activeTab === "leaderboard"
                                ? "bg-zinc-800 text-white shadow-sm border border-zinc-700"
                                : "text-zinc-400 hover:text-zinc-200"
                        }`}
                    >
                        Leaderboard ({leaderboard.length})
                    </button>
                </div>

                {activeTab === "problems" && (
                    <div className="bg-zinc-950 border border-zinc-900 rounded-2xl overflow-hidden shadow-lg">
                        {problemsLoading ? (
                            <div className="p-16 text-center text-zinc-500 space-y-3">
                                <div className="w-7 h-7 border-2 border-zinc-400 border-t-transparent rounded-full animate-spin mx-auto" />
                                <p className="text-sm text-zinc-400">Loading problem set...</p>
                            </div>
                        ) : contest.author_id !== user?.id && timeLeft.status === "upcoming" ? (
                            <div className="p-16 text-center space-y-4">
                                <div className="text-4xl">🔒</div>
                                <div>
                                    <h3 className="text-lg font-bold text-white">Problem Set Locked</h3>
                                    <p className="text-sm text-zinc-400 max-w-md mx-auto mt-1 leading-relaxed">
                                        The problems for this contest will unlock automatically when the round begins at{" "}
                                        <span className="text-zinc-200 font-semibold">{formatUtcToLocal(contest.start_time)}</span>.
                                    </p>
                                </div>
                                {!contest.registration_id && (
                                    <button
                                        onClick={handleRegisterToggle}
                                        disabled={registering}
                                        className="px-5 py-2.5 bg-zinc-100 hover:bg-white text-zinc-950 text-xs font-semibold rounded-xl transition shadow-sm"
                                    >
                                        {registering ? "Registering..." : "Register"}
                                    </button>
                                )}
                            </div>
                        ) : contest.author_id !== user?.id && !contest.registration_id ? (
                            timeLeft.status === "ended" ? (
                                <div className="p-16 text-center space-y-4">
                                    <div className="text-4xl">🏁</div>
                                    <div>
                                        <h3 className="text-lg font-bold text-white">Contest Ended</h3>
                                        <p className="text-sm text-zinc-400 max-w-md mx-auto mt-1 leading-relaxed">
                                            This contest has ended.
                                        </p>
                                    </div>
                                    <button
                                        onClick={() => setActiveTab("leaderboard")}
                                        className="px-5 py-2 bg-zinc-900 hover:bg-zinc-800 text-zinc-200 text-xs font-semibold rounded-xl border border-zinc-800 transition"
                                    >
                                        View Leaderboard
                                    </button>
                                </div>
                            ) : (
                                <div className="p-16 text-center space-y-4">
                                    <div className="text-4xl">🛡️</div>
                                    <div>
                                        <h3 className="text-lg font-bold text-white">Registration Required</h3>
                                        <p className="text-sm text-zinc-400 max-w-md mx-auto mt-1 leading-relaxed">
                                            Register for this contest to view and solve the problems.
                                        </p>
                                    </div>
                                    <button
                                        onClick={handleRegisterToggle}
                                        disabled={registering}
                                        className="px-5 py-2.5 bg-zinc-100 hover:bg-white text-zinc-950 text-xs font-semibold rounded-xl transition shadow-sm"
                                    >
                                        {registering ? "Registering..." : "Register"}
                                    </button>
                                </div>
                            )
                        ) : problems.length === 0 ? (
                            <div className="p-16 text-center text-zinc-500">
                                <p className="text-base font-semibold text-zinc-400">No problems available</p>
                                <p className="text-sm text-zinc-500 mt-1">
                                    {problemsError || "No problems have been added to this contest."}
                                </p>
                            </div>
                        ) : (
                            <div className="divide-y divide-zinc-900">
                                {problems.map((p, idx) => {
                                    const label = p.label || String.fromCharCode(65 + idx);
                                    return (
                                        <div
                                            key={p.id}
                                            onClick={() => navigate(`/problems/${p.id}?contest_id=${contest.id}`)}
                                            className="p-4 hover:bg-zinc-900/60 transition flex items-center justify-between cursor-pointer group"
                                        >
                                            <div className="flex items-center gap-3.5">
                                                <span className="w-8 h-8 rounded-lg bg-zinc-900 border border-zinc-800 flex items-center justify-center font-bold text-zinc-200 text-xs">
                                                    {label}
                                                </span>
                                                <div>
                                                    <h3 className="font-medium text-zinc-100 group-hover:text-white transition text-sm">
                                                        {p.title}
                                                    </h3>
                                                    <span className="text-xs font-mono text-zinc-500">{p.slug}</span>
                                                </div>
                                            </div>

                                            <div className="flex items-center gap-4">
                                                <span
                                                    className={`px-2.5 py-0.5 rounded-md text-xs border ${
                                                        p.difficulty?.toLowerCase() === "easy"
                                                            ? "bg-zinc-900 text-zinc-300 border-zinc-800"
                                                            : p.difficulty?.toLowerCase() === "medium"
                                                            ? "bg-zinc-800 text-zinc-200 border-zinc-700 font-medium"
                                                            : "bg-zinc-700 text-white border-zinc-600 font-semibold"
                                                    }`}
                                                >
                                                    {p.difficulty || "Unrated"}
                                                </span>
                                                <span className="text-zinc-600 group-hover:text-zinc-300 transition">&rarr;</span>
                                            </div>
                                        </div>
                                    );
                                })}
                            </div>
                        )}
                    </div>
                )}

                {activeTab === "leaderboard" && (
                    <div className="bg-zinc-950 border border-zinc-900 rounded-2xl overflow-hidden shadow-lg">
                        {leaderboard.length === 0 ? (
                            <div className="p-16 text-center text-zinc-500">
                                <p className="text-base font-semibold text-zinc-400">No submissions recorded</p>
                            </div>
                        ) : (
                            <div className="overflow-x-auto">
                                <table className="w-full border-collapse text-sm text-left">
                                    <thead className="bg-zinc-900/80 text-zinc-400 uppercase text-xs tracking-wider border-b border-zinc-800">
                                        <tr>
                                            <th className="p-3.5 text-center w-14">Rank</th>
                                            <th className="p-3.5">Participant</th>
                                            <th className="p-3.5 text-center">Solved</th>
                                            <th className="p-3.5 text-center">Penalty</th>
                                            {problems.map((p, idx) => (
                                                <th key={p.id} className="p-3.5 text-center font-bold text-zinc-300">
                                                    {p.label || String.fromCharCode(65 + idx)}
                                                </th>
                                            ))}
                                        </tr>
                                    </thead>
                                    <tbody className="divide-y divide-zinc-900">
                                        {leaderboard.map((entry, index) => {
                                            const isMe = entry.user_id === user?.id;
                                            return (
                                                <tr
                                                    key={entry.user_id}
                                                    className={`transition ${isMe ? "bg-zinc-900/80 font-medium" : "hover:bg-zinc-900/40"}`}
                                                >
                                                    <td className="p-3.5 text-center font-mono font-bold text-zinc-400 text-xs">
                                                        {index + 1}
                                                    </td>
                                                    <td className="p-3.5">
                                                        <Link
                                                            to={`/profile/${entry.username}`}
                                                            className="text-zinc-200 hover:text-white transition font-medium text-xs"
                                                        >
                                                            {entry.username} {isMe && "(You)"}
                                                        </Link>
                                                    </td>
                                                    <td className="p-3.5 text-center">
                                                        <span className="px-2.5 py-0.5 bg-zinc-900 text-zinc-100 border border-zinc-700 rounded-md font-bold text-xs">
                                                            {entry.solved}
                                                        </span>
                                                    </td>
                                                    <td className="p-3.5 text-center font-mono text-zinc-400 text-xs">
                                                        {Math.floor(entry.penalty / 60)}m {entry.penalty % 60}s
                                                    </td>
                                                    {problems.map((p) => {
                                                        const status = entry.problems[p.id];
                                                        return (
                                                            <td key={p.id} className="p-3.5 text-center font-mono text-xs">
                                                                {status ? (
                                                                    status.solved ? (
                                                                        <div className="text-zinc-100 font-bold bg-zinc-800 border border-zinc-700 rounded py-1">
                                                                            +{status.attempts}
                                                                            {status.time !== null && (
                                                                                <span className="block text-[10px] text-zinc-400 font-normal">
                                                                                    {Math.floor(status.time / 60)}m
                                                                                </span>
                                                                            )}
                                                                        </div>
                                                                    ) : (
                                                                        <div className="text-zinc-400 bg-zinc-900 border border-zinc-800 rounded py-1">
                                                                            -{status.attempts}
                                                                        </div>
                                                                    )
                                                                ) : (
                                                                    <span className="text-zinc-600">-</span>
                                                                )}
                                                            </td>
                                                        );
                                                    })}
                                                </tr>
                                            );
                                        })}
                                    </tbody>
                                </table>
                            </div>
                        )}
                    </div>
                )}
            </div>
        </div>
    );
}

