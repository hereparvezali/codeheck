import { useNavigate } from "react-router-dom";
import { getStatusColor } from "../utils/helpers";

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
    const navigator = useNavigate();

    return (
        <div className="overflow-x-auto">
            <table className="w-full border-collapse text-sm">
                <thead className="bg-gray-100 text-left">
                    <tr>
                        {id && <th className="p-3">#</th>}
                        {user_id && <th className="p-3">User</th>}
                        {problem_id && <th className="p-3">Problem</th>}
                        {contest_id && <th className="p-3">Contest</th>}
                        {language && <th className="p-3">Language</th>}
                        {status && <th className="p-3">Status</th>}
                        {verdict && <th className="p-3">Verdict</th>}
                        {time && <th className="p-3">Time (ms)</th>}
                        {memory && <th className="p-3">Memory (KB)</th>}
                        {submitted_at && <th className="p-3">Submitted At</th>}
                        {view_code && <th className="p-3">Action</th>}
                    </tr>
                </thead>
                <tbody>
                    {submissions.length === 0 ? (
                        <tr>
                            <td
                                colSpan={11}
                                className="text-center text-gray-500 py-4"
                            >
                                No submissions found.
                            </td>
                        </tr>
                    ) : (
                        submissions.map((s) => (
                            <tr
                                key={s.id}
                                className="border-b hover:bg-gray-50 transition"
                            >
                                {id && <td className="p-3">{s.id}</td>}
                                {user_id && (
                                    <td className="p-3 text-gray-700">
                                        {s.user_id}
                                    </td>
                                )}
                                {problem_id && (
                                    <td
                                        className="p-3 text-blue-600 cursor-pointer hover:underline"
                                        onClick={() =>
                                            navigator(
                                                `/problems/${s.problem_id}`,
                                            )
                                        }
                                    >
                                        {s.problem_id}
                                    </td>
                                )}
                                {contest_id && (
                                    <td className="p-3">
                                        {s.contest_id ?? "-"}
                                    </td>
                                )}
                                {language && (
                                    <td className="p-3">{s.language}</td>
                                )}
                                {status && (
                                    <td className="p-3">
                                        <span
                                            className={`px-2 py-1 rounded-full text-xs font-semibold ${getStatusColor(
                                                s.status,
                                            )}`}
                                        >
                                            {s.status}
                                        </span>
                                    </td>
                                )}
                                {verdict && (
                                    <td className="p-3 text-gray-700">
                                        {s.verdict ?? "-"}
                                    </td>
                                )}
                                {time && (
                                    <td className="p-3">
                                        {s.time ? `${s.time}` : "-"}
                                    </td>
                                )}
                                {memory && (
                                    <td className="p-3">
                                        {s.memory ? `${s.memory}` : "-"}
                                    </td>
                                )}
                                {submitted_at && (
                                    <td className="p-3 text-gray-500">
                                        {new Date(
                                            s.submitted_at,
                                        ).toLocaleString()}
                                    </td>
                                )}
                                {view_code && (
                                    <td className="p-3">
                                        <button
                                            onClick={(e) => {
                                                e.stopPropagation();
                                                navigator(
                                                    `/submissions/${s.id}`,
                                                );
                                            }}
                                            className="px-3 py-1 text-sm bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
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
