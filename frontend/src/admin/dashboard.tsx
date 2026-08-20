import { useEffect, useState } from "react";
import { ViewProblems } from "../components/view_problems";
import { ViewContests } from "../components/view_contests";
import type { Problem, ProblemsResponse } from "../problems/problems";
import type { Contest, ContestResponseWithCursor } from "../contests/contests";
import { useAuth } from "../utils/contexts/authcontext";
import { useNavigate, Link } from "react-router-dom";
import { Pagination } from "../components/pagination";

type NumberOrUndefined = number | undefined;

export default function Dashboard() {
    const navigate = useNavigate();
    const [activeTab, setActiveTab] = useState<"problems" | "contests">("problems");
    const [problems, setProblems] = useState<Problem[]>([]);
    const [contests, setContests] = useState<Contest[]>([]);
    const [ploading, setPLoading] = useState(true);
    const [cloading, setCLoading] = useState(true);
    const [ppage, setPPage] = useState(1);
    const [cpage, setCPage] = useState(1);
    const [pcursor, setPCursor] = useState<NumberOrUndefined>(undefined);
    const [ccursor, setCCursor] = useState<NumberOrUndefined>(undefined);
    const [pcursors, setPCursors] = useState<NumberOrUndefined[]>([]);
    const [ccursors, setCCursors] = useState<NumberOrUndefined[]>([]);
    const { authfetch, user } = useAuth();
    const limit = 8;

    const fetchProblems = (pc?: number) => {
        if (!user?.id) return;
        setPLoading(true);
        const params = new URLSearchParams({ limit: limit.toString(), author_id: user.id.toString() });
        if (pc) params.append("cursor", pc.toString());

        authfetch(`/problems?${params.toString()}`)
            .then(async (res) => {
                if (!res.ok) {
                    if (res.status === 401) navigate("/signin");
                    throw new Error(await res.text());
                }
                return res.json();
            })
            .then((data: ProblemsResponse) => {
                setPCursor(data.cursor);
                setProblems(data.problems || []);
            })
            .catch(console.error)
            .finally(() => setPLoading(false));
    };

    const fetchContests = (cc?: number) => {
        if (!user?.id) return;
        setCLoading(true);
        const params = new URLSearchParams({ limit: limit.toString(), author_id: user.id.toString() });
        if (cc) params.append("cursor", cc.toString());

        authfetch(`/contests?${params.toString()}`)
            .then(async (res) => {
                if (!res.ok) {
                    if (res.status === 401) navigate("/signin");
                    throw new Error(await res.text());
                }
                return res.json();
            })
            .then((data: ContestResponseWithCursor) => {
                setCCursor(data.cursor);
                setContests(data.contests || []);
            })
            .catch(console.error)
            .finally(() => setCLoading(false));
    };

    useEffect(() => {
        fetchProblems(undefined);
        fetchContests(undefined);
    }, [user]);

    const pgoNext = () => {
        if (!pcursor) return;
        setPPage((p) => p + 1);
        setPCursors((prev) => [...prev, pcursor]);
        fetchProblems(pcursor);
    };

    const pgoPrev = () => {
        if (pcursors.length === 0) return;
        const prevCursor = pcursors[pcursors.length - 2];
        setPPage((p) => p - 1);
        fetchProblems(prevCursor);
        setPCursors((prev) => prev.slice(0, -1));
    };

    const cgoNext = () => {
        if (!ccursor) return;
        setCPage((p) => p + 1);
        setCCursors((prev) => [...prev, ccursor]);
        fetchContests(ccursor);
    };

    const cgoPrev = () => {
        if (ccursors.length === 0) return;
        const prevCursor = ccursors[ccursors.length - 2];
        setCPage((p) => p - 1);
        fetchContests(prevCursor);
        setCCursors((prev) => prev.slice(0, -1));
    };

    return (
        <div className="min-h-screen bg-black text-zinc-100 py-10 px-4 sm:px-6 lg:px-8">
            <div className="max-w-6xl mx-auto space-y-6">
                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
                    <div>
                        <h1 className="text-2xl sm:text-3xl font-extrabold text-white">Admin Dashboard</h1>
                    </div>

                    <div className="flex gap-3">
                        <Link
                            to="/admin/create_problem"
                            className="px-4 py-2 bg-zinc-100 hover:bg-white text-zinc-950 text-xs font-semibold rounded-xl shadow-sm transition"
                        >
                            + Create Problem
                        </Link>
                        <Link
                            to="/admin/create_contest"
                            className="px-4 py-2 bg-zinc-900 hover:bg-zinc-800 text-zinc-200 border border-zinc-800 text-xs font-semibold rounded-xl transition"
                        >
                            + Create Contest
                        </Link>
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
                        onClick={() => setActiveTab("contests")}
                        className={`px-4 py-1.5 rounded-lg text-xs font-semibold transition ${
                            activeTab === "contests"
                                ? "bg-zinc-800 text-white shadow-sm border border-zinc-700"
                                : "text-zinc-400 hover:text-zinc-200"
                        }`}
                    >
                        Contests ({contests.length})
                    </button>
                </div>

                {activeTab === "problems" && (
                    <div className="bg-zinc-950 border border-zinc-900 rounded-2xl overflow-hidden shadow-lg p-4 space-y-4">
                        <div className="flex justify-between items-center px-2">
                            <span className="text-xs text-zinc-500 font-medium">Authored Problems</span>
                            <Pagination
                                cursor={pcursor}
                                page={ppage}
                                loading={ploading}
                                goNext={pgoNext}
                                goPrev={pgoPrev}
                            />
                        </div>

                        {ploading ? (
                            <div className="py-16 text-center text-zinc-500">Loading problems...</div>
                        ) : problems.length === 0 ? (
                            <div className="py-16 text-center text-zinc-500">
                                <p className="text-sm font-semibold text-zinc-400">No problems authored</p>
                                <Link
                                    to="/admin/create_problem"
                                    className="mt-3 px-4 py-2 bg-zinc-100 text-zinc-950 text-xs rounded-xl font-semibold inline-block"
                                >
                                    Create Problem
                                </Link>
                            </div>
                        ) : (
                            <ViewProblems problems={problems} is_public={true} edit={true} />
                        )}
                    </div>
                )}

                {activeTab === "contests" && (
                    <div className="bg-zinc-950 border border-zinc-900 rounded-2xl overflow-hidden shadow-lg p-4 space-y-4">
                        <div className="flex justify-between items-center px-2">
                            <span className="text-xs text-zinc-500 font-medium">Authored Contests</span>
                            <Pagination
                                cursor={ccursor}
                                page={cpage}
                                loading={cloading}
                                goNext={cgoNext}
                                goPrev={cgoPrev}
                            />
                        </div>

                        {cloading ? (
                            <div className="py-16 text-center text-zinc-500">Loading contests...</div>
                        ) : contests.length === 0 ? (
                            <div className="py-16 text-center text-zinc-500">
                                <p className="text-sm font-semibold text-zinc-400">No contests authored</p>
                                <Link
                                    to="/admin/create_contest"
                                    className="mt-3 px-4 py-2 bg-zinc-100 text-zinc-950 text-xs rounded-xl font-semibold inline-block"
                                >
                                    Create Contest
                                </Link>
                            </div>
                        ) : (
                            <ViewContests contests={contests} is_public={true} edit={true} />
                        )}
                    </div>
                )}
            </div>
        </div>
    );
}

