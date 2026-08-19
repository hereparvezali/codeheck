import { useAuth } from "../utils/contexts/authcontext";
import { ViewProblems } from "../components/view_problems";
import { Pagination } from "../components/pagination";
import { useEffect, useState } from "react";

export interface Problem {
    id: number;
    slug: string;
    title: string;
    difficulty?: string;
    is_public: boolean;
    created_at: string;
    status?: string;
    author_id?: number;
}

export interface ProblemsResponse {
    cursor?: number;
    problems: Problem[];
}

type NumberOrUndefined = number | undefined;

export default function Problems() {
    const { authfetch, user } = useAuth();

    const [loading, setLoading] = useState(false);
    const [problems, setProblems] = useState<Problem[]>([]);
    const [page, setPage] = useState(1);
    const [searchQuery, setSearchQuery] = useState("");
    const [selectedDifficulty, setSelectedDifficulty] = useState<string>("all");

    const [cursor, setCursor] = useState<number | undefined>();
    const [cursors, setCursors] = useState<NumberOrUndefined[]>([]);

    const limit = 10;

    const fetchProblems = async (c?: number, diff?: string, search?: string) => {
        setLoading(true);
        try {
            const params = new URLSearchParams({ limit: limit.toString() });
            if (c) params.append("cursor", c.toString());
            if (user?.id) params.append("user_id", user.id.toString());
            if (diff && diff !== "all") params.append("difficulty", diff);
            if (search && search.trim()) params.append("search", search.trim());

            const res = await authfetch(`/problems?${params.toString()}`);
            if (!res.ok) {
                const text = await res.text();
                throw new Error(text || "Failed to load problems");
            }
            const data: ProblemsResponse = await res.json();
            setProblems(data.problems || []);
            setCursor(data.cursor);
        } catch {

        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        setPage(1);
        setCursors([]);
        fetchProblems(undefined, selectedDifficulty, searchQuery);
    }, [selectedDifficulty, user]);

    const handleSearchSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        setPage(1);
        setCursors([]);
        fetchProblems(undefined, selectedDifficulty, searchQuery);
    };

    const goNext = () => {
        if (!cursor) return;
        setPage((p) => p + 1);
        setCursors((prev) => [...prev, cursor]);
        fetchProblems(cursor, selectedDifficulty, searchQuery);
    };

    const goPrev = () => {
        if (cursors.length === 0) return;
        const prevCursor = cursors[cursors.length - 2];
        setPage((p) => p - 1);
        fetchProblems(prevCursor, selectedDifficulty, searchQuery);
        setCursors((prev) => prev.slice(0, -1));
    };

    return (
        <div className="min-h-screen bg-black text-zinc-100 py-10 px-4 sm:px-6 lg:px-8">
            <div className="max-w-6xl mx-auto space-y-6">
                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
                    <div>
                        <h1 className="text-2xl sm:text-3xl font-extrabold text-white">Problems</h1>
                    </div>

                    <Pagination
                        page={page}
                        cursor={cursor}
                        loading={loading}
                        goPrev={goPrev}
                        goNext={goNext}
                    />
                </div>

                <div className="flex flex-col sm:flex-row justify-between items-stretch sm:items-center gap-4 p-3.5 bg-zinc-950 border border-zinc-900 rounded-2xl">
                    <form onSubmit={handleSearchSubmit} className="flex-1 flex gap-2">
                        <input
                            type="text"
                            placeholder="Search problems..."
                            value={searchQuery}
                            onChange={(e) => setSearchQuery(e.target.value)}
                            className="flex-1 px-4 py-2 bg-zinc-900 border border-zinc-800 rounded-xl text-sm text-white placeholder-zinc-500 focus:border-zinc-500 outline-none transition"
                        />
                        <button
                            type="submit"
                            className="px-5 py-2 bg-zinc-100 hover:bg-white text-zinc-950 font-semibold text-sm rounded-xl transition shadow-sm"
                        >
                            Search
                        </button>
                    </form>

                    <div className="flex items-center gap-1 p-1 bg-zinc-900 border border-zinc-800 rounded-xl">
                        {(["all", "Easy", "Medium", "Hard"] as const).map((diff) => (
                            <button
                                key={diff}
                                onClick={() => setSelectedDifficulty(diff)}
                                className={`px-3 py-1.5 rounded-lg text-xs font-semibold capitalize transition ${
                                    selectedDifficulty === diff
                                        ? "bg-zinc-800 text-white shadow-sm border border-zinc-700"
                                        : "text-zinc-400 hover:text-zinc-200"
                                }`}
                            >
                                {diff}
                            </button>
                        ))}
                    </div>
                </div>

                <div className="bg-zinc-950 border border-zinc-900 rounded-2xl overflow-hidden shadow-lg">
                    {loading ? (
                        <div className="py-20 text-center text-zinc-500 space-y-3">
                            <div className="w-7 h-7 border-2 border-zinc-400 border-t-transparent rounded-full animate-spin mx-auto" />
                            <p className="text-sm">Loading problems...</p>
                        </div>
                    ) : problems.length === 0 ? (
                        <div className="py-16 text-center text-zinc-500">
                            <p className="text-sm font-semibold text-zinc-400">No problems found</p>
                        </div>
                    ) : (
                        <ViewProblems
                            problems={problems}
                            id={true}
                            slug={true}
                            status={true}
                            title={true}
                            difficulty={true}
                            is_public={false}
                        />
                    )}
                </div>
            </div>
        </div>
    );
}

