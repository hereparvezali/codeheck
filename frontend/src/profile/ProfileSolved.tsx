import { useEffect, useState } from "react";
import { useAuth } from "../utils/contexts/authcontext";
import { ViewProblems } from "../components/view_problems";
import type { Problem } from "../problems/problems";
import { Link } from "react-router-dom";

interface SolvedProblemsResponse {
    cursor?: number;
    problems: Problem[];
}

interface ProfileSolvedProps {
    userId: number;
}

export default function ProfileSolved({ userId }: ProfileSolvedProps) {
    const { authfetch } = useAuth();

    const [solvedProblems, setSolvedProblems] = useState<Problem[]>([]);
    const [loading, setLoading] = useState(false);
    const [cursor, setCursor] = useState<number | undefined>(undefined);
    const [page, setPage] = useState(1);
    const [cursors, setCursors] = useState<(number | undefined)[]>([]);

    const fetchSolvedProblems = async (c?: number) => {
        setLoading(true);
        try {
            const params = new URLSearchParams({ limit: "15", user_id: userId.toString(), status: "AC" });
            if (c) params.append("cursor", c.toString());

            const res = await authfetch(`/problems?${params.toString()}`);
            if (res.ok) {
                const data: SolvedProblemsResponse = await res.json();
                setSolvedProblems(data.problems || []);
                setCursor(data.cursor);
            }
        } catch (e) {
            console.error("Failed to fetch solved problems:", e);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchSolvedProblems(undefined);

    }, [userId]);

    const goNext = () => {
        if (!cursor) return;
        setPage((p) => p + 1);
        setCursors((prev) => [...prev, cursor]);
        fetchSolvedProblems(cursor);
    };

    const goPrev = () => {
        if (cursors.length === 0) return;
        const prevCursor = cursors[cursors.length - 2];
        setPage((p) => p - 1);
        fetchSolvedProblems(prevCursor);
        setCursors((prev) => prev.slice(0, -1));
    };

    return (
        <div className="space-y-4">
            <div className="flex justify-between items-center mb-2">
                <h3 className="text-sm font-bold text-white">Solved Problems</h3>
                {solvedProblems.length > 0 && (
                    <div className="flex items-center gap-2">
                        <button
                            onClick={goPrev}
                            disabled={page === 1 || loading}
                            className="px-3 py-1 bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 text-zinc-300 rounded-lg text-xs font-semibold disabled:opacity-40 transition"
                        >
                            &larr; Prev
                        </button>
                        <span className="text-xs text-zinc-500 font-mono">Page {page}</span>
                        <button
                            onClick={goNext}
                            disabled={!cursor || loading}
                            className="px-3 py-1 bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 text-zinc-300 rounded-lg text-xs font-semibold disabled:opacity-40 transition"
                        >
                            Next &rarr;
                        </button>
                    </div>
                )}
            </div>

            {loading ? (
                <div className="py-16 text-center text-zinc-500 space-y-3">
                    <div className="w-7 h-7 border-2 border-zinc-400 border-t-transparent rounded-full animate-spin mx-auto" />
                    <p className="text-xs">Loading solved problems...</p>
                </div>
            ) : solvedProblems.length === 0 ? (
                <div className="p-12 text-center bg-zinc-950 border border-zinc-900 rounded-2xl">
                    <p className="text-sm font-semibold text-zinc-300 mb-1">No solved problems yet</p>
                    <p className="text-xs text-zinc-500 mb-4">Start practicing in the problemset to earn Accepted verdicts.</p>
                    <Link
                        to="/problems"
                        className="px-4 py-2 bg-zinc-100 hover:bg-white text-zinc-950 rounded-xl text-xs font-semibold inline-block transition shadow-sm"
                    >
                        Start Solving
                    </Link>
                </div>
            ) : (
                <div className="bg-zinc-950 border border-zinc-900 rounded-2xl overflow-hidden shadow-lg">
                    <ViewProblems
                        problems={solvedProblems}
                        id={true}
                        title={true}
                        slug={true}
                        difficulty={true}
                        status={true}
                    />
                </div>
            )}
        </div>
    );
}
