import { Link } from "react-router-dom";

export interface UserStats {
    user_id: number;
    username: string;
    email: string;
    rating: number;
    created_at: string;
    total_solved: number;
    easy_solved: number;
    medium_solved: number;
    hard_solved: number;
    total_submissions: number;
    accepted_submissions: number;
}

interface ProfileOverviewProps {
    stats: UserStats | null;
}

export default function ProfileOverview({ stats }: ProfileOverviewProps) {
    if (!stats) return null;

    const acceptanceRate =
        stats.total_submissions > 0
            ? ((stats.accepted_submissions / stats.total_submissions) * 100).toFixed(1)
            : "0.0";

    return (
        <div className="space-y-6">
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                <div className="p-5 bg-zinc-950 border border-zinc-900 rounded-2xl">
                    <div className="text-xs font-semibold text-zinc-500 uppercase tracking-wider mb-2">Total Solved</div>
                    <div className="text-3xl font-extrabold text-white">{stats.total_solved}</div>
                </div>

                <div className="p-5 bg-zinc-950 border border-zinc-900 rounded-2xl">
                    <div className="text-xs font-semibold text-zinc-500 uppercase tracking-wider mb-2">Acceptance Rate</div>
                    <div className="text-3xl font-extrabold text-zinc-200">{acceptanceRate}%</div>
                </div>

                <div className="p-5 bg-zinc-950 border border-zinc-900 rounded-2xl">
                    <div className="text-xs font-semibold text-zinc-500 uppercase tracking-wider mb-2">Contest Rating</div>
                    <div className="text-3xl font-extrabold text-zinc-200">{stats.rating}</div>
                </div>

                <div className="p-5 bg-zinc-950 border border-zinc-900 rounded-2xl">
                    <div className="text-xs font-semibold text-zinc-500 uppercase tracking-wider mb-2">Total Submissions</div>
                    <div className="text-3xl font-extrabold text-zinc-200">{stats.total_submissions}</div>
                </div>
            </div>

            <div className="p-6 bg-zinc-950 border border-zinc-900 rounded-2xl">
                <h3 className="text-sm font-bold text-white mb-4">Solved Problems by Difficulty</h3>
                <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
                    <div className="p-4 bg-zinc-900 border border-zinc-800 rounded-xl">
                        <div className="flex justify-between items-center mb-2">
                            <span className="text-xs font-medium text-zinc-400 uppercase">Easy</span>
                            <span className="text-base font-bold text-white">{stats.easy_solved}</span>
                        </div>
                        <div className="w-full h-1.5 bg-zinc-800 rounded-full overflow-hidden">
                            <div
                                className="h-full bg-zinc-400"
                                style={{
                                    width: `${stats.total_solved > 0 ? (stats.easy_solved / stats.total_solved) * 100 : 0}%`,
                                }}
                            />
                        </div>
                    </div>

                    <div className="p-4 bg-zinc-900 border border-zinc-800 rounded-xl">
                        <div className="flex justify-between items-center mb-2">
                            <span className="text-xs font-medium text-zinc-400 uppercase">Medium</span>
                            <span className="text-base font-bold text-white">{stats.medium_solved}</span>
                        </div>
                        <div className="w-full h-1.5 bg-zinc-800 rounded-full overflow-hidden">
                            <div
                                className="h-full bg-zinc-400"
                                style={{
                                    width: `${stats.total_solved > 0 ? (stats.medium_solved / stats.total_solved) * 100 : 0}%`,
                                }}
                            />
                        </div>
                    </div>

                    <div className="p-4 bg-zinc-900 border border-zinc-800 rounded-xl">
                        <div className="flex justify-between items-center mb-2">
                            <span className="text-xs font-medium text-zinc-400 uppercase">Hard</span>
                            <span className="text-base font-bold text-white">{stats.hard_solved}</span>
                        </div>
                        <div className="w-full h-1.5 bg-zinc-800 rounded-full overflow-hidden">
                            <div
                                className="h-full bg-zinc-400"
                                style={{
                                    width: `${stats.total_solved > 0 ? (stats.hard_solved / stats.total_solved) * 100 : 0}%`,
                                }}
                            />
                        </div>
                    </div>
                </div>
            </div>

            <div className="p-6 bg-zinc-950 border border-zinc-900 rounded-2xl flex flex-col sm:flex-row justify-between items-center gap-4">
                <div>
                    <h4 className="text-sm font-bold text-white">Continue Practicing</h4>
                </div>
                <div className="flex gap-3">
                    <Link
                        to="/problems"
                        className="px-4 py-2 bg-zinc-100 hover:bg-white text-zinc-950 rounded-xl text-xs font-semibold shadow-sm transition"
                    >
                        Problems
                    </Link>
                    <Link
                        to="/contests"
                        className="px-4 py-2 bg-zinc-900 hover:bg-zinc-800 text-zinc-300 border border-zinc-800 rounded-xl text-xs font-semibold transition"
                    >
                        Contests
                    </Link>
                </div>
            </div>
        </div>
    );
}
