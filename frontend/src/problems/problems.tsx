import { useAuth } from "../utils/contexts/authcontext";
import { ViewProblems } from "../components/view_problems";
import { Pagination } from "../components/pagination";
import { useNavigate } from "react-router-dom";
import { useEffect, useState } from "react";

export interface Problem {
    id: number;
    slug: string;
    title: string;
    difficulty?: string;
    is_public: boolean;
    created_at: string;
}

export interface ProblemsResponse {
    cursor?: number;
    problems: Problem[];
}
type NumberOrUndefined = number | undefined;

const Problems = () => {
    const { authfetch } = useAuth();
    const navigator = useNavigate();

    const [loading, setLoading] = useState(false);
    const [problems, setProblems] = useState<Problem[]>([]);
    const [page, setPage] = useState(1);

    // For cursor-based navigation
    const [cursor, setCursor] = useState<number | undefined>();
    const [cursors, setCursors] = useState<NumberOrUndefined[]>([]);

    const limit = 4;

    const fetchProblems = async (cursor?: number) => {
        setLoading(true);
        try {
            const params = new URLSearchParams({ limit: limit.toString() });
            if (cursor) params.append("cursor", cursor.toString());

            const res = await authfetch(
                `/problems?${params.toString()}`,
                { method: "GET" },
            );

            if (!res.ok) {
                if (res.status === 401) navigator("/signin");
                const text = await res.text();
                throw new Error(text || "Failed to load problems");
            }
            const data: ProblemsResponse = await res.json();
            setProblems(data.problems || []);
            setCursor(data.cursor);
        } catch (e) {
            console.error(e);
        } finally {
            setLoading(false);
        }
    };

    // Initial load
    useEffect(() => {
        fetchProblems(undefined);
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    const goNext = () => {
        if (!cursor) return;
        setPage((p) => p + 1);
        setCursors((prev) => [...prev, cursor]);
        fetchProblems(cursor);
    };

    const goPrev = () => {
        if (cursors.length == 0) return;
        const prevCursor = cursors[cursors.length - 2];
        setPage((p) => p - 1);
        fetchProblems(prevCursor);
        setCursors((prev) => prev.slice(0, -1));
    };

    return (
        <div className="max-w-6xl mx-auto p-4">
            <div className="flex justify-between items-center mb-4">
                <h1 className="text-2xl font-bold">Problems</h1>
                <Pagination
                    page={page}
                    cursor={cursor}
                    loading={loading}
                    goPrev={goPrev}
                    goNext={goNext}
                />
            </div>
            <div>
                {loading && (
                    <p className="text-gray-500 text-center">
                        Loading problems...
                    </p>
                )}

                {!loading && problems.length === 0 && (
                    <p className="text-gray-500 text-center">
                        No problems available.
                    </p>
                )}
            </div>
            <div>
                <ViewProblems problems={problems} show_is_public={false} />
            </div>
        </div>
    );
};
export default Problems;
