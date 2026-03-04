import React, { useState, useEffect } from "react";
import { useAuth } from "../utils/contexts/authcontext";
import { useNavigate, useParams } from "react-router-dom";

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
    problem_id: number;
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

    // For managing problems
    const [contestProblems, setContestProblems] = useState<ContestProblem[]>(
        [],
    );
    const [availableProblems, setAvailableProblems] = useState<Problem[]>([]);
    const [selectedProblemIds, setSelectedProblemIds] = useState<number[]>([]);
    const [loadingProblems, setLoadingProblems] = useState(false);

    // Fetch existing contest data
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
                    start_time: toLocalDatetimeString(data.start_time),
                    end_time: toLocalDatetimeString(data.end_time),
                    is_public: data.is_public,
                });

                // Fetch contest problems
                return authfetch(`/contest/problems?contest_id=${data.id}`);
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
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [id]);

    // Fetch available problems when on problems tab
    useEffect(() => {
        if (activeTab === "problems" && availableProblems.length === 0) {
            fetchAvailableProblems();
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
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
                ? toUtcString(form.start_time)
                : undefined,
            end_time: form.end_time ? toUtcString(form.end_time) : undefined,
        };

        authfetch(`/contest?contest_id=${contestId}`, {
            method: "PUT",
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
                navigator(`/contests/${data.id}`);
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

            // Refresh contest problems
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

            // Remove from local state
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
            <div className="flex items-center justify-center min-h-screen">
                <div className="text-center">
                    <div className="animate-spin text-4xl mb-4">⏳</div>
                    <p className="text-gray-600">Loading contest data...</p>
                </div>
            </div>
        );
    }

    return (
        <div className="max-w-4xl mx-auto mt-10 p-6 border rounded-lg shadow-md">
            <h2 className="text-3xl font-bold mb-2">Edit Contest</h2>
            <p className="text-gray-600 mb-6">
                Update contest details and manage problems
            </p>

            {error && (
                <div className="mb-4 p-4 bg-red-50 border border-red-200 rounded-lg">
                    <p className="text-red-700 font-medium">Error</p>
                    <p className="text-red-600 text-sm">{error}</p>
                </div>
            )}

            {/* Tab Navigation */}
            <div className="flex gap-2 mb-6 border-b">
                <button
                    type="button"
                    onClick={() => setActiveTab("details")}
                    className={`px-4 py-2 font-medium transition-colors ${
                        activeTab === "details"
                            ? "border-b-2 border-blue-600 text-blue-600"
                            : "text-gray-600 hover:text-gray-800"
                    }`}
                >
                    Contest Details
                </button>
                <button
                    type="button"
                    onClick={() => setActiveTab("problems")}
                    className={`px-4 py-2 font-medium transition-colors ${
                        activeTab === "problems"
                            ? "border-b-2 border-blue-600 text-blue-600"
                            : "text-gray-600 hover:text-gray-800"
                    }`}
                >
                    Problems ({contestProblems.length})
                </button>
            </div>

            {/* Contest Details Tab */}
            {activeTab === "details" && (
                <form onSubmit={handleContestSubmit} className="space-y-4">
                    <div>
                        <label className="block text-sm font-medium mb-2">
                            Title *
                        </label>
                        <input
                            type="text"
                            name="title"
                            value={form.title}
                            onChange={handleChange}
                            required
                            className="w-full p-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        />
                    </div>

                    <div>
                        <label className="block text-sm font-medium mb-2">
                            Slug *
                        </label>
                        <input
                            type="text"
                            name="slug"
                            value={form.slug}
                            onChange={handleChange}
                            required
                            className="w-full p-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        />
                    </div>

                    <div>
                        <label className="block text-sm font-medium mb-2">
                            Description
                        </label>
                        <textarea
                            name="description"
                            value={form.description}
                            onChange={handleChange}
                            className="w-full p-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                            rows={4}
                        />
                    </div>

                    <div>
                        <label className="block text-sm font-medium mb-2">
                            Start Time *
                        </label>
                        <input
                            type="datetime-local"
                            name="start_time"
                            value={form.start_time}
                            onChange={handleChange}
                            required
                            className="w-full p-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        />
                    </div>

                    <div>
                        <label className="block text-sm font-medium mb-2">
                            End Time *
                        </label>
                        <input
                            type="datetime-local"
                            name="end_time"
                            value={form.end_time}
                            onChange={handleChange}
                            required
                            className="w-full p-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        />
                    </div>

                    <div className="flex items-center gap-2 p-4 bg-gray-50 rounded-lg">
                        <input
                            type="checkbox"
                            name="is_public"
                            id="is_public"
                            checked={form.is_public}
                            onChange={handleChange}
                            className="w-4 h-4 text-blue-600 focus:ring-2 focus:ring-blue-500"
                        />
                        <label
                            htmlFor="is_public"
                            className="text-sm font-medium"
                        >
                            Public Contest
                        </label>
                    </div>

                    <div className="flex gap-3 pt-4">
                        <button
                            type="button"
                            onClick={() => navigator(-1)}
                            className="px-6 py-2 border border-gray-300 rounded-lg hover:bg-gray-50"
                        >
                            Cancel
                        </button>
                        <button
                            type="submit"
                            disabled={loading}
                            className="flex-1 bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 disabled:opacity-50"
                        >
                            {loading ? "Updating..." : "Update Contest"}
                        </button>
                    </div>
                </form>
            )}

            {/* Problems Tab */}
            {activeTab === "problems" && (
                <div className="space-y-6">
                    {/* Current Problems */}
                    <div>
                        <h3 className="text-xl font-bold mb-3">
                            Current Problems
                        </h3>
                        {contestProblems.length === 0 ? (
                            <p className="text-center py-8 text-gray-500 bg-gray-50 rounded-lg">
                                No problems added to this contest yet
                            </p>
                        ) : (
                            <div className="space-y-2">
                                {contestProblems.map((cp) => (
                                    <div
                                        key={cp.id}
                                        className="flex items-center justify-between p-3 border rounded-lg hover:bg-gray-50"
                                    >
                                        <div>
                                            <p className="font-medium">
                                                {cp.problem?.title ||
                                                    "Unknown Problem"}
                                            </p>
                                            <p className="text-sm text-gray-500">
                                                {cp.problem?.slug} •{" "}
                                                {cp.problem?.difficulty ||
                                                    "N/A"}
                                                {cp.label &&
                                                    ` • Label: ${cp.label}`}
                                            </p>
                                        </div>
                                        <button
                                            onClick={() =>
                                                handleRemoveProblem(cp.id)
                                            }
                                            disabled={loading}
                                            className="text-red-600 hover:text-red-800 text-sm font-medium disabled:opacity-50"
                                        >
                                            Remove
                                        </button>
                                    </div>
                                ))}
                            </div>
                        )}
                    </div>

                    {/* Add New Problems */}
                    <div>
                        <h3 className="text-xl font-bold mb-3">
                            Add More Problems
                        </h3>
                        {loadingProblems ? (
                            <p className="text-center py-4">
                                Loading problems...
                            </p>
                        ) : availableProblems.length === 0 ? (
                            <p className="text-center py-4 text-gray-500">
                                No problems available. Create some problems
                                first.
                            </p>
                        ) : (
                            <>
                                <div className="space-y-2 mb-4 max-h-96 overflow-y-auto">
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
                                                className="flex items-center gap-3 p-3 border rounded hover:bg-gray-50"
                                            >
                                                <input
                                                    type="checkbox"
                                                    checked={selectedProblemIds.includes(
                                                        problem.id,
                                                    )}
                                                    onChange={() =>
                                                        handleProblemToggle(
                                                            problem.id,
                                                        )
                                                    }
                                                    className="w-4 h-4"
                                                />
                                                <div className="flex-1">
                                                    <p className="font-medium">
                                                        {problem.title}
                                                    </p>
                                                    <p className="text-sm text-gray-500">
                                                        {problem.slug} •{" "}
                                                        {problem.difficulty ||
                                                            "N/A"}
                                                    </p>
                                                </div>
                                                <span
                                                    className={`text-xs px-2 py-1 rounded ${
                                                        problem.is_public
                                                            ? "bg-green-100 text-green-800"
                                                            : "bg-gray-100 text-gray-800"
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
                                    className="w-full bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
                                >
                                    {loading
                                        ? "Adding..."
                                        : `Add ${selectedProblemIds.length} Problem(s)`}
                                </button>
                            </>
                        )}
                    </div>
                </div>
            )}
        </div>
    );
};

export default EditContest;

function toUtcString(datetimeLocal: string): string {
    const date = new Date(datetimeLocal);
    const year = date.getUTCFullYear();
    const month = String(date.getUTCMonth() + 1).padStart(2, "0");
    const day = String(date.getUTCDate()).padStart(2, "0");
    const hours = String(date.getUTCHours()).padStart(2, "0");
    const minutes = String(date.getUTCMinutes()).padStart(2, "0");

    return `${year}-${month}-${day}T${hours}:${minutes}:00`;
}

function toLocalDatetimeString(utcString: string): string {
    const date = new Date(utcString);
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, "0");
    const day = String(date.getDate()).padStart(2, "0");
    const hours = String(date.getHours()).padStart(2, "0");
    const minutes = String(date.getMinutes()).padStart(2, "0");

    return `${year}-${month}-${day}T${hours}:${minutes}`;
}
