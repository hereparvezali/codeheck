import React, { useState, useEffect } from "react";
import { useAuth } from "../utils/contexts/authcontext";
import { useNavigate } from "react-router-dom";

interface CreateContestPayload {
    title: string;
    slug: string;
    description?: string;
    start_time: string;
    end_time: string;
    is_public: boolean;
}

interface ContestResponse {
    id: number;
    title: string;
    slug: string;
    description?: string;
    start_time: string;
    end_time: string;
    is_public: boolean;
}

interface Problem {
    id: number;
    title: string;
    slug: string;
    difficulty?: string;
    is_public: boolean;
}

interface ProblemIdAndLabel {
    problem_id: number;
    label?: string;
}

const CreateContest = () => {
    const navigator = useNavigate();
    const { authfetch } = useAuth();
    const [form, setForm] = useState<CreateContestPayload>({
        title: "",
        slug: "",
        description: "",
        start_time: "",
        end_time: "",
        is_public: false,
    });
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [step, setStep] = useState<"contest" | "problems">("contest");
    const [createdContest, setCreatedContest] = useState<ContestResponse | null>(null);
    const [availableProblems, setAvailableProblems] = useState<Problem[]>([]);
    const [selectedProblems, setSelectedProblems] = useState<Map<number, string>>(new Map());
    const [loadingProblems, setLoadingProblems] = useState(false);
    const [durationHours, setDurationHours] = useState<number>(2);

    // Fetch available problems for selection
    useEffect(() => {
        if (step === "problems") {
            fetchAvailableProblems();
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [step]);

    const fetchAvailableProblems = async () => {
        setLoadingProblems(true);
        try {
            const res = await authfetch("/problems?limit=100", {
                method: "GET",
            });

            if (!res.ok) {
                if (res.status === 401) navigator("/signin");
                throw new Error(await res.text());
            }

            const data = await res.json();
            setAvailableProblems(data.problems || []);
        } catch (e) {
            console.error("Failed to fetch problems:", e);
            setError(e instanceof Error ? e.message : "Failed to load problems");
        } finally {
            setLoadingProblems(false);
        }
    };

    const handleChange = (
        e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>,
    ) => {
        const { name, value, type } = e.target;

        setForm((prev: CreateContestPayload) => ({
            ...prev,
            [name]:
                type === "checkbox"
                    ? (e.target as HTMLInputElement).checked
                    : value,
        }));
    };

    const handleStartTimeChange = (value: string) => {
        setForm((prev) => {
            const newForm = { ...prev, start_time: value };
            // Auto-calculate end time based on duration
            if (value && durationHours > 0) {
                const startDate = new Date(value);
                const endDate = new Date(startDate.getTime() + durationHours * 60 * 60 * 1000);
                newForm.end_time = formatDateTimeLocal(endDate);
            }
            return newForm;
        });
    };

    const handleDurationChange = (hours: number) => {
        setDurationHours(hours);
        if (form.start_time && hours > 0) {
            const startDate = new Date(form.start_time);
            const endDate = new Date(startDate.getTime() + hours * 60 * 60 * 1000);
            setForm((prev) => ({
                ...prev,
                end_time: formatDateTimeLocal(endDate),
            }));
        }
    };

    const formatDateTimeLocal = (date: Date): string => {
        const year = date.getFullYear();
        const month = String(date.getMonth() + 1).padStart(2, "0");
        const day = String(date.getDate()).padStart(2, "0");
        const hours = String(date.getHours()).padStart(2, "0");
        const minutes = String(date.getMinutes()).padStart(2, "0");
        return `${year}-${month}-${day}T${hours}:${minutes}`;
    };

    const handleContestSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        setLoading(true);
        setError(null);

        const payload = {
            ...form,
            start_time: toUtcString(form.start_time),
            end_time: toUtcString(form.end_time),
        };

        authfetch(`/contest`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(payload),
        })
            .then(async (res) => {
                if (!res.ok) {
                    if (res.status === 401) {
                        navigator("/signin");
                        return;
                    }
                    const text = await res.text();
                    setError(text);
                    throw new Error(text);
                }
                return res.json();
            })
            .then((data: ContestResponse) => {
                setCreatedContest(data);
                setStep("problems");
            })
            .catch((e) => {
                setError(e.message ?? "Something went wrong");
                console.error(e);
            })
            .finally(() => {
                setLoading(false);
            });
    };

    const handleProblemToggle = (problemId: number, currentLabel?: string) => {
        setSelectedProblems((prev) => {
            const newMap = new Map(prev);
            if (newMap.has(problemId)) {
                newMap.delete(problemId);
            } else {
                newMap.set(problemId, currentLabel || "");
            }
            return newMap;
        });
    };

    const handleLabelChange = (problemId: number, label: string) => {
        setSelectedProblems((prev) => {
            const newMap = new Map(prev);
            newMap.set(problemId, label);
            return newMap;
        });
    };

    const handleAddProblems = async () => {
        if (!createdContest || selectedProblems.size === 0) return;

        setLoading(true);
        setError(null);

        const problems: ProblemIdAndLabel[] = Array.from(selectedProblems.entries()).map(
            ([problem_id, label]) => ({
                problem_id,
                label: label || undefined,
            })
        );

        try {
            const res = await authfetch("/contest/add_problem", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    contest_id: createdContest.id,
                    problems: problems,
                }),
            });

            if (!res.ok) {
                if (res.status === 401) {
                    navigator("/signin");
                    return;
                }
                const text = await res.text();
                throw new Error(text);
            }

            // Success - navigate to contest page
            navigator(`/contests/${createdContest.slug}`);
        } catch (e) {
            setError(e instanceof Error ? e.message : "Failed to add problems");
            console.error(e);
        } finally {
            setLoading(false);
        }
    };

    const handleSkipProblems = () => {
        if (createdContest) {
            navigator(`/contests/${createdContest.slug}`);
        }
    };

    if (step === "problems" && createdContest) {
        return (
            <div className="max-w-6xl mx-auto mt-10 p-6 bg-white rounded-xl shadow-lg">
                <div className="mb-6">
                    <h2 className="text-3xl font-bold mb-2">
                        Add Problems to "{createdContest.title}"
                    </h2>
                    <p className="text-gray-600">
                        Select problems and optionally assign labels (A, B, C, etc.) to organize your contest.
                    </p>
                </div>

                {error && (
                    <div className="mb-4 p-4 bg-red-50 border border-red-200 rounded-lg">
                        <p className="text-red-700 font-medium">Error</p>
                        <p className="text-red-600 text-sm">{error}</p>
                    </div>
                )}

                {loadingProblems ? (
                    <div className="text-center py-12">
                        <div className="animate-spin text-4xl mb-4">⏳</div>
                        <p className="text-gray-600">Loading problems...</p>
                    </div>
                ) : availableProblems.length === 0 ? (
                    <div className="text-center py-12 bg-gray-50 rounded-lg border-2 border-dashed border-gray-300">
                        <p className="text-gray-500 text-lg mb-4">
                            No problems available. Create some problems first.
                        </p>
                        <button
                            onClick={() => navigator("/admin/create_problem")}
                            className="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
                        >
                            Create Problem
                        </button>
                    </div>
                ) : (
                    <>
                        <div className="mb-4 p-4 bg-blue-50 border border-blue-200 rounded-lg">
                            <p className="text-blue-900 font-medium">
                                {selectedProblems.size} problem(s) selected
                            </p>
                        </div>

                        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6 max-h-[500px] overflow-y-auto">
                            {availableProblems.map((problem) => {
                                const isSelected = selectedProblems.has(problem.id);
                                const label = selectedProblems.get(problem.id) || "";

                                return (
                                    <div
                                        key={problem.id}
                                        className={`border rounded-lg p-4 transition-all ${
                                            isSelected
                                                ? "border-blue-500 bg-blue-50 shadow-md"
                                                : "border-gray-200 hover:border-gray-300 hover:shadow"
                                        }`}
                                    >
                                        <div className="flex items-start gap-3">
                                            <input
                                                type="checkbox"
                                                checked={isSelected}
                                                onChange={() => handleProblemToggle(problem.id, label)}
                                                className="mt-1 w-4 h-4"
                                            />
                                            <div className="flex-1">
                                                <div className="flex items-start justify-between mb-2">
                                                    <div>
                                                        <p className="font-semibold text-lg">{problem.title}</p>
                                                        <p className="text-sm text-gray-500">{problem.slug}</p>
                                                    </div>
                                                    <span
                                                        className={`text-xs px-2 py-1 rounded ${
                                                            problem.difficulty === "easy"
                                                                ? "bg-green-100 text-green-800"
                                                                : problem.difficulty === "medium"
                                                                ? "bg-yellow-100 text-yellow-800"
                                                                : problem.difficulty === "hard"
                                                                ? "bg-red-100 text-red-800"
                                                                : "bg-gray-100 text-gray-800"
                                                        }`}
                                                    >
                                                        {problem.difficulty || "N/A"}
                                                    </span>
                                                </div>

                                                {isSelected && (
                                                    <div className="mt-3">
                                                        <label className="block text-xs font-medium text-gray-700 mb-1">
                                                            Problem Label (optional)
                                                        </label>
                                                        <input
                                                            type="text"
                                                            value={label}
                                                            onChange={(e) =>
                                                                handleLabelChange(problem.id, e.target.value)
                                                            }
                                                            placeholder="e.g., A, B, C"
                                                            className="w-full px-3 py-1 border border-gray-300 rounded text-sm focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                                        />
                                                    </div>
                                                )}

                                                <div className="mt-2">
                                                    <span
                                                        className={`text-xs px-2 py-1 rounded ${
                                                            problem.is_public
                                                                ? "bg-green-100 text-green-800"
                                                                : "bg-gray-100 text-gray-800"
                                                        }`}
                                                    >
                                                        {problem.is_public ? "Public" : "Private"}
                                                    </span>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                );
                            })}
                        </div>

                        <div className="flex gap-3 pt-4 border-t">
                            <button
                                onClick={handleSkipProblems}
                                disabled={loading}
                                className="px-6 py-3 border border-gray-300 rounded-lg hover:bg-gray-50 disabled:opacity-50 font-medium"
                            >
                                Skip for Now
                            </button>
                            <button
                                onClick={handleAddProblems}
                                disabled={loading || selectedProblems.size === 0}
                                className="flex-1 bg-blue-600 text-white px-6 py-3 rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed font-medium"
                            >
                                {loading ? (
                                    <span className="flex items-center justify-center gap-2">
                                        <span className="animate-spin">⏳</span>
                                        Adding Problems...
                                    </span>
                                ) : (
                                    `Add ${selectedProblems.size} Problem(s) & Finish`
                                )}
                            </button>
                        </div>
                    </>
                )}
            </div>
        );
    }

    return (
        <div className="max-w-3xl mx-auto mt-10 p-6 bg-white rounded-xl shadow-lg">
            <div className="mb-6">
                <h2 className="text-3xl font-bold mb-2">Create New Contest</h2>
                <p className="text-gray-600">
                    Set up a new competitive programming contest with problems and time limits
                </p>
            </div>

            {error && (
                <div className="mb-4 p-4 bg-red-50 border border-red-200 rounded-lg">
                    <p className="text-red-700 font-medium">Error</p>
                    <p className="text-red-600 text-sm">{error}</p>
                </div>
            )}

            <form onSubmit={handleContestSubmit} className="space-y-6">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">
                            Contest Title *
                        </label>
                        <input
                            type="text"
                            name="title"
                            value={form.title}
                            onChange={handleChange}
                            required
                            placeholder="e.g., Weekly Contest #42"
                            className="w-full p-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        />
                    </div>

                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">
                            Slug (URL-friendly) *
                        </label>
                        <input
                            type="text"
                            name="slug"
                            value={form.slug}
                            onChange={handleChange}
                            required
                            placeholder="e.g., weekly-contest-42"
                            className="w-full p-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        />
                    </div>
                </div>

                <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                        Description
                    </label>
                    <textarea
                        name="description"
                        value={form.description}
                        onChange={handleChange}
                        placeholder="Describe your contest, rules, scoring system, etc."
                        className="w-full p-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        rows={4}
                    />
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">
                            Start Time *
                        </label>
                        <input
                            type="datetime-local"
                            name="start_time"
                            value={form.start_time}
                            onChange={(e) => handleStartTimeChange(e.target.value)}
                            required
                            className="w-full p-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        />
                    </div>

                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">
                            Duration (hours) *
                        </label>
                        <div className="flex gap-2">
                            {[1, 2, 3, 4, 6, 12, 24].map((hours) => (
                                <button
                                    key={hours}
                                    type="button"
                                    onClick={() => handleDurationChange(hours)}
                                    className={`px-3 py-2 rounded-lg text-sm font-medium transition-colors ${
                                        durationHours === hours
                                            ? "bg-blue-600 text-white"
                                            : "bg-gray-100 text-gray-700 hover:bg-gray-200"
                                    }`}
                                >
                                    {hours}h
                                </button>
                            ))}
                        </div>
                    </div>
                </div>

                <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                        End Time *
                    </label>
                    <input
                        type="datetime-local"
                        name="end_time"
                        value={form.end_time}
                        onChange={handleChange}
                        required
                        className="w-full p-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                    />
                    {form.start_time && form.end_time && (
                        <p className="text-sm text-gray-500 mt-1">
                            Duration: {calculateDuration(form.start_time, form.end_time)}
                        </p>
                    )}
                </div>

                <div className="flex items-center gap-3 p-4 bg-gray-50 rounded-lg border border-gray-200">
                    <input
                        type="checkbox"
                        name="is_public"
                        id="is_public"
                        checked={form.is_public}
                        onChange={handleChange}
                        className="w-4 h-4 text-blue-600 focus:ring-2 focus:ring-blue-500"
                    />
                    <label htmlFor="is_public" className="text-sm font-medium text-gray-700">
                        <span className="block">Make this contest public</span>
                        <span className="text-gray-500 text-xs">
                            Public contests are visible to all users
                        </span>
                    </label>
                </div>

                <div className="flex gap-3 pt-4 border-t">
                    <button
                        type="button"
                        onClick={() => navigator(-1)}
                        className="px-6 py-3 border border-gray-300 rounded-lg hover:bg-gray-50 font-medium"
                    >
                        Cancel
                    </button>
                    <button
                        type="submit"
                        disabled={loading}
                        className="flex-1 bg-blue-600 text-white px-6 py-3 rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed font-medium"
                    >
                        {loading ? (
                            <span className="flex items-center justify-center gap-2">
                                <span className="animate-spin">⏳</span>
                                Creating Contest...
                            </span>
                        ) : (
                            "Create Contest & Add Problems"
                        )}
                    </button>
                </div>
            </form>
        </div>
    );
};

export default CreateContest;

function toUtcString(datetimeLocal: string): string {
    const date = new Date(datetimeLocal);
    const year = date.getUTCFullYear();
    const month = String(date.getUTCMonth() + 1).padStart(2, "0");
    const day = String(date.getUTCDate()).padStart(2, "0");
    const hours = String(date.getUTCHours()).padStart(2, "0");
    const minutes = String(date.getUTCMinutes()).padStart(2, "0");

    return `${year}-${month}-${day}T${hours}:${minutes}:00`;
}

function calculateDuration(start: string, end: string): string {
    const startDate = new Date(start);
    const endDate = new Date(end);
    const diffMs = endDate.getTime() - startDate.getTime();
    const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
    const diffMinutes = Math.floor((diffMs % (1000 * 60 * 60)) / (1000 * 60));
    
    if (diffHours < 0 || diffMinutes < 0) return "Invalid duration";
    
    if (diffHours === 0) {
        return `${diffMinutes} minutes`;
    } else if (diffMinutes === 0) {
        return `${diffHours} ${diffHours === 1 ? "hour" : "hours"}`;
    } else {
        return `${diffHours} ${diffHours === 1 ? "hour" : "hours"} ${diffMinutes} minutes`;
    }
}