// import { useNavigate } from "react-router-dom";
// import type { Contest } from "../contests/contests";

// interface ViewContestsProps {
//     contests: Contest[];
//     handleRegister?: (contest_id: number) => void;
//     handleUnRegister?: (registration_id?: number) => void;
//     id?: boolean;
//     title?: boolean;
//     slug?: boolean;
//     description?: boolean;
//     start_time?: boolean;
//     end_time?: boolean;
//     is_public?: boolean;
//     author_id?: boolean;

//     edit?: boolean;
// }
// export function ViewContests({
//     contests,
//     handleRegister,
//     handleUnRegister,
//     id = false,
//     title = false,
//     slug = false,
//     description = false,
//     start_time = false,
//     end_time = false,
//     is_public = false,
//     author_id = false,

//     edit = false,
// }: ViewContestsProps) {
//     const navigator = useNavigate();
//     const getStatus = (start: string, end: string) => {
//         const now = new Date();
//         const startDate = new Date(start);
//         const endDate = new Date(end);

//         if (now < startDate) return { text: "Upcoming", color: "#007bff" };
//         if (now >= startDate && now <= endDate)
//             return { text: "Ongoing", color: "#28a745" };
//         return { text: "Ended", color: "#dc3545" };
//     };
//     return (
//         <div className="flex flex-col gap-4">
//             {contests.map((c) => {
//                 const status = getStatus(c.start_time, c.end_time);
//                 return (
//                     <div
//                         key={c.id}
//                         onClick={() => navigator(`/contests/${c.id}`)}
//                         className="cursor-pointer border rounded-lg p-4 shadow transition-transform transform hover:scale-[1.01] bg-white"
//                     >
//                         <div className="flex justify-between items-center mb-2">
//                             <h2 className="text-lg font-semibold">{c.title}</h2>
//                             <span className="text-sm text-gray-500">
//                                 #{c.id}
//                             </span>

//                             {edit && (
//                                 <button
//                                     onClick={(e) => {
//                                         e.stopPropagation();
//                                         navigator(
//                                             `/admin/edit_contest/${c.id}`,
//                                         );
//                                     }}
//                                     className="px-3 py-1 text-sm bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
//                                 >
//                                     Edit
//                                 </button>
//                             )}
//                             {handleRegister && (
//                                 <button
//                                     hidden={c.registration_id ? true : false}
//                                     onClick={(e) => {
//                                         e.stopPropagation();
//                                         handleRegister(c.id);
//                                     }}
//                                     className="px-2 bg-blue-600 text-white rounded hover:bg-blue-700"
//                                 >
//                                     Register
//                                 </button>
//                             )}
//                             {handleUnRegister && (
//                                 <button
//                                     hidden={c.registration_id ? false : true}
//                                     onClick={(e) => {
//                                         e.stopPropagation();
//                                         handleUnRegister(c.registration_id);
//                                     }}
//                                     className="px-2 bg-blue-600 text-white rounded hover:bg-blue-700"
//                                 >
//                                     UnRegister
//                                 </button>
//                             )}
//                         </div>

//                         <p className="text-sm mb-1">
//                             <span className="font-medium">Status:</span>{" "}
//                             {status.text}
//                         </p>
//                         <p className="text-sm mb-1">
//                             {c.is_public ? "🌐 Public" : "🔒 Private"}
//                         </p>
//                         {c.description && (
//                             <p className="text-gray-700 text-sm mb-2">
//                                 {c.description}
//                             </p>
//                         )}
//                         <p className="text-xs text-gray-500">
//                             🕒 {new Date(c.start_time).toLocaleDateString()} →{" "}
//                             {new Date(c.start_time).toLocaleDateString()}
//                         </p>
//                     </div>
//                 );
//             })}
//         </div>
//     );
// }

import { useNavigate } from "react-router-dom";
import type { Contest } from "../contests/contests";

