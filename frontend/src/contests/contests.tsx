import { useEffect, useState } from "react";
import { useAuth } from "../utils/contexts/authcontext";
import { useNavigate } from "react-router-dom";
import { Pagination } from "../components/pagination";
import { ViewContests } from "../components/view_contests";
import { parseUtcDate } from "../utils/helpers";

export interface Contest {
    id: number;
    title: string;
    slug: string;
    description?: string;
    start_time: string;
    end_time: string;
    is_public: boolean;
    author_id?: number;
    registration_id?: number;
    registered_at?: string;
}

export interface ContestResponseWithCursor {
    cursor?: number;
    contests: Contest[];
}

type NumberOrUndefined = number | undefined;

export default function Contests() {
    const { authfetch, user } = useAuth();
    const navigate = useNavigate();

    const [contests, setContests] = useState<Contest[]>([]);
    const [cursor, setCursor] = useState<number | undefined>(undefined);
    const [cursors, setCursors] = useState<NumberOrUndefined[]>([]);
    const [page, setPage] = useState(1);
    const [loading, setLoading] = useState(false);
    const [filter, setFilter] = useState<"all" | "upcoming" | "ongoing" | "past">("all");

    const limit = 6;

    const fetchContests = async (c?: number) => {
        setLoading(true);
        const params = new URLSearchParams({ limit: limit.toString() });
        if (c) params.append("cursor", c.toString());

        try {
            const res = await authfetch(`/contests?${params.toString()}`);
            if (!res.ok) {
                const text = await res.text().catch(() => res.statusText);
                throw new Error(text || "Failed to load contests");
            }

            const data: ContestResponseWithCursor = await res.json();
            setContests(data.contests || []);
            setCursor(data.cursor);
        } catch (e) {
            console.error("Failed to fetch contests:", e);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchContests(undefined);
    }, [user]);

    const goNext = () => {
        if (!cursor) return;
        setPage((p) => p + 1);
        setCursors((prev) => [...prev, cursor]);
        fetchContests(cursor);
    };

    const goPrev = () => {
        if (page <= 1) return;
        const prevCursor = cursors[cursors.length - 2];
        setPage((p) => p - 1);
        fetchContests(prevCursor);
        setCursors((prev) => prev.slice(0, -1));
    };

    const handleRegister = async (contest_id: number) => {
        if (!user) {
            navigate("/signin");
            return;
        }

        try {
            const res = await authfetch("/contest/registration", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ contest_id }),
            });
            if (res.ok) {
                fetchContests(cursors[cursors.length - 1]);
            }
        } catch {

        }
    };

    const handleUnRegister = async (registration_id?: number) => {
        if (!registration_id) return;
        try {
            const res = await authfetch(`/contest/registration?id=${registration_id}`, {
                method: "DELETE",
            });
            if (res.ok) {
                fetchContests(cursors[cursors.length - 1]);
            }
        } catch {

        }
    };

    const filteredContests = contests.filter((c) => {
        const now = new Date();
        const start = parseUtcDate(c.start_time);
        const end = parseUtcDate(c.end_time);

        if (filter === "upcoming") return now < start;
        if (filter === "ongoing") return now >= start && now <= end;
        if (filter === "past") return now > end;
        return true;
    });

    return (
        <div className="min-h-screen bg-black text-zinc-100 py-10 px-4 sm:px-6 lg:px-8">
            <div className="max-w-6xl mx-auto space-y-6">
                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
                    <div>
                        <h1 className="text-2xl sm:text-3xl font-extrabold text-white">Contests</h1>
                    </div>

                    <Pagination
                        page={page}
                        cursor={cursor}
                        loading={loading}
                        goNext={goNext}
                        goPrev={goPrev}
                    />
                </div>

                <div className="flex items-center gap-1.5 p-1 bg-zinc-950 border border-zinc-900 rounded-xl w-fit">
                    {(["all", "ongoing", "upcoming", "past"] as const).map((f) => (
                        <button
                            key={f}
                            onClick={() => setFilter(f)}
                            className={`px-3.5 py-1.5 rounded-lg text-xs font-semibold capitalize transition ${
                                filter === f
                                    ? "bg-zinc-800 text-white shadow-sm border border-zinc-700"
                                    : "text-zinc-400 hover:text-zinc-200"
                            }`}
                        >
                            {f === "all" ? "All" : f}
                        </button>
                    ))}
                </div>

                {loading ? (
                    <div className="py-20 text-center text-zinc-500 space-y-3">
                        <div className="w-7 h-7 border-2 border-zinc-400 border-t-transparent rounded-full animate-spin mx-auto" />
                        <p className="text-sm">Loading contests...</p>
                    </div>
                ) : (
                    <ViewContests
                        contests={filteredContests}
                        handleRegister={handleRegister}
                        handleUnRegister={handleUnRegister}
                        id={true}
                        title={true}
                        description={true}
                        start_time={true}
                        end_time={true}
                        edit={false}
                    />
                )}
            </div>
        </div>
    );
}

