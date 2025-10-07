import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { useAuth } from "../utils/contexts/authcontext";
import { ViewProblems } from "../components/view_problems";
import type { Problem } from "../problems/problems";

interface SolvedProblemsResponse {
    cursor?: number;
    problems: Problem[];
    count?: number;
}

interface ProfileSolvedProps {
    onCountUpdate?: (count: number) => void;
}

const ProfileSolved = ({ onCountUpdate }: ProfileSolvedProps) => {
    const navigator = useNavigate();
    const { authfetch, user } = useAuth();

    const [solvedProblems, setSolvedProblems] = useState<Problem[]>([]);
    const [solvedLoading, setSolvedLoading] = useState(false);
    const [solvedCount, setSolvedCount] = useState(0);
    const [solvedCursor, setSolvedCursor] = useState<number | undefined>(undefined);
    const [solvedPage, setSolvedPage] = useState(1);
    const [solvedCursors, setSolvedCursors] = useState<(number | undefined)[]>([]);

    useEffect(() => {
        if (solvedProblems.length === 0) {
            fetchSolvedProblems();
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    const fetchSolvedProblems = async (cursor?: number) => {
        if (!user) return;

        setSolvedLoading(true);
        try {
            const params = new URLSearchParams({ limit: "20" });
            if (cursor) params.append("cursor", cursor.toString());

            const res = await authfetch(
                `/problems?user_id=${user.id}&status=AC&${params.toString()}`,
            );

            if (!res.ok) {
                if (res.status === 401) navigator("/signin");
                throw new Error(await res.text());
            }

            const data: SolvedProblemsResponse = await res.json();
            setSolvedProblems(data.problems || []);
            setSolvedCursor(data.cursor);
            if (data.count !== undefined) {
                setSolvedCount(data.count);
                if (onCountUpdate) {
                    onCountUpdate(data.count);
                }
            }
        } catch (e) {
            console.error("Failed to fetch solved problems:", e);
        } finally {
            setSolvedLoading(false);
        }
    };

    const goNextSolved = () => {
        if (!solvedCursor) return;
        setSolvedPage((p) => p + 1);
        setSolvedCursors((prev) => [...prev, solvedCursor]);
        fetchSolvedProblems(solvedCursor);
    };

    const goPrevSolved = () => {
        if (solvedCursors.length === 0) return;
        const prevCursor = solvedCursors[solvedCursors.length - 2];
        setSolvedPage((p) => p - 1);
        fetchSolvedProblems(prevCursor);
        setSolvedCursors((prev) => prev.slice(0, -1));
    };

    return (
        <div>
            <div className="flex items-center justify-between mb-4">
                <h2 className="text-2xl font-bold">Solved Problems</h2>
                <div className="flex items-center gap-4">
                    {solvedCount > 0 && (
                        <span className="text-sm text-gray-600">
                            Total: {solvedCount} problems
                        </span>
                    )}
                    {solvedProblems.length > 0 && (
                        <div className="flex gap-2">
                            <button
                                onClick={goPrevSolved}
                                disabled={solvedPage === 1 || solvedLoading}
                                className="px-3 py-1 border rounded hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed text-sm"
                            >
                                ← Previous
                            </button>
                            <span className="px-3 py-1 text-gray-600 text-sm">
                                Page {solvedPage}
                            </span>
                            <button
                                onClick={goNextSolved}
                                disabled={!solvedCursor || solvedLoading}
                                className="px-3 py-1 border rounded hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed text-sm"
                            >
                                Next →
                            </button>
                        </div>
                    )}
                </div>
            </div>

            {solvedLoading ? (
                <div className="text-center py-12">
                    <div className="animate-spin text-4xl mb-4">⏳</div>
                    <p className="text-gray-600">Loading solved problems...</p>
                </div>
            ) : solvedProblems.length === 0 ? (
                <div className="bg-gray-50 rounded-lg p-12 text-center">
                    <p className="text-gray-500 text-lg mb-4">
                        You haven't solved any problems yet
                    </p>
                    <button
                        onClick={() => navigator("/problems")}
                        className="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
                    >
                        Start Solving
                    </button>
                </div>
            ) : (
                <div className="space-y-3">
                    <ViewProblems
                        problems={solvedProblems}
                        id={true}
                        title={true}
                        slug={true}
                        status={true}
                    />
                </div>
            )}
        </div>
    );
};

export default ProfileSolved;