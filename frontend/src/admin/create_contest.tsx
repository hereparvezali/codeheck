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
    const [selectedProblemIds, setSelectedProblemIds] = useState<number[]>([]);
    const [loadingProblems, setLoadingProblems] = useState(false);

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

    const handleProblemToggle = (problemId: number) => {
        setSelectedProblemIds((prev) =>
            prev.includes(problemId)
                ? prev.filter((id) => id !== problemId)
                : [...prev, problemId],
        );
    };

    const handleAddProblems = async () => {
        if (!createdContest) return;

        setLoading(true);
        setError(null);

        const problems: ProblemIdAndLabel[] = selectedProblemIds.map((id) => ({
            problem_id: id,
            label: undefined,
        }));

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
            <div className="max-w-4xl mx-auto mt-10 p-6 border rounded-lg shadow-md">
                <h2 className="text-xl font-bold mb-4">
                    Add Problems to "{createdContest.title}"
                </h2>

                {error && <p className="text-red-600 mb-4">{error}</p>}

                <p className="text-gray-600 mb-4">
                    Select problems to add to your contest. You can also skip this step and add problems later.
                </p>

                {loadingProblems ? (
                    <p className="text-center py-4">Loading problems...</p>
                ) : availableProblems.length === 0 ? (
                    <p className="text-center py-4 text-gray-500">
                        No problems available. Create some problems first.
                    </p>
                ) : (
                    <div className="space-y-2 mb-6 max-h-96 overflow-y-auto">
                        {availableProblems.map((problem) => (
                            <div
                                key={problem.id}
                                className="flex items-center gap-3 p-3 border rounded hover:bg-gray-50"
                            >
                                <input
                                    type="checkbox"
                                    checked={selectedProblemIds.includes(problem.id)}
                                    onChange={() => handleProblemToggle(problem.id)}
                                    className="w-4 h-4"
                                />
                                <div className="flex-1">
                                    <p className="font-medium">{problem.title}</p>
                                    <p className="text-sm text-gray-500">
                                        {problem.slug} • {problem.difficulty || "N/A"}
                                    </p>
                                </div>
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
                        ))}
                    </div>
                )}

                <div className="flex gap-3">
                    <button
                        onClick={handleAddProblems}
                        disabled={loading || selectedProblemIds.length === 0}
                        className="flex-1 bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        {loading
                            ? "Adding..."
                            : `Add ${selectedProblemIds.length} Problem(s)`}
                    </button>
                    <button
                        onClick={handleSkipProblems}
                        disabled={loading}
                        className="px-6 py-2 border border-gray-300 rounded hover:bg-gray-50 disabled:opacity-50"
                    >
                        Skip
                    </button>
                </div>
            </div>
        );
    }

    return (
        <div className="max-w-lg mx-auto mt-10 p-6 border rounded-lg shadow-md">
            <h2 className="text-xl font-bold mb-4">Create Contest</h2>

            {error && <p className="text-red-600 mb-2">{error}</p>}

            <form onSubmit={handleContestSubmit} className="space-y-4">
                <div>
                    <label className="block text-sm font-medium">Title *</label>
                    <input
                        type="text"
                        name="title"
                        value={form.title}
                        onChange={handleChange}
                        required
                        className="w-full p-2 border rounded"
                    />
                </div>

                <div>
                    <label className="block text-sm font-medium">Slug *</label>
                    <input
                        type="text"
                        name="slug"
                        value={form.slug}
                        onChange={handleChange}
                        required
                        className="w-full p-2 border rounded"
                    />
                </div>

                <div>
                    <label className="block text-sm font-medium">
                        Description
                    </label>
                    <textarea
                        name="description"
                        value={form.description}
                        onChange={handleChange}
                        className="w-full p-2 border rounded"
                        rows={4}
                    />
                </div>

                <div>
                    <label className="block text-sm font-medium">
                        Start Time *
                    </label>
                    <input
                        type="datetime-local"
                        name="start_time"
                        value={form.start_time}
                        onChange={handleChange}
                        required
                        className="w-full p-2 border rounded"
                    />
                </div>

                <div>
                    <label className="block text-sm font-medium">
                        End Time *
                    </label>
                    <input
                        type="datetime-local"
                        name="end_time"
                        value={form.end_time}
                        onChange={handleChange}
                        required
                        className="w-full p-2 border rounded"
                    />
                </div>

                <div className="flex items-center">
                    <input
                        type="checkbox"
                        name="is_public"
                        checked={form.is_public}
                        onChange={handleChange}
                        className="mr-2"
                    />
                    <label className="text-sm">Public Contest</label>
                </div>

                <button
                    type="submit"
                    disabled={loading}
                    className="w-full bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 disabled:opacity-50"
                >
                    {loading ? "Creating..." : "Create Contest"}
                </button>
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