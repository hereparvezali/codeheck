import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { useAuth } from "../utils/contexts/authcontext";

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
}

interface ProblemResponse {
    id: number;
    title: string;
    slug: string;
    difficulty: string;
    label?: string;
}

const ContestDetail = () => {
    const navigate = useNavigate();
    const { id } = useParams<{ id: string }>();
    const { authfetch, user } = useAuth();

    const [contest, setContest] = useState<Contest | null>(null);
    const [problems, setProblems] = useState<ProblemResponse[]>([]);
    const [leaderboard, setLeaderboard] = useState<LeaderboardEntry[]>([]);
    const [loading, setLoading] = useState(true);
    const [problemsLoading, setProblemsLoading] = useState(true);
    const [leaderboardLoading, setLeaderboardLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    // Fetch contest
    useEffect(() => {
        if (!id) return;

        setLoading(true);
        authfetch(`/contest?id=${id}&user_id=${user?.id}`)
            .then(async (res) => {
                if (!res.ok) {
                    throw new Error(
                        `Failed to fetch contest: ${res.statusText}`,
                    );
                }
                return res.json() as Promise<Contest>;
            })
            .then(setContest)
            .catch((err) => setError(err.message))
            .finally(() => setLoading(false));
    }, [id, user, authfetch]);

    // Fetch problems
    useEffect(() => {
        if (!contest) return;

        setProblemsLoading(true);
        authfetch(`/contest/problems?id=${contest.id}`)
            .then(async (res) => {
                if (res.status === 401) {
                    navigate("/signin");
                    return [];
                }
                if (!res.ok) {
                    throw new Error(await res.text());
                }
                return res.json();
            })
            .then((data: ProblemResponse[]) => {
                // console.log(data);
                setProblems(data);
            })
            .catch((err) => console.error(err))
            .finally(() => setProblemsLoading(false));
    }, [contest, authfetch, navigate, user]);

    // Fetch leaderboard
    useEffect(() => {
        if (!contest) return;

        setLeaderboardLoading(true);
        authfetch(`/contest/leaderboard?contest_id=${contest.id}`)
            .then(async (res) => {
                if (res.status === 401) {
                    navigate("/signin");
                    return [];
                }
                if (!res.ok) {
                    throw new Error(await res.text());
                }
                return res.json();
            })
            .then((data: { standings: LeaderboardEntry[] }) => {
                setLeaderboard(data.standings);
            })
            .catch((err) => console.error(err))
            .finally(() => setLeaderboardLoading(false));
    }, [contest, authfetch, navigate]);

    if (loading)
        return (
            <p className="text-gray-500 text-center mt-10">
                Loading contest details…
            </p>
        );
    if (error)
        return <p className="text-red-500 text-center mt-10">Error: {error}</p>;
    if (!contest)
        return (
            <p className="text-gray-500 text-center mt-10">No contest found.</p>
        );

    return (
        <div className="max-w-4xl mx-auto p-4">
            {/* Contest Info */}
            <header className="mb-6">
                <h1 className="text-3xl font-bold mb-2">{contest.title}</h1>{" "}
                {/*<button
                    className=""
                    onClick={() => {
                        console.log("Edit button clicked");
                    }}
                    disabled={contest.author_id == user.id ? false : true}
                >
                    Edit
                </button>*/}
                {contest.description && (
                    <p className="text-gray-700 mb-2">{contest.description}</p>
                )}
                <p className="text-gray-600">
                    <span className="font-semibold">Start:</span>{" "}
                    {new Date(contest.start_time).toLocaleString()} <br />
                    <span className="font-semibold">End:</span>{" "}
                    {new Date(contest.end_time).toLocaleString()} <br />
                    <span className="font-semibold">Visibility:</span>{" "}
                    {contest.is_public ? "Public" : "Private"}
                </p>
            </header>

            <hr className="my-6 border-gray-300" />

            {/* Problems */}
            <section>
                <div className="flex justify-between items-center mb-4">
                    <h2 className="text-2xl font-semibold">Problems</h2>
                </div>

                {problemsLoading ? (
                    <p className="text-gray-500">Loading problems…</p>
                ) : problems.length === 0 ? (
                    <p className="text-gray-500">
                        No problems available for this contest yet.
                    </p>
                ) : (
                    <ol className="space-y-4">
                        {problems.map((p) => (
                            <li
                                key={p.id}
                                className="p-4 border border-gray-200 rounded-md hover:shadow-lg transition cursor-pointer flex justify-between items-center"
                                onClick={() => navigate(`/problems/${p.id}`)}
                            >
                                <div>
                                    <span className="font-semibold mr-2">
                                        {p.label || ""}
                                    </span>
                                    <span className="text-blue-600 underline">
                                        {p.title}
                                    </span>
                                </div>
                                <span
                                    className={`ml-4 font-medium ${
                                        p.difficulty === "hard"
                                            ? "text-red-600"
                                            : p.difficulty === "medium"
                                              ? "text-yellow-600"
                                              : "text-green-600"
                                    }`}
                                >
                                    {p.difficulty}
                                </span>
                            </li>
                        ))}
                    </ol>
                )}
            </section>

            <hr className="my-6 border-gray-300" />

            {/* Leaderboard */}
            <section>
                <h2 className="text-2xl font-semibold mb-4">Leaderboard</h2>

                {leaderboardLoading ? (
                    <p className="text-gray-500">Loading leaderboard…</p>
                ) : leaderboard.length === 0 ? (
                    <p className="text-gray-500">No submissions yet.</p>
                ) : (
                    <div className="overflow-x-auto">
                        <table className="min-w-full table-auto border-collapse border border-gray-300">
                            <thead>
                                <tr className="bg-gray-100">
                                    <th className="border border-gray-300 px-4 py-2">Rank</th>
                                    <th className="border border-gray-300 px-4 py-2">Username</th>
                                    <th className="border border-gray-300 px-4 py-2">Solved</th>
                                    <th className="border border-gray-300 px-4 py-2">Penalty</th>
                                    {problems.map((p) => (
                                        <th key={p.id} className="border border-gray-300 px-4 py-2">
                                            {p.label || p.title}
                                        </th>
                                    ))}
                                </tr>
                            </thead>
                            <tbody>
                                {leaderboard.map((entry, index) => (
                                    <tr key={entry.user_id} className="hover:bg-gray-50">
                                        <td className="border border-gray-300 px-4 py-2 text-center">
                                            {index + 1}
                                        </td>
                                        <td className="border border-gray-300 px-4 py-2">
                                            {entry.username}
                                        </td>
                                        <td className="border border-gray-300 px-4 py-2 text-center">
                                            {entry.solved}
                                        </td>
                                        <td className="border border-gray-300 px-4 py-2 text-center">
                                            {Math.floor(entry.penalty / 60)}:{(entry.penalty % 60).toString().padStart(2, '0')}
                                        </td>
                                        {problems.map((p) => {
                                            const status = entry.problems[p.id];
                                            return (
                                                <td key={p.id} className="border border-gray-300 px-4 py-2 text-center">
                                                    {status ? (
                                                        status.solved ? (
                                                            <span className="text-green-600 font-bold">
                                                                + ({status.attempts})
                                                            </span>
                                                        ) : (
                                                            <span className="text-red-600">
                                                                - ({status.attempts})
                                                            </span>
                                                        )
                                                    ) : (
                                                        <span className="text-gray-400">-</span>
                                                    )}
                                                </td>
                                            );
                                        })}
                                    </tr>
                                ))}
                            </tbody>
                        </table>
                    </div>
                )}
            </section>
        </div>
    );
};

export default ContestDetail;
