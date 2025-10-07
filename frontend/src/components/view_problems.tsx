// import { useNavigate } from "react-router-dom";
// import type { Problem } from "../problems/problems";

// interface ViewProblemsProps {
//     problems: Problem[];
//     id?: boolean;
//     slug?: boolean;
//     title?: boolean;
//     difficulty?: boolean;
//     is_public?: boolean;
//     edit?: boolean;
// }
// const difficultyColors: Record<string, string> = {
//     Easy: "bg-green-100 text-green-800",
//     Medium: "bg-yellow-100 text-yellow-800",
//     Hard: "bg-red-100 text-red-800",
// };
// export function ViewProblems({
//     problems,
//     id = false,
//     slug = false,
//     title = false,
//     difficulty = false,
//     is_public = false,
//     edit = false,
// }: ViewProblemsProps) {
//     const navigator = useNavigate();
//     return (
//         <table className="w-full border-collapse">
//             <thead className="bg-gray-100 text-left">
//                 <tr>
//                     <th className="p-3">#</th>
//                     <th className="p-3">Title</th>
//                     <th className="p-3">Slug</th>
//                     <th className="p-3">Difficulty</th>
//                     {show_is_public && <th className="p-3">Visibility</th>}
//                     <th className="p-3">Created At</th>
//                     {edit && <th className="p-3">Actions</th>}
//                 </tr>
//             </thead>
//             <tbody>
//                 {problems.map((p) => (
//                     <tr key={p.id} className="border-b hover:bg-gray-50">
//                         <td className="p-3">
//                             {/*{(page - 1) * limit + idx + 1}*/}
//                             {p.id}
//                         </td>
//                         <td
//                             className="p-3 text-blue-600 cursor-pointer hover:underline"
//                             onClick={() => navigator(`/problems/${p.id}`)}
//                         >
//                             {p.title}
//                         </td>
//                         <td className="p-3">{p.id}</td>
//                         <td className="p-3">
//                             {p.difficulty ? (
//                                 <span
//                                     className={`px-2 py-1 rounded-full text-xs font-semibold ${
//                                         difficultyColors[p.difficulty] ||
//                                         "bg-gray-200 text-gray-800"
//                                     }`}
//                                 >
//                                     {p.difficulty}
//                                 </span>
//                             ) : (
//                                 "-"
//                             )}
//                         </td>
//                         {show_is_public && (
//                             <td className="p-3">
//                                 {p.is_public ? "Public" : "Private"}
//                             </td>
//                         )}
//                         <td className="p-3 text-sm text-gray-500">
//                             {new Date(p.created_at).toLocaleDateString()}
//                         </td>
//                         {edit && (
//                             <td className="p-3">
//                                 <button
//                                     onClick={(e) => {
//                                         e.stopPropagation();
//                                         navigator(
//                                             `/admin/edit_problem/${p.id}`,
//                                         );
//                                     }}
//                                     className="px-3 py-1 text-sm bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
//                                 >
//                                     Edit
//                                 </button>
//                             </td>
//                         )}
//                     </tr>
//                 ))}
//             </tbody>
//         </table>
//     );
// }

import { useNavigate } from "react-router-dom";
import type { Problem } from "../problems/problems";
import { getStatusColor } from "../utils/helpers";

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

const difficultyColors: Record<string, string> = {
    Easy: "bg-green-100 text-green-800",
    Medium: "bg-yellow-100 text-yellow-800",
    Hard: "bg-red-100 text-red-800",
};

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
            <table className="w-full border-collapse">
                <thead className="bg-gray-100 text-left">
                    <tr>
                        {id && <th className="p-3">#</th>}
                        {title && <th className="p-3">Title</th>}
                        {slug && <th className="p-3">Slug</th>}
                        {difficulty && <th className="p-3">Difficulty</th>}
                        {status && <th className="p-3">Status</th>}
                        {is_public && <th className="p-3">Visibility</th>}
                        <th className="p-3">Created At</th>
                        {edit && <th className="p-3">Actions</th>}
                    </tr>
                </thead>

                <tbody>
                    {problems.map((p) => (
                        <tr
                            key={p.id}
                            className="border-b hover:bg-gray-50 transition-colors"
                        >
                            {id && <td className="p-3">{p.id}</td>}

                            {title && (
                                <td
                                    className="p-3 text-blue-600 cursor-pointer hover:underline"
                                    onClick={() =>
                                        navigate(`/problems/${p.id}`)
                                    }
                                >
                                    {p.title}
                                </td>
                            )}

                            {slug && <td className="p-3">{p.slug}</td>}

                            {difficulty && (
                                <td className="p-3">
                                    {p.difficulty ? (
                                        <span
                                            className={`px-2 py-1 rounded-full text-xs font-semibold ${
                                                difficultyColors[
                                                    p.difficulty
                                                ] || "bg-gray-200 text-gray-800"
                                            }`}
                                        >
                                            {p.difficulty}
                                        </span>
                                    ) : (
                                        "-"
                                    )}
                                </td>
                            )}

                            {is_public && (
                                <td className="p-3">
                                    {p.is_public ? "Public" : "Private"}
                                </td>
                            )}

                            {status && (
                                <td
                                    className={`p-3 ${getStatusColor(p.status? p.status: "")}`}
                                >
                                    {p.status}
                                </td>
                            )}

                            <td className="p-3 text-sm text-gray-500">
                                {new Date(p.created_at).toLocaleDateString()}
                            </td>

                            {edit && (
                                <td className="p-3">
                                    <button
                                        onClick={(e) => {
                                            e.stopPropagation();
                                            navigate(
                                                `/admin/edit_problem/${p.id}`,
                                            );
                                        }}
                                        className="px-3 py-1 text-sm bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
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
                <p className="text-center text-gray-500 mt-4">
                    No problems found.
                </p>
            )}
        </div>
    );
}
