import { useEffect, useState } from "react";
import { useAuth } from "../utils/contexts/authcontext";
import { ViewSubmissions, type Submission } from "../components/view_submissions";
import { Link } from "react-router-dom";

interface SubmissionsResponse {
    cursor?: number;
    submissions: Submission[];
}

interface ProfileSubmissionsProps {
    userId: number;
}

export default function ProfileSubmissions({ userId }: ProfileSubmissionsProps) {
    const { authfetch } = useAuth();

    const [submissions, setSubmissions] = useState<Submission[]>([]);
    const [loading, setLoading] = useState(false);
    const [cursor, setCursor] = useState<number | undefined>(undefined);
    const [page, setPage] = useState(1);
    const [cursors, setCursors] = useState<(number | undefined)[]>([]);

    const fetchSubmissions = async (c?: number) => {
        setLoading(true);
        try {
            const params = new URLSearchParams({ limit: "15", user_id: userId.toString() });
            if (c) params.append("cursor", c.toString());

            const res = await authfetch(`/submissions?${params.toString()}`);
            if (res.ok) {
                const data: SubmissionsResponse = await res.json();
                setSubmissions(data.submissions || []);
                setCursor(data.cursor);
            }
        } catch (e) {
            console.error("Failed to fetch submissions:", e);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchSubmissions(undefined);

    }, [userId]);

    const goNext = () => {
        if (!cursor) return;
        setPage((p) => p + 1);
        setCursors((prev) => [...prev, cursor]);
        fetchSubmissions(cursor);
    };

    const goPrev = () => {
        if (cursors.length === 0) return;
        const prevCursor = cursors[cursors.length - 2];
        setPage((p) => p - 1);
        fetchSubmissions(prevCursor);
        setCursors((prev) => prev.slice(0, -1));
    };

    return (
        <div className="space-y-4">
            <div className="flex justify-between items-center mb-2">
                <h3 className="text-sm font-bold text-white">Submissions History</h3>
                {submissions.length > 0 && (
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
                    <p className="text-xs">Loading submission history...</p>
                </div>
            ) : submissions.length === 0 ? (
                <div className="p-12 text-center bg-zinc-950 border border-zinc-900 rounded-2xl">
                    <p className="text-sm font-semibold text-zinc-300 mb-1">No submissions yet</p>
                    <p className="text-xs text-zinc-500 mb-4">Your submitted solutions will be tracked here with execution metrics.</p>
                    <Link
                        to="/problems"
                        className="px-4 py-2 bg-zinc-100 hover:bg-white text-zinc-950 rounded-xl text-xs font-semibold inline-block transition shadow-sm"
                    >
                        Browse Problemset
                    </Link>
                </div>
            ) : (
                <div className="bg-zinc-950 border border-zinc-900 rounded-2xl overflow-hidden shadow-lg">
                    <ViewSubmissions
                        submissions={submissions}
                        status={true}
                        id={true}
                        problem_id={true}
                        contest_id={true}
                        language={true}
                        verdict={true}
                        time={true}
                        memory={true}
                        view_code={true}
                    />
                </div>
            )}
        </div>
    );
}
