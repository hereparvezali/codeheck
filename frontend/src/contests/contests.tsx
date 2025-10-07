import { useEffect, useState } from "react";
import { useAuth } from "../utils/contexts/authcontext";
import { useNavigate } from "react-router-dom";
import { Pagination } from "../components/pagination";
import { ViewContests } from "../components/view_contests";

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
    const { authfetch } = useAuth();
    const navigator = useNavigate();

    const [contests, setContests] = useState<Contest[]>([]);
    const [cursor, setCursor] = useState<number | undefined>(undefined);
    const [cursors, setCursors] = useState<NumberOrUndefined[]>([]);
    const [page, setPage] = useState(1);
    const limit = 4;

    const fetchContests = async (cursor?: number) => {
        const params = new URLSearchParams({ limit: limit.toString() });
        if (cursor) params.append("cursor", cursor.toString());

        try {
            const res = await authfetch(`/contests?${params.toString()}`, {
                method: "GET",
            });

            if (!res.ok) {
                if (res.status === 401) {
                    navigator("/signin");
                    return;
                }
                const text = await res.text().catch(() => res.statusText);
                throw new Error(text || "Failed to load contests");
            }

            const data: ContestResponseWithCursor = await res.json();
            setContests(data.contests || []);
            setCursor(data.cursor);

            // eslint-disable-next-line @typescript-eslint/no-explicit-any
        } catch (e: any) {
            if (e.name === "AbortError") return;
            console.error("Failed to fetch contests:", e);
        }
    };
    useEffect(() => {
        fetchContests(undefined);
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

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
    const handleRegister = (contest_id: number) => {
        authfetch(`/contest/registration?contest_id=${contest_id}`, {
            method: "POST",
        })
            .then(async (res) => {
                if (!res.ok) {
                    if (res.status === 401) {
                        navigator("/signin");
                    }
                    const err = await res.text();
                    throw new Error(err);
                }
            })
            .catch((err) => {
                alert(err.toString());
            });
    };
    const handleUnRegister = (registration_id?: number) => {
        if (!registration_id) return;
        authfetch(`/contest/registration?registration_id=${registration_id}`, {
            method: "DELETE",
        }).then(async (res) => {
            console.log(res.statusText);
        });
    };
    return (
        <div className="max-w-6xl mx-auto p-4">
            <div className="flex justify-between items-center mb-4">
                <h1 className="text-2xl font-bold">Contests</h1>
                <Pagination
                    page={page}
                    cursor={cursor}
                    goNext={goNext}
                    goPrev={goPrev}
                />
            </div>

            <ViewContests
                contests={contests}
                handleRegister={handleRegister}
                handleUnRegister={handleUnRegister}
                id={true}
                title={true}
                description={true}
                start_time={true}
                end_time={true}
                edit={false}
            />
        </div>
    );
}
