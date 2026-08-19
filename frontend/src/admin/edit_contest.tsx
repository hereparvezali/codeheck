import React, { useState, useEffect } from "react";
import { useAuth } from "../utils/contexts/authcontext";
import { useNavigate, useParams } from "react-router-dom";
import { toLocalDatetimeInput, toUtcIsoString } from "../utils/helpers";

interface UpdateContestPayload {
    title?: string;
    slug?: string;
    description?: string;
    start_time?: string;
    end_time?: string;
    is_public?: boolean;
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

interface ContestProblem {
    id: number;
    problem_id?: number;
    title?: string;
    slug?: string;
    difficulty?: string;
    label?: string;
    problem?: Problem;
}

interface ProblemIdAndLabel {
    problem_id: number;
    label?: string;
}

const EditContest = () => {
    const { id } = useParams<{ id: string }>();
    const navigator = useNavigate();
    const { authfetch } = useAuth();

    const [form, setForm] = useState<UpdateContestPayload>({
        title: "",
        slug: "",
        description: "",
        start_time: "",
        end_time: "",
        is_public: false,
    });
    const [contestId, setContestId] = useState<number | null>(null);
    const [loading, setLoading] = useState(false);
    const [fetchLoading, setFetchLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [activeTab, setActiveTab] = useState<"details" | "problems">(
        "details",
    );


    const [contestProblems, setContestProblems] = useState<ContestProblem[]>(
        [],
    );
    const [availableProblems, setAvailableProblems] = useState<Problem[]>([]);
    const [selectedProblemIds, setSelectedProblemIds] = useState<number[]>([]);
    const [loadingProblems, setLoadingProblems] = useState(false);


    useEffect(() => {
        if (!id) return;

        setFetchLoading(true);
        authfetch(`/contest?id=${id}`, {
            method: "GET",
        })
            .then(async (res) => {
                if (!res.ok) {
                    if (res.status === 401) navigator("/signin");
                    throw new Error(await res.text());
                }
                return res.json();
            })
            .then((data: ContestResponse) => {
                setContestId(data.id);
                setForm({
                    title: data.title,
                    slug: data.slug,
                    description: data.description || "",
                    start_time: toLocalDatetimeInput(data.start_time),
                    end_time: toLocalDatetimeInput(data.end_time),
                    is_public: data.is_public,
                });


                return authfetch(`/contest/problems?id=${data.id}`);
            })
            .then(async (res) => {
                if (res && res.ok) {
                    const problems = await res.json();
                    setContestProblems(problems || []);
                }
            })
            .catch((err) => {
                setError(err.message);
                console.error(err);
            })
            .finally(() => {
                setFetchLoading(false);
            });

    }, [id]);


    useEffect(() => {
        if (activeTab === "problems" && availableProblems.length === 0) {
            fetchAvailableProblems();
        }

    }, [activeTab]);

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
            setError(
                e instanceof Error ? e.message : "Failed to load problems",
            );
        } finally {
            setLoadingProblems(false);
        }
    };

    const handleChange = (
        e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>,
    ) => {
        const { name, value, type } = e.target;

        setForm((prev: UpdateContestPayload) => ({
            ...prev,
            [name]:
                type === "checkbox"
                    ? (e.target as HTMLInputElement).checked
                    : value,
        }));
    };

    const handleContestSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        if (!contestId) return;

        setLoading(true);
        setError(null);

        const payload = {
            ...form,
            start_time: form.start_time
                ? toUtcIsoString(form.start_time)
                : undefined,
            end_time: form.end_time ? toUtcIsoString(form.end_time) : undefined,
        };

