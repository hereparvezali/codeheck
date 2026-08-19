import { useNavigate } from "react-router-dom";
import type { Problem } from "../problems/problems";
import { getDifficultyColor, getStatusColor, parseUtcDate } from "../utils/helpers";

interface ViewProblemsProps {
    problems: Problem[];
    id?: boolean;
    slug?: boolean;
    title?: boolean;
    difficulty?: boolean;
    is_public?: boolean;
    edit?: boolean;
    status?: boolean;
}

export function ViewProblems({
    problems,
    id = true,
    slug = false,
    title = true,
    difficulty = true,
    status = false,
    is_public = false,
    edit = false,
}: ViewProblemsProps) {
    const navigate = useNavigate();

    return (
        <div className="overflow-x-auto">
            <table className="w-full border-collapse text-left text-sm">
                <thead className="bg-zinc-900/80 text-zinc-400 uppercase text-xs tracking-wider border-b border-zinc-800">
                    <tr>
                        {id && <th className="p-4">#</th>}
                        {title && <th className="p-4">Title</th>}
                        {slug && <th className="p-4">Slug</th>}
                        {difficulty && <th className="p-4">Difficulty</th>}
                        {status && <th className="p-4">Status</th>}
                        {is_public && <th className="p-4">Visibility</th>}
                        <th className="p-4">Created At</th>
                        {edit && <th className="p-4 text-right">Actions</th>}
                    </tr>
                </thead>

                <tbody className="divide-y divide-zinc-900">
                    {problems.map((p) => (
                        <tr
                            key={p.id}
                            className="hover:bg-zinc-900/60 transition-colors group cursor-pointer"
                            onClick={() => navigate(`/problems/${p.id}`)}
                        >
                            {id && <td className="p-4 font-mono text-zinc-500 text-xs">#{p.id}</td>}

                            {title && (
                                <td className="p-4 font-medium text-zinc-200 group-hover:text-white transition">
                                    {p.title}
                                </td>
                            )}

                            {slug && <td className="p-4 text-zinc-400 font-mono text-xs">{p.slug}</td>}

                            {difficulty && (
                                <td className="p-4">
                                    {p.difficulty ? (
                                        <span
                                            className={`px-2.5 py-0.5 rounded-md text-xs capitalize ${getDifficultyColor(
                                                p.difficulty
                                            )}`}
                                        >
                                            {p.difficulty}
                                        </span>
                                    ) : (
                                        "-"
                                    )}
                                </td>
                            )}

                            {status && (
                                <td className="p-4">
                                    {p.status ? (
                                        <span className={`px-2 py-0.5 rounded text-xs ${getStatusColor(p.status)}`}>
                                            {p.status}
                                        </span>
                                    ) : (
                                        <span className="text-zinc-600 text-xs">-</span>
                                    )}
                                </td>
                            )}

                            {is_public && (
                                <td className="p-4">
                                    <span className={`text-xs font-medium px-2 py-0.5 rounded border ${p.is_public ? "text-zinc-300 bg-zinc-900 border-zinc-800" : "text-zinc-500 bg-zinc-950 border-zinc-900"}`}>
                                        {p.is_public ? "Public" : "Private"}
                                    </span>
                                </td>
                            )}

                            <td className="p-4 text-xs text-zinc-500">
                                {parseUtcDate(p.created_at).toLocaleDateString()}
                            </td>

                            {edit && (
                                <td className="p-4 text-right">
                                    <button
                                        onClick={(e) => {
                                            e.stopPropagation();
                                            navigate(`/admin/edit_problem/${p.id}`);
                                        }}
                                        className="px-3 py-1 text-xs bg-zinc-900 hover:bg-zinc-800 text-zinc-300 border border-zinc-800 rounded-lg transition"
                                    >
                                        Edit
                                    </button>
                                </td>
                            )}
                        </tr>
                    ))}
                </tbody>
            </table>

            {problems.length === 0 && (
                <div className="text-center text-zinc-500 py-12">
                    No problems found.
                </div>
            )}
        </div>
    );
}
