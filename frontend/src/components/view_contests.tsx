import { useNavigate } from "react-router-dom";
import type { Contest } from "../contests/contests";

interface ViewContestsProps {
    contests: Contest[];
    loading?: boolean;
    error?: string;
    handleRegister?: (contest_id: number) => void;
    handleUnRegister?: (registration_id?: number) => void;
}
export function ViewContests({
    contests,
    handleRegister,
    handleUnRegister,
}: ViewContestsProps) {
    const navigator = useNavigate();
    const getStatus = (start: string, end: string) => {
        const now = new Date();
        const startDate = new Date(start);
        const endDate = new Date(end);

        if (now < startDate) return { text: "Upcoming", color: "#007bff" };
        if (now >= startDate && now <= endDate)
            return { text: "Ongoing", color: "#28a745" };
        return { text: "Ended", color: "#dc3545" };
    };
    return (
        <div className="flex flex-col gap-4">
            {contests.map((c) => {
                const status = getStatus(c.start_time, c.end_time);
                return (
                    <div
                        key={c.id}
                        onClick={() => navigator(`/contests/${c.id}`)}
                        className="cursor-pointer border rounded-lg p-4 shadow transition-transform transform hover:scale-[1.01] bg-white"
                    >
                        <div className="flex justify-between items-center mb-2">
                            <h2 className="text-lg font-semibold">{c.title}</h2>
                            <span className="text-sm text-gray-500">
                                #{c.id}
                            </span>

                            {handleRegister && (
                                <button
                                    hidden={c.registration_id ? true : false}
                                    onClick={() => handleRegister(c.id)}
                                    className="px-2 bg-blue-600 text-white rounded hover:bg-blue-700"
                                >
                                    Register
                                </button>
                            )}
                            {handleUnRegister && (
                                <button
                                    hidden={c.registration_id ? false : true}
                                    onClick={() =>
                                        handleUnRegister(c.registration_id)
                                    }
                                    className="px-2 bg-blue-600 text-white rounded hover:bg-blue-700"
                                >
                                    UnRegister
                                </button>
                            )}
                        </div>

                        <p className="text-sm mb-1">
                            <span className="font-medium">Status:</span>{" "}
                            {status.text}
                        </p>
                        <p className="text-sm mb-1">
                            {c.is_public ? "🌐 Public" : "🔒 Private"}
                        </p>
                        {c.description && (
                            <p className="text-gray-700 text-sm mb-2">
                                {c.description}
                            </p>
                        )}
                        <p className="text-xs text-gray-500">
                            🕒 {new Date(c.start_time).toLocaleDateString()} →{" "}
                            {new Date(c.start_time).toLocaleDateString()}
                        </p>
                    </div>
                );
            })}
        </div>
    );
}
