import { useNavigate } from "react-router-dom";
import type { Contest } from "../contests/contests";
import { formatUtcToLocal, parseUtcDate } from "../utils/helpers";

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
    id = true,
    title = true,
    slug = false,
    description = true,
    start_time = true,
    end_time = true,
    is_public = true,
    author_id = false,
    edit = false,
}: ViewContestsProps) {
    const navigate = useNavigate();

    const getStatus = (start: string, end: string) => {
        const now = new Date();
        const startDate = parseUtcDate(start);
        const endDate = parseUtcDate(end);

        if (now < startDate)
            return {
                text: "Upcoming",
                badgeClass: "bg-sky-500/10 text-sky-400 border-sky-500/25",
            };
        if (now >= startDate && now <= endDate)
            return {
                text: "● Live Now",
                badgeClass: "bg-emerald-500/10 text-emerald-400 border-emerald-500/30 font-semibold",
            };
        return {
            text: "Finished",
            badgeClass: "bg-zinc-900/60 text-zinc-500 border-zinc-800/80",
        };
    };

    if (contests.length === 0) {
        return (
            <div className="text-center text-zinc-500 py-16 bg-zinc-950 border border-zinc-900 rounded-2xl">
                <p className="text-base font-semibold text-zinc-400">No contests available</p>
                <p className="text-sm text-zinc-500 mt-1">Check back later for newly scheduled competitive events.</p>
            </div>
        );
    }

    return (
        <div className="grid grid-cols-1 gap-3.5">
            {contests.map((c) => {
                const status = getStatus(c.start_time, c.end_time);
                return (
                    <div
                        key={c.id}
                        onClick={() => navigate(`/contests/${c.id}`)}
                        className="p-5 bg-zinc-950 border border-zinc-900 hover:border-zinc-800 rounded-2xl transition cursor-pointer group"
                    >
                        <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-3 mb-2.5">
                            <div className="flex items-center gap-3">
                                {id && <span className="text-xs font-mono text-zinc-500">#{c.id}</span>}
                                {title && (
                                    <h3 className="text-lg font-bold text-zinc-100 group-hover:text-white transition">
                                        {c.title}
                                    </h3>
                                )}
                                <span className={`px-2.5 py-0.5 rounded-full text-xs border ${status.badgeClass}`}>
                                    {status.text}
                                </span>
                            </div>

                            <div className="flex items-center gap-2" onClick={(e) => e.stopPropagation()}>
                                {edit && (
                                    <button
                                        onClick={() => navigate(`/admin/edit_contest/${c.id}`)}
                                        className="px-3 py-1 text-xs bg-zinc-900 hover:bg-zinc-800 text-zinc-300 rounded-lg border border-zinc-800 transition"
                                    >
                                        Edit
                                    </button>
                                )}

                                {status.text === "Finished" ? (
                                    <button
                                        disabled
                                        className="px-4 py-1.5 bg-zinc-900 text-zinc-600 rounded-xl text-xs font-semibold border border-zinc-800/50 cursor-not-allowed"
                                    >
                                        Ended
                                    </button>
                                ) : (
                                    <>
                                        {handleRegister && !c.registration_id && (
                                            <button
                                                onClick={() => handleRegister(c.id)}
                                                className="px-4 py-1.5 bg-zinc-100 hover:bg-white text-zinc-950 rounded-xl text-xs font-semibold transition shadow-sm"
                                            >
                                                Register
                                            </button>
                                        )}

                                        {handleUnRegister && c.registration_id && (
                                            <button
                                                onClick={() => handleUnRegister(c.registration_id)}
                                                className="px-4 py-1.5 bg-zinc-900 hover:bg-zinc-800 text-zinc-300 border border-zinc-800 rounded-xl text-xs font-semibold transition"
                                            >
                                                Unregister
                                            </button>
                                        )}
                                    </>
                                )}
                            </div>
                        </div>

                        {slug && <p className="text-xs font-mono text-zinc-500 mb-2">{c.slug}</p>}

                        {description && c.description && (
                            <p className="text-sm text-zinc-400 line-clamp-2 mb-3 leading-relaxed">
                                {c.description}
                            </p>
                        )}

                        <div className="flex flex-wrap items-center justify-between gap-4 text-xs text-zinc-500 border-t border-zinc-900 pt-3">
                            {(start_time || end_time) && (
                                <div className="flex items-center gap-1.5 font-mono">
                                    <span>🕒</span>
                                    <span>{start_time && formatUtcToLocal(c.start_time)}</span>
                                    <span>&rarr;</span>
                                    <span>{end_time && formatUtcToLocal(c.end_time)}</span>
                                </div>
                            )}

                            <div className="flex items-center gap-3">
                                {is_public && (
                                    <span className={`text-xs px-2 py-0.5 rounded border ${c.is_public ? "text-zinc-300 bg-zinc-900 border-zinc-800" : "text-zinc-500 bg-zinc-950 border-zinc-900"}`}>
                                        {c.is_public ? "Public" : "Private"}
                                    </span>
                                )}
                                {author_id && c.author_id && (
                                    <span className="text-zinc-500">Author: #{c.author_id}</span>
                                )}
                            </div>
                        </div>
                    </div>
                );
            })}
        </div>
    );
}
