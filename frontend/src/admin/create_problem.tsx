import { useNavigate } from "react-router-dom";
import { useAuth } from "../utils/contexts/authcontext";
import { useState, type FormEvent, type ChangeEvent } from "react";
import type { ProblemPayload } from "../problems/problem";

interface Case {
    input: string;
    output: string;
}

interface CreateProblemPayload {
    title: string;
    slug: string;
    statement?: string;
    input_spec?: string;
    output_spec?: string;
    sample_inputs?: string;
    sample_outputs?: string;
    time_limit: number;
    memory_limit: number;
    difficulty?: string;
    is_public: boolean;
}

const CreateProblem = () => {
    const navigate = useNavigate();
    const { authfetch } = useAuth();

    const [formData, setFormData] = useState<CreateProblemPayload>({
        title: "",
        slug: "",
        statement: "",
        input_spec: "",
        output_spec: "",
        sample_inputs: "",
        sample_outputs: "",
        time_limit: 1000,
        memory_limit: 256,
        difficulty: "easy",
        is_public: false,
    });
    const [cases, setCases] = useState<Case[]>([]);
    const [loading, setLoading] = useState(false);
    const [testcaseloading, setTestcaseloading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [activeTab, setActiveTab] = useState<"basic" | "samples" | "testcases">("basic");

    const handleChange = (
        e: ChangeEvent<
            HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement
        >,
    ) => {
        const { name, value, type } = e.target;
        setFormData((prev) => ({
            ...prev,
            [name]:
                type === "number"
                    ? Number(value)
                    : type === "checkbox"
                    ? (e.target as HTMLInputElement).checked
                    : value,
        }));
    };

    const handleCaseChange = (
        index: number,
        field: "input" | "output",
        value: string,
    ) => {
        setCases((prev) => {
            const newCases = [...prev];
            newCases[index] = { ...newCases[index], [field]: value };
            return newCases;
        });
    };

    const addCase = () => {
        setCases((prev) => [...prev, { input: "", output: "" }]);
    };

    const removeCaseAt = (index: number) => {
        setCases((prev) => prev.filter((_, i) => i !== index));
    };

    const clearAllCases = () => {
        setCases([]);
    };

    const handleSubmit = async (e: FormEvent) => {
        e.preventDefault();
        setLoading(true);
        setError(null);

        try {
            const res = await authfetch("/problem", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(formData),
            });

            if (!res.ok) {
                if (res.status === 401) navigate("/signin");
                const text = await res.text();
                throw new Error(text || "Failed to create problem");
            }

            const data: ProblemPayload = await res.json();
            const validCases = cases.filter(
                (c) => c.input.trim() !== "" || c.output.trim() !== ""
            );

            if (validCases.length > 0) {
                await handleCaseSubmit(data.id, validCases);
            } else {
                navigate(`/problems/${formData.slug}`);
            }
        } catch (err: any) {
            setError(err.message || "An unexpected error occurred");
        } finally {
            setLoading(false);
        }
    };

    const handleCaseSubmit = async (problem_id: number, validCases: Case[]) => {
        setTestcaseloading(true);
        try {
            const res = await authfetch(`/problem/testcases`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({ problem_id, cases: validCases }),
            });

            if (!res.ok) {
                if (res.status === 401) navigate("/signin");
                throw new Error(await res.text());
            }

            navigate(`/problems/${formData.slug}`);
        } catch (e: any) {
            console.error("Failed to insert test cases:", e);
            setError(e.message || "Failed to insert test cases");
        } finally {
            setTestcaseloading(false);
        }
    };

    return (
        <div className="min-h-screen bg-black text-zinc-100 py-10 px-4 sm:px-6 lg:px-8">
            <div className="max-w-5xl mx-auto p-6 sm:p-8 bg-zinc-950 border border-zinc-900 rounded-2xl shadow-lg space-y-6">
                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 border-b border-zinc-900 pb-5">
                    <div>
                        <h2 className="text-2xl sm:text-3xl font-extrabold text-white tracking-tight">Create New Problem</h2>
                        <p className="text-xs text-zinc-400 mt-1">Fill in the details to create a competitive programming problem</p>
                    </div>
                    <button
                        type="button"
                        onClick={() => navigate(-1)}
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

                <div className="flex gap-2 border-b border-zinc-900 pb-2">
                    <button
                        type="button"
                        onClick={() => setActiveTab("basic")}
                        className={`px-3.5 py-1.5 text-xs font-semibold rounded-xl transition ${
                            activeTab === "basic"
                                ? "bg-zinc-800 text-white border border-zinc-700 shadow-sm"
                                : "text-zinc-400 hover:text-zinc-200"
                        }`}
                    >
                        Basic Info
                    </button>
                    <button
                        type="button"
                        onClick={() => setActiveTab("samples")}
                        className={`px-3.5 py-1.5 text-xs font-semibold rounded-xl transition ${
                            activeTab === "samples"
                                ? "bg-zinc-800 text-white border border-zinc-700 shadow-sm"
                                : "text-zinc-400 hover:text-zinc-200"
                        }`}
                    >
                        Samples & Specs
                    </button>
                    <button
                        type="button"
                        onClick={() => setActiveTab("testcases")}
                        className={`px-3.5 py-1.5 text-xs font-semibold rounded-xl transition ${
                            activeTab === "testcases"
                                ? "bg-zinc-800 text-white border border-zinc-700 shadow-sm"
                                : "text-zinc-400 hover:text-zinc-200"
                        }`}
                    >
                        Test Cases ({cases.length})
                    </button>
                </div>

                <form onSubmit={handleSubmit} className="space-y-6">
                    {activeTab === "basic" && (
                        <div className="space-y-4">
                            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                                <div>
                                    <label className="block text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2">
                                        Problem Title *
                                    </label>
                                    <input
                                        type="text"
                                        name="title"
                                        value={formData.title}
                                        onChange={handleChange}
                                        required
                                        placeholder="e.g., Two Sum"
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
                                        value={formData.slug}
                                        onChange={handleChange}
                                        required
                                        placeholder="e.g., two-sum"
                                        className="w-full bg-zinc-900 border border-zinc-800 rounded-xl px-3.5 py-2 text-white placeholder-zinc-600 focus:border-zinc-500 outline-none transition text-xs font-mono"
                                    />
                                </div>
                            </div>

                            <div>
                                <label className="block text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2">
                                    Problem Statement *
                                </label>
                                <textarea
                                    name="statement"
                                    value={formData.statement}
                                    onChange={handleChange}
                                    rows={8}
                                    placeholder="Describe the problem clearly. Include constraints, examples, and any important notes (Markdown supported)."
                                    className="w-full bg-zinc-900 border border-zinc-800 rounded-xl px-3.5 py-2.5 text-white placeholder-zinc-600 focus:border-zinc-500 outline-none transition font-mono text-xs leading-relaxed"
                                />
                                <p className="text-[11px] text-zinc-500 mt-1">
                                    Markdown syntax is supported for mathematical formulations and code blocks.
                                </p>
                            </div>

                            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                                <div>
                                    <label className="block text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2">
                                        Difficulty *
                                    </label>
                                    <select
                                        name="difficulty"
                                        value={formData.difficulty}
                                        onChange={handleChange}
                                        className="w-full bg-zinc-900 border border-zinc-800 rounded-xl px-3 py-2 text-white focus:border-zinc-500 outline-none transition text-xs"
                                    >
                                        <option value="easy">Easy</option>
                                        <option value="medium">Medium</option>
                                        <option value="hard">Hard</option>
                                    </select>
                                </div>
                                <div>
                                    <label className="block text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2">
                                        Time Limit (ms) *
                                    </label>
                                    <input
                                        type="number"
                                        name="time_limit"
                                        value={formData.time_limit}
                                        onChange={handleChange}
                                        required
                                        min="100"
                                        step="100"
                                        className="w-full bg-zinc-900 border border-zinc-800 rounded-xl px-3.5 py-2 text-white placeholder-zinc-600 focus:border-zinc-500 outline-none transition text-xs font-mono"
                                    />
                                </div>
                                <div>
                                    <label className="block text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2">
                                        Memory Limit (MB) *
                                    </label>
                                    <input
                                        type="number"
                                        name="memory_limit"
                                        value={formData.memory_limit}
                                        onChange={handleChange}
                                        required
                                        min="16"
                                        step="16"
                                        className="w-full bg-zinc-900 border border-zinc-800 rounded-xl px-3.5 py-2 text-white placeholder-zinc-600 focus:border-zinc-500 outline-none transition text-xs font-mono"
                                    />
                                </div>
                            </div>

                            <div className="flex items-center gap-3 p-4 bg-zinc-900/60 border border-zinc-800 rounded-xl">
                                <input
                                    type="checkbox"
                                    name="is_public"
                                    id="is_public"
                                    checked={formData.is_public}
                                    onChange={handleChange}
                                    className="w-4 h-4 rounded bg-zinc-900 border-zinc-700 text-zinc-300 focus:ring-0 cursor-pointer"
                                />
                                <label htmlFor="is_public" className="text-xs font-medium text-zinc-300 cursor-pointer">
                                    Make this problem publicly visible in the problem repository
                                </label>
                            </div>
                        </div>
                    )}

                    {activeTab === "samples" && (
                        <div className="space-y-4">
                            <div>
                                <label className="block text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2">
                                    Input Specification
                                </label>
                                <textarea
                                    name="input_spec"
                                    value={formData.input_spec}
                                    onChange={handleChange}
                                    rows={4}
                                    placeholder="Describe the input format line by line..."
                                    className="w-full bg-zinc-900 border border-zinc-800 rounded-xl px-3.5 py-2 text-white placeholder-zinc-600 focus:border-zinc-500 outline-none transition font-mono text-xs"
                                />
                            </div>

                            <div>
                                <label className="block text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2">
                                    Output Specification
                                </label>
                                <textarea
                                    name="output_spec"
                                    value={formData.output_spec}
                                    onChange={handleChange}
                                    rows={4}
                                    placeholder="Describe the expected output format..."
                                    className="w-full bg-zinc-900 border border-zinc-800 rounded-xl px-3.5 py-2 text-white placeholder-zinc-600 focus:border-zinc-500 outline-none transition font-mono text-xs"
                                />
                            </div>

                            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                                <div>
                                    <label className="block text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2">
                                        Sample Input(s)
                                    </label>
                                    <textarea
                                        name="sample_inputs"
                                        value={formData.sample_inputs}
                                        onChange={handleChange}
                                        rows={6}
                                        placeholder="Example input for users to understand..."
                                        className="w-full bg-zinc-900 border border-zinc-800 rounded-xl px-3.5 py-2 text-white placeholder-zinc-600 focus:border-zinc-500 outline-none transition font-mono text-xs"
                                    />
                                </div>
                                <div>
                                    <label className="block text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2">
                                        Sample Output(s)
                                    </label>
                                    <textarea
                                        name="sample_outputs"
                                        value={formData.sample_outputs}
                                        onChange={handleChange}
                                        rows={6}
                                        placeholder="Expected output corresponding to the sample input..."
                                        className="w-full bg-zinc-900 border border-zinc-800 rounded-xl px-3.5 py-2 text-white placeholder-zinc-600 focus:border-zinc-500 outline-none transition font-mono text-xs"
                                    />
                                </div>
                            </div>
                        </div>
                    )}

                    {activeTab === "testcases" && (
                        <div className="space-y-4">
                            <div className="flex items-center justify-between p-4 bg-zinc-900 border border-zinc-800 rounded-xl">
                                <div>
                                    <p className="font-semibold text-zinc-200 text-xs">Hidden Test Cases</p>
                                    <p className="text-[11px] text-zinc-500 mt-0.5">
                                        Add test cases to judge user submissions against the problem.
                                    </p>
                                </div>
                                <div className="flex items-center gap-2">
                                    {cases.length > 0 && (
                                        <button
                                            type="button"
                                            onClick={clearAllCases}
                                            className="px-3 py-1.5 bg-zinc-900 hover:bg-zinc-800 text-zinc-400 hover:text-red-400 rounded-xl text-xs font-medium transition border border-zinc-800"
                                        >
                                            Clear All
                                        </button>
                                    )}
                                    <button
                                        type="button"
                                        onClick={addCase}
                                        className="px-3.5 py-1.5 bg-zinc-100 hover:bg-white text-zinc-950 rounded-xl text-xs font-semibold transition flex items-center gap-1.5 shadow-sm"
                                    >
                                        <span>+</span> Add Test Case
                                    </button>
                                </div>
                            </div>

                            {cases.length === 0 ? (
                                <div className="text-center py-12 bg-zinc-950 rounded-xl border border-dashed border-zinc-800">
                                    <p className="text-zinc-400 font-medium text-xs">No test cases added yet</p>
                                    <p className="text-[11px] text-zinc-500 mt-1">
                                        Click "Add Test Case" to configure hidden inputs and outputs.
                                    </p>
                                </div>
                            ) : (
                                <div className="space-y-4">
                                    {cases.map((c, idx) => (
                                        <div
                                            key={idx}
                                            className="border border-zinc-800 rounded-xl p-4 bg-zinc-900/60 transition"
                                        >
                                            <div className="flex items-center justify-between mb-3">
                                                <h4 className="font-semibold text-zinc-200 text-xs">
                                                    Test Case #{idx + 1}
                                                </h4>
                                                <button
                                                    type="button"
                                                    onClick={() => removeCaseAt(idx)}
                                                    className="text-zinc-400 hover:text-red-400 text-xs font-semibold px-2.5 py-1 bg-zinc-900 hover:bg-zinc-800 rounded-lg border border-zinc-800 transition"
                                                >
                                                    Remove
                                                </button>
                                            </div>
                                            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                                                <div>
                                                    <label className="block text-xs font-medium text-zinc-400 uppercase tracking-wider mb-1">
                                                        Input
                                                    </label>
                                                    <textarea
                                                        placeholder={`Input for test case ${idx + 1}`}
                                                        value={c.input}
                                                        onChange={(e) =>
                                                            handleCaseChange(
                                                                idx,
                                                                "input",
                                                                e.target.value,
                                                            )
                                                        }
                                                        rows={4}
                                                        className="w-full bg-zinc-900 border border-zinc-800 rounded-lg px-3 py-2 font-mono text-xs text-white placeholder-zinc-600 focus:border-zinc-500 outline-none transition"
                                                    />
                                                </div>
                                                <div>
                                                    <label className="block text-xs font-medium text-zinc-400 uppercase tracking-wider mb-1">
                                                        Expected Output
                                                    </label>
                                                    <textarea
                                                        placeholder={`Output for test case ${idx + 1}`}
                                                        value={c.output}
                                                        onChange={(e) =>
                                                            handleCaseChange(
                                                                idx,
                                                                "output",
                                                                e.target.value,
                                                            )
                                                        }
                                                        rows={4}
                                                        className="w-full bg-zinc-900 border border-zinc-800 rounded-lg px-3 py-2 font-mono text-xs text-white placeholder-zinc-600 focus:border-zinc-500 outline-none transition"
                                                    />
                                                </div>
                                            </div>
                                        </div>
                                    ))}
                                </div>
                            )}
                        </div>
                    )}

                    <div className="flex gap-3 pt-4 border-t border-zinc-900">
                        <button
                            type="button"
                            onClick={() => navigate(-1)}
                            className="px-5 py-2.5 bg-zinc-900 hover:bg-zinc-800 text-zinc-300 border border-zinc-800 rounded-xl font-medium transition text-xs"
                        >
                            Cancel
                        </button>
                        <button
                            type="submit"
                            disabled={loading || testcaseloading}
                            className="flex-1 bg-zinc-100 hover:bg-white text-zinc-950 font-semibold py-2.5 px-6 rounded-xl shadow-sm disabled:opacity-50 disabled:cursor-not-allowed transition text-xs"
                        >
                            {loading || testcaseloading ? (
                                <span className="flex items-center justify-center gap-2">
                                    <div className="w-3 h-3 border-2 border-zinc-900 border-t-transparent rounded-full animate-spin" />
                                    {loading ? "Creating Problem..." : "Adding Test Cases..."}
                                </span>
                            ) : (
                                "Create Problem"
                            )}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    );
};

export default CreateProblem;