interface ViewContestsProps {
    contests: Contest[];
    handleRegister?: (contest_id: number) => void;
    handleUnRegister?: (registration_id?: number) => void;
    id?: boolean;
    title?: boolean;
    slug?: boolean;
    description?: boolean;
    start_time?: boolean;
    end_time?: boolean;
    is_public?: boolean;
    author_id?: boolean;
    edit?: boolean;
}

export function ViewContests({
    contests,
    handleRegister,
    handleUnRegister,
    id = false,
    title = true,
    slug = false,
    description = true,
    start_time = true,
    end_time = true,
    is_public = true,
    author_id = false,
    edit = false,
}: ViewContestsProps) {
    const navigator = useNavigate();

    const getStatus = (start: string, end: string) => {
        const now = new Date();
        const startDate = new Date(start);
        const endDate = new Date(end);

        if (now < startDate) return { text: "Upcoming", color: "text-blue-600" };
        if (now >= startDate && now <= endDate)
            return { text: "Ongoing", color: "text-green-600" };
        return { text: "Ended", color: "text-red-600" };
    };

    return (
        <div className="flex flex-col gap-4">
            {contests.length === 0 ? (
                <p className="text-center text-gray-500 text-sm">
                    No contests available.
                </p>
            ) : (
                contests.map((c) => {
                    const status = getStatus(c.start_time, c.end_time);
                    return (
                        <div
                            key={c.id}
                            onClick={() => navigator(`/contests/${c.id}`)}
                            className="cursor-pointer border rounded-lg p-4 shadow transition-transform transform hover:scale-[1.01] bg-white"
                        >
                            <div className="flex flex-wrap justify-between items-center gap-2 mb-2">
                                {title && (
                                    <h2 className="text-lg font-semibold text-blue-600 hover:underline">
                                        {c.title}
                                    </h2>
                                )}

                                {id && (
                                    <span className="text-sm text-gray-500">
                                        #{c.id}
                                    </span>
                                )}

                                {edit && (
                                    <button
                                        onClick={(e) => {
                                            e.stopPropagation();
                                            navigator(`/admin/edit_contest/${c.id}`);
                                        }}
                                        className="px-3 py-1 text-sm bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
                                    >
                                        Edit
                                    </button>
                                )}

                                {handleRegister && !c.registration_id && (
                                    <button
                                        onClick={(e) => {
                                            e.stopPropagation();
                                            handleRegister(c.id);
                                        }}
                                        className="px-2 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 text-sm"
                                    >
                                        Register
                                    </button>
                                )}

                                {handleUnRegister && c.registration_id && (
                                    <button
                                        onClick={(e) => {
                                            e.stopPropagation();
                                            handleUnRegister(c.registration_id);
                                        }}
                                        className="px-2 py-1 bg-red-600 text-white rounded hover:bg-red-700 text-sm"
                                    >
                                        Unregister
                                    </button>
                                )}
                            </div>

                            {slug && (
                                <p className="text-sm text-gray-500 mb-1">
                                    Slug: {c.slug}
                                </p>
                            )}

                            {description && c.description && (
                                <p className="text-gray-700 text-sm mb-2 line-clamp-3">
                                    {c.description}
                                </p>
                            )}

                            {is_public && (
                                <p className="text-sm mb-1">
                                    {c.is_public ? "🌐 Public" : "🔒 Private"}
                                </p>
                            )}

                            <p className={`text-sm mb-1 font-medium ${status.color}`}>
                                Status: {status.text}
                            </p>

                            {(start_time || end_time) && (
                                <p className="text-xs text-gray-500">
                                    🕒 {start_time && new Date(c.start_time).toLocaleString()}{" "}
                                    →{" "}
                                    {end_time && new Date(c.end_time).toLocaleString()}
                                </p>
                            )}

                            {author_id && (
                                <p className="text-xs text-gray-400 mt-1">
                                    Author ID: {c.author_id}
                                </p>
                            )}
                        </div>
                    );
                })
            )}
        </div>
    );
}
