import { useNavigate } from "react-router-dom";
import { formatUtcToLocal, getStatusColor } from "../utils/helpers";

export interface Submission {
    id: number;
    user_id: number;
    problem_id: number;
    language: string;
    code: string;
    status: string;
    verdict?: string;
    time?: number;
    memory?: number;
    submitted_at: string;
    contest_id?: number;
}

interface ViewSubmissionsProps {
    submissions: Submission[];
    id?: boolean;
    user_id?: boolean;
    problem_id?: boolean;
    contest_id?: boolean;
    language?: boolean;
    status?: boolean;
    verdict?: boolean;
    time?: boolean;
    memory?: boolean;
    submitted_at?: boolean;
    view_code?: boolean;
}

export function ViewSubmissions({
    submissions,
    id = true,
    user_id = false,
    problem_id = true,
    contest_id = false,
    language = true,
    status = true,
    verdict = false,
    time = true,
    memory = true,
    submitted_at = true,
    view_code = false,
}: ViewSubmissionsProps) {
    const navigate = useNavigate();

    return (
        <div className="overflow-x-auto">
            <table className="w-full border-collapse text-left text-sm">
                <thead className="bg-zinc-900/80 text-zinc-400 uppercase text-xs tracking-wider border-b border-zinc-800">
                    <tr>
                        {id && <th className="p-3.5">#</th>}
                        {user_id && <th className="p-3.5">User</th>}
                        {problem_id && <th className="p-3.5">Problem</th>}
                        {contest_id && <th className="p-3.5">Contest</th>}
                        {language && <th className="p-3.5">Language</th>}
                        {status && <th className="p-3.5">Status</th>}
                        {verdict && <th className="p-3.5">Verdict</th>}
                        {time && <th className="p-3.5">Time</th>}
                        {memory && <th className="p-3.5">Memory</th>}
                        {submitted_at && <th className="p-3.5">Submitted At</th>}
                        {view_code && <th className="p-3.5 text-right">Action</th>}
                    </tr>
                </thead>
                <tbody className="divide-y divide-zinc-900">
                    {submissions.length === 0 ? (
                        <tr>
                            <td colSpan={11} className="text-center text-zinc-500 py-8">
                                No submissions found.
                            </td>
                        </tr>
                    ) : (
                        submissions.map((s) => (
                            <tr
                                key={s.id}
                                className="hover:bg-zinc-900/40 transition-colors group cursor-pointer"
                                onClick={() => navigate(`/submissions/${s.id}`)}
                            >
                                {id && <td className="p-3.5 font-mono text-zinc-500 text-xs">#{s.id}</td>}
                                {user_id && (
                                    <td className="p-3.5 font-mono text-zinc-400 text-xs">
                                        User #{s.user_id}
                                    </td>
                                )}
                                {problem_id && (
                                    <td
                                        className="p-3.5 text-zinc-200 hover:text-white font-medium text-xs"
                                        onClick={(e) => {
                                            e.stopPropagation();
                                            navigate(`/problems/${s.problem_id}`);
                                        }}
                                    >
                                        Problem #{s.problem_id}
                                    </td>
                                )}
                                {contest_id && (
                                    <td className="p-3.5 text-zinc-500 text-xs">
                                        {s.contest_id ? `Contest #${s.contest_id}` : "-"}
                                    </td>
                                )}
                                {language && (
                                    <td className="p-3.5 font-mono text-xs text-zinc-300 uppercase">
                                        {s.language}
                                    </td>
                                )}
                                {status && (
                                    <td className="p-3.5">
                                        <span className={`px-2.5 py-0.5 rounded text-xs ${getStatusColor(s.status)}`}>
                                            {s.status}
                                        </span>
                                    </td>
                                )}
                                {verdict && (
                                    <td className="p-3.5 text-zinc-400 text-xs max-w-xs truncate">
                                        {s.verdict ?? "-"}
                                    </td>
                                )}
                                {time && (
                                    <td className="p-3.5 font-mono text-xs text-zinc-400">
                                        {s.time !== undefined && s.time !== null ? `${s.time} ms` : "-"}
                                    </td>
                                )}
                                {memory && (
                                    <td className="p-3.5 font-mono text-xs text-zinc-400">
                                        {s.memory !== undefined && s.memory !== null ? `${s.memory} KB` : "-"}
                                    </td>
                                )}
                                {submitted_at && (
                                    <td className="p-3.5 text-xs text-zinc-500">
                                        {formatUtcToLocal(s.submitted_at)}
                                    </td>
                                )}
                                {view_code && (
                                    <td className="p-3.5 text-right">
                                        <button
                                            onClick={(e) => {
                                                e.stopPropagation();
                                                navigate(`/submissions/${s.id}`);
                                            }}
                                            className="px-3 py-1 text-xs bg-zinc-900 hover:bg-zinc-800 text-zinc-300 border border-zinc-800 rounded-lg transition"
                                        >
                                            View Code
                                        </button>
                                    </td>
                                )}
                            </tr>
                        ))
                    )}
                </tbody>
            </table>
        </div>
    );
}