        authfetch(`/contest`, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ id: contestId, ...payload }),
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
            .then(async () => {
                navigator(`/contests/${contestId}`);
            })
            .catch((e) => {
                setError(e.message ?? "Something went wrong");
                console.error(e);
            })
            .finally(() => {
                setLoading(false);
            });
    };

    const handleProblemToggle = (problemId: number) => {
        setSelectedProblemIds((prev) =>
            prev.includes(problemId)
                ? prev.filter((id) => id !== problemId)
                : [...prev, problemId],
        );
    };

    const handleAddProblems = async () => {
        if (!contestId || selectedProblemIds.length === 0) return;

        setLoading(true);
        setError(null);
        let counter: number = contestProblems.length;
        const problems: ProblemIdAndLabel[] = selectedProblemIds.map((id) => {
            const label = String.fromCharCode(counter + 65);
            counter += 1;
            return { problem_id: id, label: label };
        });
        const payload: { id: number; problems: ProblemIdAndLabel[] } = {
            id: contestId,
            problems: problems,
        };
        try {
            const res = await authfetch("/contest/problems", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(payload),
            });

            if (!res.ok) {
                if (res.status === 401) {
                    navigator("/signin");
                    return;
                }
                const text = await res.text();
                throw new Error(text);
            }


            const problemsRes = await authfetch(
                `/contest/problems?id=${contestId}`,
            );
            if (problemsRes.ok) {
                const updatedProblems = await problemsRes.json();
                setContestProblems(updatedProblems || []);
            }

            setSelectedProblemIds([]);
            setError(null);
        } catch (e) {
            setError(e instanceof Error ? e.message : "Failed to add problems");
            console.error(e);
        } finally {
            setLoading(false);
        }
    };

    const handleRemoveProblem = async (contestProblemId: number) => {
        if (!contestId) return;

        setLoading(true);
        try {
            const res = await authfetch(
                `/contest/problems?problem_id=${contestProblemId}&contest_id=${contestId}`,
                {
                    method: "DELETE",
                },
            );

            if (!res.ok) {
                if (res.status === 401) {
                    navigator("/signin");
                    return;
                }
                throw new Error(await res.text());
            }


            setContestProblems((prev) =>
                prev.filter((p) => p.id !== contestProblemId),
            );
        } catch (e) {
            setError(
                e instanceof Error ? e.message : "Failed to remove problem",
            );
            console.error(e);
        } finally {
            setLoading(false);
        }
    };

    if (fetchLoading) {
        return (
            <div className="flex items-center justify-center min-h-screen bg-black text-zinc-100">
                <div className="text-center space-y-3">
                    <div className="w-7 h-7 border-2 border-zinc-400 border-t-transparent rounded-full animate-spin mx-auto" />
                    <p className="text-zinc-400 text-xs">Loading contest data...</p>
                </div>
            </div>
        );
    }

    return (
        <div className="min-h-screen bg-black text-zinc-100 py-10 px-4 sm:px-6 lg:px-8">
            <div className="max-w-4xl mx-auto p-6 sm:p-8 bg-zinc-950 border border-zinc-900 rounded-2xl shadow-lg space-y-6">
                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 border-b border-zinc-900 pb-5">
                    <div>
                        <h2 className="text-2xl sm:text-3xl font-extrabold text-white tracking-tight">Edit Contest</h2>
                        <p className="text-xs text-zinc-400 mt-1">
                            Update contest schedule, metadata, and manage problem pool
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

                {}
                <div className="flex gap-2 border-b border-zinc-900 pb-2">
                    <button
                        type="button"
                        onClick={() => setActiveTab("details")}
                        className={`px-3.5 py-1.5 text-xs font-semibold rounded-xl transition ${
                            activeTab === "details"
                                ? "bg-zinc-800 text-white border border-zinc-700 shadow-sm"
                                : "text-zinc-400 hover:text-zinc-200"
                        }`}
                    >
                        Contest Details
                    </button>
                    <button
                        type="button"
                        onClick={() => setActiveTab("problems")}
                        className={`px-3.5 py-1.5 text-xs font-semibold rounded-xl transition ${
                            activeTab === "problems"
                                ? "bg-zinc-800 text-white border border-zinc-700 shadow-sm"
                                : "text-zinc-400 hover:text-zinc-200"
                        }`}
                    >
                        Problems ({contestProblems.length})
                    </button>
                </div>

                {}
                {activeTab === "details" && (
                    <form onSubmit={handleContestSubmit} className="space-y-6">
                        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div>
                                <label className="block text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2">
                                    Title *
                                </label>
                                <input
                                    type="text"
                                    name="title"
                                    value={form.title}
                                    onChange={handleChange}
                                    required
                                    className="w-full bg-zinc-900 border border-zinc-800 rounded-xl px-3.5 py-2 text-white placeholder-zinc-600 focus:border-zinc-500 outline-none transition text-xs"
                                />
                            </div>

                            <div>
                                <label className="block text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2">
                                    Slug *
                                </label>
                                <input
                                    type="text"
                                    name="slug"
                                    value={form.slug}
                                    onChange={handleChange}
                                    required
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
                                    onChange={handleChange}
                                    required
                                    className="w-full bg-zinc-900 border border-zinc-800 rounded-xl px-3.5 py-2 text-white placeholder-zinc-600 focus:border-zinc-500 outline-none transition text-xs font-mono"
                                />
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
                            </div>
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
                            <label
                                htmlFor="is_public"
                                className="text-xs font-medium text-zinc-300 cursor-pointer"
                            >
                                <span className="block text-white font-medium">Public Contest</span>
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
                                {loading ? "Updating..." : "Update Contest"}
                            </button>
                        </div>
                    </form>
                )}

                {}
                {activeTab === "problems" && (
                    <div className="space-y-6">
                        {}
                        <div>
                            <h3 className="text-sm font-bold text-white mb-3">
                                Current Contest Problems ({contestProblems.length})
                            </h3>
                            {contestProblems.length === 0 ? (
                                <div className="text-center py-8 text-zinc-400 bg-zinc-950 rounded-xl border border-dashed border-zinc-800 text-xs">
                                    No problems added to this contest yet
                                </div>
                            ) : (
                                <div className="space-y-2">
                                    {contestProblems.map((cp) => (
                                        <div
                                            key={cp.id}
                                            className="flex items-center justify-between p-4 bg-zinc-900/60 border border-zinc-800 rounded-xl hover:border-zinc-700 transition"
                                        >
                                            <div>
                                                <p className="font-semibold text-white text-xs">
                                                    {cp.title ||
                                                        cp.problem?.title ||
                                                        "Problem #" + (cp.problem_id || cp.id)}
                                                </p>
                                                <p className="text-xs text-zinc-500 font-mono mt-0.5">
                                                    {cp.slug || cp.problem?.slug} •{" "}
                                                    <span className="capitalize text-zinc-400">{cp.difficulty || cp.problem?.difficulty || "N/A"}</span>
                                                    {cp.label && (
                                                        <span className="ml-2 px-1.5 py-0.5 bg-zinc-800 text-zinc-200 border border-zinc-700 rounded text-[11px] font-bold font-mono">
                                                            {cp.label}
                                                        </span>
                                                    )}
                                                </p>
                                            </div>
                                            <button
                                                onClick={() =>
                                                    handleRemoveProblem(cp.id)
                                                }
                                                disabled={loading}
                                                className="text-zinc-400 hover:text-white text-xs font-semibold px-2.5 py-1 bg-zinc-900 hover:bg-zinc-800 rounded-lg border border-zinc-800 transition disabled:opacity-50"
                                            >
                                                Remove
                                            </button>
                                        </div>
                                    ))}
                                </div>
                            )}
                        </div>

                        {}
                        <div className="pt-4 border-t border-zinc-900">
                            <h3 className="text-sm font-bold text-white mb-3">
                                Add More Problems to Contest
                            </h3>
                            {loadingProblems ? (
                                <p className="text-center py-6 text-zinc-500 text-xs">
                                    Loading available problems...
                                </p>
                            ) : availableProblems.length === 0 ? (
                                <p className="text-center py-6 text-zinc-500 text-xs bg-zinc-950 rounded-xl border border-zinc-900">
                                    No additional problems available. Create some problems first.
                                </p>
                            ) : (
                                <>
                                    <div className="space-y-2 mb-4 max-h-96 overflow-y-auto pr-1">
                                        {availableProblems
                                            .filter(
                                                (p) =>
                                                    !contestProblems.some(
                                                        (cp) =>
                                                            cp.problem_id === p.id,
                                                    ),
                                            )
                                            .map((problem) => (
                                                <div
                                                    key={problem.id}
                                                    onClick={() => handleProblemToggle(problem.id)}
                                                    className={`flex items-center gap-3 p-3.5 border rounded-xl transition cursor-pointer ${
                                                        selectedProblemIds.includes(problem.id)
                                                            ? "border-zinc-600 bg-zinc-900"
                                                            : "border-zinc-800 bg-zinc-950 hover:border-zinc-700"
                                                    }`}
                                                >
                                                    <input
                                                        type="checkbox"
                                                        checked={selectedProblemIds.includes(
                                                            problem.id,
                                                        )}
                                                        onChange={() => {}}
                                                        className="w-4 h-4 rounded bg-zinc-900 border-zinc-700 text-zinc-300 focus:ring-0 cursor-pointer"
                                                    />
                                                    <div className="flex-1">
                                                        <p className="font-semibold text-xs text-white">
                                                            {problem.title}
                                                        </p>
                                                        <p className="text-xs text-zinc-500 font-mono mt-0.5">
                                                            {problem.slug} •{" "}
                                                            <span className="capitalize text-zinc-400">{problem.difficulty || "N/A"}</span>
                                                        </p>
                                                    </div>
                                                    <span
                                                        className={`text-xs px-2 py-0.5 rounded border ${
                                                            problem.is_public
                                                                ? "bg-zinc-900 text-zinc-300 border-zinc-800"
                                                                : "bg-zinc-950 text-zinc-500 border-zinc-900"
                                                        }`}
                                                    >
                                                        {problem.is_public
                                                            ? "Public"
                                                            : "Private"}
                                                    </span>
                                                </div>
                                            ))}
                                    </div>

                                    <button
                                        onClick={handleAddProblems}
                                        disabled={
                                            loading ||
                                            selectedProblemIds.length === 0
                                        }
                                        className="w-full bg-zinc-100 hover:bg-white text-zinc-950 font-semibold py-2.5 px-4 rounded-xl shadow-sm disabled:opacity-50 disabled:cursor-not-allowed transition text-xs"
                                    >
                                        {loading
                                            ? "Adding Problems..."
                                            : `Add ${selectedProblemIds.length} Selected Problem(s)`}
                                    </button>
                                </>
                            )}
                        </div>
                    </div>
                )}
            </div>
        </div>
    );
};

export default EditContest;
