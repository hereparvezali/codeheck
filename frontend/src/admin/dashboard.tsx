import { useEffect, useState } from "react";
import { ViewProblems } from "../components/view_problems";
import { ViewContests } from "../components/view_contests";
import type { Problem, ProblemsResponse } from "../problems/problems";
import type { Contest, ContestResponseWithCursor } from "../contests/contests";
import { useAuth } from "../utils/contexts/authcontext";
import { useNavigate } from "react-router-dom";
import { Pagination } from "../components/pagination";

type NumberOrUndefined = number | undefined;
export default function Dashboard() {
    const navigator = useNavigate();
    const [activeTab, setActiveTab] = useState<"problems" | "contests">(
        "problems",
    );
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
    const limit = 4;

    const fetchProblems = (pcursor?: number) => {
        if (!user) return;

        setPLoading(true);
        const params = new URLSearchParams({ limit: limit.toString() });
        if (pcursor) params.append("cursor", pcursor.toString());
        params.append("author_id", user.id.toString());

        authfetch(`/problems?${params.toString()}`, {
            method: "GET",
        })
            .then(async (res) => {
                if (!res.ok) {
                    if (res.status === 401) {
                        navigator("/signin");
                    }
                    throw new Error(await res.text());
                }
                return res.json();
            })
            .then((data: ProblemsResponse) => {
                setPCursor(data.cursor);
                setProblems(data.problems);
            })
            .catch((e) => console.log(e))
            .finally(() => {
                setPLoading(false);
            });
    };
    const fetchContests = (ccursor?: number) => {
        if(!user) return;
        setCLoading(true);
        const params = new URLSearchParams({ limit: limit.toString() });
        if (ccursor) params.append("cursor", ccursor.toString());
        params.append("author_id", user.id.toString());

        authfetch(`/contests?${params.toString()}`, {
            method: "GET",
        })
            .then(async (res) => {
                if (!res.ok) {
                    if (res.status === 401) {
                        navigator("/signin");
                    }
                    throw new Error(await res.text());
                }
                return res.json();
            })
            .then((data: ContestResponseWithCursor) => {
                setCCursor(data.cursor);
                setContests(data.contests);
            })
            .catch((e) => console.log(e))
            .finally(() => {
                setCLoading(false);
            });
    };

    useEffect(() => {
        fetchProblems(undefined);
        fetchContests(undefined);
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);
    const pgoNext = () => {
        if (!pcursor) return;
        setPPage((p) => p + 1);
        setPCursors((prev) => [...prev, pcursor]);
        fetchProblems(pcursor);
    };

    const pgoPrev = () => {
        if (pcursors.length == 0) return;
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
        if (ccursors.length == 0) return;
        const crevCursor = ccursors[ccursors.length - 2];
        setCPage((p) => p - 1);
        fetchContests(crevCursor);
        setCCursors((prev) => prev.slice(0, -1));
    };
    return (
        <div className="p-6">
            <h1 className="text-2xl font-bold mb-6">Admin Dashboard</h1>

            {/* Tabs */}
            <div className="flex gap-4 border-b mb-6">
                <button
                    onClick={() => setActiveTab("problems")}
                    className={`pb-2 px-4 ${
                        activeTab === "problems"
                            ? "border-b-2 border-blue-500 font-semibold text-blue-600"
                            : "text-gray-500"
                    }`}
                >
                    Problems
                </button>
                <button
                    onClick={() => setActiveTab("contests")}
                    className={`pb-2 px-4 ${
                        activeTab === "contests"
                            ? "border-b-2 border-blue-500 font-semibold text-blue-600"
                            : "text-gray-500"
                    }`}
                >
                    Contests
                </button>
            </div>

            {/* Problems Tab */}
            {activeTab === "problems" && (
                <div>
                    <div className="flex justify-between items-center pb-3">
                        <Pagination
                            cursor={pcursor}
                            page={ppage}
                            loading={ploading}
                            goNext={pgoNext}
                            goPrev={pgoPrev}
                        />
                        <button
                            className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
                            onClick={() => navigator("/admin/create_problem")}
                        >
                            + Create Problem
                        </button>
                    </div>
                    <ViewProblems problems={problems} show_is_public={true} />
                </div>
            )}

            {/* Contests Tab */}
            {activeTab === "contests" && (
                <div>
                    <div className="flex justify-between items-center pb-3">
                        <Pagination
                            cursor={ccursor}
                            page={cpage}
                            loading={cloading}
                            goNext={cgoNext}
                            goPrev={cgoPrev}
                        />
                        <button
                            className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
                            onClick={() => navigator("/admin/create_contest")}
                        >
                            + Create Contest
                        </button>
                    </div>
                    <ViewContests contests={contests} />
                </div>
            )}
        </div>
    );
}
