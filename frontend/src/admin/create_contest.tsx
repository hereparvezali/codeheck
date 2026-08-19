import React, { useState, useEffect } from "react";
import { useAuth } from "../utils/contexts/authcontext";
import { useNavigate } from "react-router-dom";
import { getDifficultyColor, toUtcIsoString } from "../utils/helpers";

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


    useEffect(() => {
        if (step === "problems") {
            fetchAvailableProblems();
        }

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
            start_time: toUtcIsoString(form.start_time),
            end_time: toUtcIsoString(form.end_time),
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
            const res = await authfetch("/contest/problems", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    id: createdContest.id,
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
            navigator(`/contests/${createdContest.id}`);
        }
    };

    if (step === "problems" && createdContest) {
        return (
            <div className="min-h-screen bg-black text-zinc-100 py-10 px-4 sm:px-6 lg:px-8">
                <div className="max-w-5xl mx-auto p-6 sm:p-8 bg-zinc-950 border border-zinc-900 rounded-2xl shadow-lg space-y-6">
                    <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 border-b border-zinc-900 pb-5">
                        <div>
                            <h2 className="text-2xl sm:text-3xl font-extrabold text-white tracking-tight">
                                Add Problems to "{createdContest.title}"
                            </h2>
                            <p className="text-xs text-zinc-400 mt-1">
                                Select problems and optionally assign labels (A, B, C, etc.) to organize your contest.
                            </p>
                        </div>
                        <button
                            type="button"
                            onClick={handleSkipProblems}
                            className="px-4 py-2 bg-zinc-900 hover:bg-zinc-800 text-zinc-300 border border-zinc-800 rounded-xl text-xs font-medium transition"
                        >
                            Skip for Now &rarr;
                        </button>
                    </div>

                    {error && (
                        <div className="p-4 bg-zinc-900 border border-zinc-800 rounded-xl text-xs text-zinc-300">
                            <p className="font-semibold text-white">Error</p>
                            <p className="mt-0.5">{error}</p>
                        </div>
                    )}

                    {loadingProblems ? (
                        <div className="text-center py-12 space-y-3">
                            <div className="w-7 h-7 border-2 border-zinc-400 border-t-transparent rounded-full animate-spin mx-auto" />
                            <p className="text-zinc-400 text-xs">Loading problems...</p>
                        </div>
                    ) : availableProblems.length === 0 ? (
                        <div className="text-center py-12 bg-zinc-950 rounded-xl border border-dashed border-zinc-800">
                            <p className="text-zinc-400 font-medium text-xs mb-3">
                                No problems available. Create some problems first.
                            </p>
                            <button
                                onClick={() => navigator("/admin/create_problem")}
                                className="px-4 py-2 bg-zinc-100 hover:bg-white text-zinc-950 rounded-xl text-xs font-semibold transition shadow-sm"
                            >
                                Create Problem
                            </button>
                        </div>
                    ) : (
                        <>
                            <div className="p-3.5 bg-zinc-900 border border-zinc-800 rounded-xl flex items-center justify-between">
                                <p className="font-semibold text-zinc-200 text-xs">
                                    {selectedProblems.size} problem(s) selected
                                </p>
                                <span className="text-[11px] text-zinc-400">
                                    Click cards to toggle selection
                                </span>
                            </div>

                            <div className="grid grid-cols-1 md:grid-cols-2 gap-4 max-h-[500px] overflow-y-auto pr-1">
                                {availableProblems.map((problem) => {
                                    const isSelected = selectedProblems.has(problem.id);
                                    const label = selectedProblems.get(problem.id) || "";

                                    return (
                                        <div
                                            key={problem.id}
                                            onClick={() => handleProblemToggle(problem.id, label)}
                                            className={`border rounded-xl p-4 transition-all cursor-pointer ${
                                                isSelected
                                                    ? "border-zinc-600 bg-zinc-900"
                                                    : "border-zinc-800 bg-zinc-950 hover:border-zinc-700"
                                            }`}
                                        >
                                            <div className="flex items-start gap-3">
                                                <input
                                                    type="checkbox"
                                                    checked={isSelected}
                                                    onChange={() => {}}
                                                    className="mt-1 w-4 h-4 rounded bg-zinc-900 border-zinc-700 text-zinc-300 focus:ring-0 cursor-pointer"
                                                />
                                                <div className="flex-1">
                                                    <div className="flex items-start justify-between mb-2">
                                                        <div>
                                                            <p className="font-semibold text-sm text-white">{problem.title}</p>
                                                            <p className="text-xs text-zinc-500 font-mono mt-0.5">{problem.slug}</p>
                                                        </div>
                                                        <span
                                                            className={`text-xs px-2.5 py-0.5 rounded capitalize ${getDifficultyColor(
                                                                problem.difficulty
                                                            )}`}
                                                        >
                                                            {problem.difficulty || "N/A"}
                                                        </span>
                                                    </div>

                                                    {isSelected && (
                                                        <div className="mt-3 pt-3 border-t border-zinc-800" onClick={(e) => e.stopPropagation()}>
                                                            <label className="block text-xs font-medium text-zinc-400 uppercase tracking-wider mb-1">
                                                                Problem Label (optional)
                                                            </label>
                                                            <input
                                                                type="text"
                                                                value={label}
                                                                onChange={(e) =>
                                                                    handleLabelChange(problem.id, e.target.value)
                                                                }
                                                                placeholder="e.g., A, B, C"
                                                                className="w-full px-3 py-1.5 bg-zinc-900 border border-zinc-800 rounded-lg text-xs text-white placeholder-zinc-600 focus:border-zinc-500 outline-none font-mono"
                                                            />
                                                        </div>
                                                    )}

                                                    <div className="mt-2.5">
                                                        <span
                                                            className={`text-xs px-2 py-0.5 rounded border ${
                                                                problem.is_public
                                                                    ? "bg-zinc-900 text-zinc-300 border-zinc-800"
                                                                    : "bg-zinc-950 text-zinc-500 border-zinc-900"
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

                            <div className="flex gap-3 pt-4 border-t border-zinc-900">
                                <button
                                    onClick={handleSkipProblems}
                                    disabled={loading}
                                    className="px-5 py-2.5 bg-zinc-900 hover:bg-zinc-800 text-zinc-300 border border-zinc-800 rounded-xl font-medium transition text-xs disabled:opacity-50"
                                >
                                    Skip for Now
                                </button>
                                <button
                                    onClick={handleAddProblems}
                                    disabled={loading || selectedProblems.size === 0}
                                    className="flex-1 bg-zinc-100 hover:bg-white text-zinc-950 font-semibold py-2.5 px-6 rounded-xl shadow-sm disabled:opacity-50 disabled:cursor-not-allowed transition text-xs"
                                >
                                    {loading ? (
                                        <span className="flex items-center justify-center gap-2">
                                            <div className="w-3 h-3 border-2 border-zinc-900 border-t-transparent rounded-full animate-spin" />
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
            </div>
        );
    }

    return (
        <div className="min-h-screen bg-black text-zinc-100 py-10 px-4 sm:px-6 lg:px-8">
            <div className="max-w-4xl mx-auto p-6 sm:p-8 bg-zinc-950 border border-zinc-900 rounded-2xl shadow-lg space-y-6">
                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 border-b border-zinc-900 pb-5">
                    <div>
                        <h2 className="text-2xl sm:text-3xl font-extrabold text-white tracking-tight">Create New Contest</h2>
                        <p className="text-xs text-zinc-400 mt-1">
                            Set up a new competitive programming contest with problems and time limits
                        </p>
                    </div>
                    <button
                        type="button"
                        onClick={() => navigator(-1)}
                        className="px-4 py-2 bg-zinc-900 hover:bg-zinc-800 text-zinc-300 border border-zinc-800 rounded-xl text-xs font-medium transition"
                    >
                        &larr; Back to Dashboard
                    </button>
                </div>

                {error && (
                    <div className="p-4 bg-zinc-900 border border-zinc-800 rounded-xl text-xs text-zinc-300">
                        <p className="font-semibold text-white">Error</p>
                        <p className="mt-0.5">{error}</p>
                    </div>
                )}

                <form onSubmit={handleContestSubmit} className="space-y-6">
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                            <label className="block text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2">
                                Contest Title *
                            </label>
                            <input
                                type="text"
                                name="title"
                                value={form.title}
                                onChange={handleChange}
                                required
                                placeholder="e.g., Weekly Contest #42"
                                className="w-full bg-zinc-900 border border-zinc-800 rounded-xl px-3.5 py-2 text-white placeholder-zinc-600 focus:border-zinc-500 outline-none transition text-xs"
                            />
                        </div>

                        <div>
                            <label className="block text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2">
                                Slug (URL-friendly) *
                            </label>
                            <input
                                type="text"
                                name="slug"
                                value={form.slug}
                                onChange={handleChange}
                                required
                                placeholder="e.g., weekly-contest-42"
                                className="w-full bg-zinc-900 border border-zinc-800 rounded-xl px-3.5 py-2 text-white placeholder-zinc-600 focus:border-zinc-500 outline-none transition text-xs font-mono"
                            />
                        </div>
                    </div>

                    <div>
                        <label className="block text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2">
                            Description
                        </label>
                        <textarea
                            name="description"
                            value={form.description}
                            onChange={handleChange}
                            placeholder="Describe your contest, rules, scoring system, etc."
                            className="w-full bg-zinc-900 border border-zinc-800 rounded-xl px-3.5 py-2.5 text-white placeholder-zinc-600 focus:border-zinc-500 outline-none transition text-xs leading-relaxed"
                            rows={4}
                        />
                    </div>

                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                            <label className="block text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2">
                                Start Time *
                            </label>
                            <input
                                type="datetime-local"
                                name="start_time"
                                value={form.start_time}
                                onChange={(e) => handleStartTimeChange(e.target.value)}
                                required
                                className="w-full bg-zinc-900 border border-zinc-800 rounded-xl px-3.5 py-2 text-white placeholder-zinc-600 focus:border-zinc-500 outline-none transition text-xs font-mono"
                            />
                        </div>

                        <div>
                            <label className="block text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2">
                                Quick Duration (hours)
                            </label>
                            <div className="flex flex-wrap gap-1.5">
                                {[1, 2, 3, 4, 6, 12, 24].map((hours) => (
                                    <button
                                        key={hours}
                                        type="button"
                                        onClick={() => handleDurationChange(hours)}
                                        className={`px-3 py-1.5 rounded-lg text-xs font-semibold transition ${
                                            durationHours === hours
                                                ? "bg-zinc-800 text-white border border-zinc-700 shadow-sm"
                                                : "bg-zinc-950 text-zinc-400 border border-zinc-900 hover:bg-zinc-900 hover:text-zinc-200"
                                        }`}
                                    >
                                        {hours}h
                                    </button>
                                ))}
                            </div>
                        </div>
                    </div>

                    <div>
                        <label className="block text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2">
                            End Time *
                        </label>
                        <input
                            type="datetime-local"
                            name="end_time"
                            value={form.end_time}
                            onChange={handleChange}
                            required
                            className="w-full bg-zinc-900 border border-zinc-800 rounded-xl px-3.5 py-2 text-white placeholder-zinc-600 focus:border-zinc-500 outline-none transition text-xs font-mono"
                        />
                        {form.start_time && form.end_time && (
                            <p className="text-xs text-zinc-400 font-mono mt-1.5">
                                ⏱️ Total Duration: {calculateDuration(form.start_time, form.end_time)}
                            </p>
                        )}
                    </div>

                    <div className="flex items-center gap-3 p-4 bg-zinc-900/60 border border-zinc-800 rounded-xl">
                        <input
                            type="checkbox"
                            name="is_public"
                            id="is_public"
                            checked={form.is_public}
                            onChange={handleChange}
                            className="w-4 h-4 rounded bg-zinc-900 border-zinc-700 text-zinc-300 focus:ring-0 cursor-pointer"
                        />
                        <label htmlFor="is_public" className="text-xs font-medium text-zinc-300 cursor-pointer">
                            <span className="block text-white font-medium">Make this contest publicly visible</span>
                            <span className="text-zinc-500 text-[11px]">
                                Public contests appear in the public arena and are accessible to all participants.
                            </span>
                        </label>
                    </div>

                    <div className="flex gap-3 pt-4 border-t border-zinc-900">
                        <button
                            type="button"
                            onClick={() => navigator(-1)}
                            className="px-5 py-2.5 bg-zinc-900 hover:bg-zinc-800 text-zinc-300 border border-zinc-800 rounded-xl font-medium transition text-xs"
                        >
                            Cancel
                        </button>
                        <button
                            type="submit"
                            disabled={loading}
                            className="flex-1 bg-zinc-100 hover:bg-white text-zinc-950 font-semibold py-2.5 px-6 rounded-xl shadow-sm disabled:opacity-50 disabled:cursor-not-allowed transition text-xs"
                        >
                            {loading ? (
                                <span className="flex items-center justify-center gap-2">
                                    <div className="w-3 h-3 border-2 border-zinc-900 border-t-transparent rounded-full animate-spin" />
                                    Creating Contest...
                                </span>
                            ) : (
                                "Create Contest & Add Problems"
                            )}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    );
};

export default CreateContest;

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
