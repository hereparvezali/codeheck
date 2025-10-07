import { useNavigate } from "react-router-dom";
import type { Problem } from "../problems/problems";

interface ViewProblemsProps {
    problems: Problem[];
    show_is_public: boolean;
}
const difficultyColors: Record<string, string> = {
    Easy: "bg-green-100 text-green-800",
    Medium: "bg-yellow-100 text-yellow-800",
    Hard: "bg-red-100 text-red-800",
};
export function ViewProblems({ problems, show_is_public }: ViewProblemsProps) {
    const navigator = useNavigate();
    return (
        <table className="w-full border-collapse">
            <thead className="bg-gray-100 text-left">
                <tr>
                    <th className="p-3">#</th>
                    <th className="p-3">Title</th>
                    <th className="p-3">Slug</th>
                    <th className="p-3">Difficulty</th>
                    {show_is_public && <th className="p-3">Visibility</th>}
                    <th className="p-3">Created At</th>
                </tr>
            </thead>
            <tbody>
                {problems.map((p) => (
                    <tr key={p.id} className="border-b hover:bg-gray-50">
                        <td className="p-3">
                            {/*{(page - 1) * limit + idx + 1}*/}
                            {p.id}
                        </td>
                        <td
                            className="p-3 text-blue-600 cursor-pointer hover:underline"
                            onClick={() => navigator(`/problems/${p.id}`)}
                        >
                            {p.title}
                        </td>
                        <td className="p-3">{p.id}</td>
                        <td className="p-3">
                            {p.difficulty ? (
                                <span
                                    className={`px-2 py-1 rounded-full text-xs font-semibold ${
                                        difficultyColors[p.difficulty] ||
                                        "bg-gray-200 text-gray-800"
                                    }`}
                                >
                                    {p.difficulty}
                                </span>
                            ) : (
                                "-"
                            )}
                        </td>
                        {show_is_public && (
                            <td className="p-3">
                                {p.is_public ? "Public" : "Private"}
                            </td>
                        )}
                        <td className="p-3 text-sm text-gray-500">
                            {new Date(p.created_at).toLocaleDateString()}
                        </td>
                    </tr>
                ))}
            </tbody>
        </table>
    );
}
