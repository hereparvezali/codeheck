import { useNavigate, useParams } from "react-router-dom";
import { useAuth } from "../utils/contexts/authcontext";
import { useState, type FormEvent, type ChangeEvent, useEffect } from "react";
import type { ProblemPayload } from "../problems/problem";

interface Case {
    input: string;
    output: string;
}

interface UpdateProblemPayload {
    title?: string;
    slug?: string;
    statement?: string;
    input_spec?: string;
    output_spec?: string;
    sample_inputs?: string;
    sample_outputs?: string;
    time_limit?: number;
    memory_limit?: number;
    difficulty?: string;
    is_public?: boolean;
}

const EditProblem = () => {
    const { id } = useParams<{ id: string }>();
    const navigate = useNavigate();
    const { authfetch } = useAuth();

    const [problemId, setProblemId] = useState<number | null>(null);
    const [caseCount, setCaseCount] = useState(0);
    const [formData, setFormData] = useState<UpdateProblemPayload>({
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
    const [fetchLoading, setFetchLoading] = useState(true);
    const [testcaseloading, setTestcaseloading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [activeTab, setActiveTab] = useState<"basic" | "samples" | "testcases">("basic");

    // Fetch existing problem data
    useEffect(() => {
        if (!id) return;
        
        setFetchLoading(true);
        authfetch(`/problem?id=${id}`, {
            method: "GET",
        })
            .then(async (res) => {
                if (!res.ok) {
                    if (res.status === 401) navigate("/signin");
                    throw new Error(await res.text());
                }
                return res.json();
            })
            .then((data: ProblemPayload) => {
                setProblemId(data.id);
                setFormData({
                    title: data.title,
                    slug: data.slug,
                    statement: data.statement || "",
                    input_spec: data.input_spec || "",
                    output_spec: data.output_spec || "",
                    sample_inputs: typeof data.sample_inputs === 'string' 
                        ? data.sample_inputs 
                        : JSON.stringify(data.sample_inputs, null, 2),
                    sample_outputs: typeof data.sample_outputs === 'string' 
                        ? data.sample_outputs 
                        : JSON.stringify(data.sample_outputs, null, 2),
                    time_limit: data.time_limit,
                    memory_limit: data.memory_limit,
                    difficulty: data.difficulty || "easy",
                    is_public: data.is_public,
                });
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

    // Sync cases array length with caseCount
    useEffect(() => {
        setCases((prev) => {
            if (caseCount > prev.length) {
                return [
                    ...prev,
                    ...Array(caseCount - prev.length).fill({
                        input: "",
                        output: "",
                    }),
                ];
            } else {
                return prev.slice(0, caseCount);
            }
        });
    }, [caseCount]);

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

    const handleSubmit = (e: FormEvent) => {
        e.preventDefault();
        if (!problemId) return;

        setLoading(true);
        setError(null);

        authfetch(`/problem/${problemId}`, {
            method: "PUT",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(formData),
        })
            .then(async (res) => {
                if (!res.ok) {
                    if (res.status === 401) navigate("/signin");
                    const text = await res.text();
                    throw new Error(text || "Failed to update problem");
                }
                return res.json();
            })
            .then(async (data: ProblemPayload) => {
                if (cases.length > 0) {
                    await handleCaseSubmit(data.id);
                } else {
                    navigate(`/problems/${formData.slug || id}`);
                }
            })
            .catch((err) => setError(err.message))
            .finally(() => {
                setLoading(false);
            });
    };

    const handleCaseSubmit = async (problem_id: number) => {
        setTestcaseloading(true);
        authfetch(`/problem/testcases`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ problem_id: problem_id, cases: cases }),
        })
            .then(async (res) => {
                if (!res.ok) {
                    if (res.status === 401) navigate("/signin");
                    throw new Error(await res.text());
                }
                return res.json();
            })
            .then(() => {
                navigate(`/problems/${formData.slug || id}`);
            })
            .catch((e) => {
                console.error(e);
                setError("Failed to add test cases");
            })
            .finally(() => setTestcaseloading(false));
    };

    const removeCaseAt = (index: number) => {
        setCases((prev) => prev.filter((_, i) => i !== index));
        setCaseCount((prev) => prev - 1);
    };

    const addCase = () => {
        setCaseCount((prev) => prev + 1);
    };

    if (fetchLoading) {
        return (
            <div className="flex items-center justify-center min-h-screen">
                <div className="text-center">
                    <div className="animate-spin text-4xl mb-4">⏳</div>
                    <p className="text-gray-600">Loading problem data...</p>
                </div>
            </div>
        );
    }

    return (
        <div className="max-w-5xl mx-auto p-6 bg-white shadow-md rounded-lg my-6">
            <div className="mb-6">
                <h2 className="text-3xl font-bold mb-2">Edit Problem</h2>
                <p className="text-gray-600">Update the problem details and test cases</p>
            </div>
            
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
                    onClick={() => setActiveTab("basic")}
                    className={`px-4 py-2 font-medium transition-colors ${
                        activeTab === "basic"
                            ? "border-b-2 border-blue-600 text-blue-600"
                            : "text-gray-600 hover:text-gray-800"
                    }`}
                >
                    Basic Info
                </button>
                <button
                    type="button"
                    onClick={() => setActiveTab("samples")}
                    className={`px-4 py-2 font-medium transition-colors ${
                        activeTab === "samples"
                            ? "border-b-2 border-blue-600 text-blue-600"
                            : "text-gray-600 hover:text-gray-800"
                    }`}
                >
                    Samples & Specs
                </button>
                <button
                    type="button"
                    onClick={() => setActiveTab("testcases")}
                    className={`px-4 py-2 font-medium transition-colors ${
                        activeTab === "testcases"
                            ? "border-b-2 border-blue-600 text-blue-600"
                            : "text-gray-600 hover:text-gray-800"
                    }`}
                >
                    Test Cases ({caseCount})
                </button>
            </div>

            <form onSubmit={handleSubmit} className="space-y-6">
                {/* Basic Info Tab */}
                {activeTab === "basic" && (
                    <div className="space-y-4">
                        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div>
                                <label className="block font-medium text-gray-700 mb-2">
                                    Problem Title *
                                </label>
                                <input
                                    type="text"
                                    name="title"
                                    value={formData.title}
                                    onChange={handleChange}
                                    required
                                    placeholder="e.g., Two Sum"
                                    className="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                />
                            </div>

                            <div>
                                <label className="block font-medium text-gray-700 mb-2">
                                    Slug (URL-friendly) *
                                </label>
                                <input
                                    type="text"
                                    name="slug"
                                    value={formData.slug}
                                    onChange={handleChange}
                                    required
                                    placeholder="e.g., two-sum"
                                    className="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                />
                            </div>
                        </div>

                        <div>
                            <label className="block font-medium text-gray-700 mb-2">
                                Problem Statement *
                            </label>
                            <textarea
                                name="statement"
                                value={formData.statement}
                                onChange={handleChange}
                                rows={8}
                                placeholder="Describe the problem clearly. Include constraints, examples, and any important notes."
                                className="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 focus:border-transparent font-mono text-sm"
                            />
                            <p className="text-xs text-gray-500 mt-1">
                                Support for markdown formatting
                            </p>
                        </div>

                        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                            <div>
                                <label className="block font-medium text-gray-700 mb-2">
                                    Difficulty *
                                </label>
                                <select
                                    name="difficulty"
                                    value={formData.difficulty}
                                    onChange={handleChange}
                                    className="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                >
                                    <option value="easy">Easy</option>
                                    <option value="medium">Medium</option>
                                    <option value="hard">Hard</option>
                                </select>
                            </div>
                            <div>
                                <label className="block font-medium text-gray-700 mb-2">
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
                                    className="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                />
                            </div>
                            <div>
                                <label className="block font-medium text-gray-700 mb-2">
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
                                    className="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                />
                            </div>
                        </div>

                        <div className="flex items-center gap-2 p-4 bg-gray-50 rounded-lg">
                            <input
                                type="checkbox"
                                name="is_public"
                                id="is_public"
                                checked={formData.is_public}
                                onChange={handleChange}
                                className="w-4 h-4 text-blue-600 focus:ring-2 focus:ring-blue-500"
                            />
                            <label htmlFor="is_public" className="text-sm font-medium text-gray-700">
                                Make this problem publicly visible
                            </label>
                        </div>
                    </div>
                )}

                {/* Samples & Specs Tab */}
                {activeTab === "samples" && (
                    <div className="space-y-4">
                        <div>
                            <label className="block font-medium text-gray-700 mb-2">
                                Input Specification
                            </label>
                            <textarea
                                name="input_spec"
                                value={formData.input_spec}
                                onChange={handleChange}
                                rows={4}
                                placeholder="Describe the input format line by line"
                                className="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 focus:border-transparent font-mono text-sm"
                            />
                        </div>

                        <div>
                            <label className="block font-medium text-gray-700 mb-2">
                                Output Specification
                            </label>
                            <textarea
                                name="output_spec"
                                value={formData.output_spec}
                                onChange={handleChange}
                                rows={4}
                                placeholder="Describe the expected output format"
                                className="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 focus:border-transparent font-mono text-sm"
                            />
                        </div>

                        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div>
                                <label className="block font-medium text-gray-700 mb-2">
                                    Sample Input(s)
                                </label>
                                <textarea
                                    name="sample_inputs"
                                    value={formData.sample_inputs}
                                    onChange={handleChange}
                                    rows={6}
                                    placeholder="Example input for users to understand"
                                    className="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 focus:border-transparent font-mono text-sm"
                                />
                            </div>
                            <div>
                                <label className="block font-medium text-gray-700 mb-2">
                                    Sample Output(s)
                                </label>
                                <textarea
                                    name="sample_outputs"
                                    value={formData.sample_outputs}
                                    onChange={handleChange}
                                    rows={6}
                                    placeholder="Expected output for the sample input"
                                    className="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 focus:border-transparent font-mono text-sm"
                                />
                            </div>
                        </div>
                    </div>
                )}

                {/* Test Cases Tab */}
                {activeTab === "testcases" && (
                    <div className="space-y-4">
                        <div className="flex items-center justify-between p-4 bg-blue-50 border border-blue-200 rounded-lg">
                            <div>
                                <p className="font-medium text-blue-900">Add New Test Cases</p>
                                <p className="text-sm text-blue-700">
                                    Add additional test cases (existing ones won't be deleted)
                                </p>
                            </div>
                            <button
                                type="button"
                                onClick={addCase}
                                className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors flex items-center gap-2"
                            >
                                <span>+</span> Add Test Case
                            </button>
                        </div>

                        {cases.length === 0 ? (
                            <div className="text-center py-12 bg-gray-50 rounded-lg border-2 border-dashed border-gray-300">
                                <p className="text-gray-500">No new test cases added</p>
                                <p className="text-sm text-gray-400 mt-1">
                                    Click "Add Test Case" to add new test cases
                                </p>
                            </div>
                        ) : (
                            <div className="space-y-4">
                                {cases.map((c, idx) => (
                                    <div
                                        key={idx}
                                        className="border border-gray-300 rounded-lg p-4 bg-white hover:shadow-md transition-shadow"
                                    >
                                        <div className="flex items-center justify-between mb-3">
                                            <h4 className="font-semibold text-gray-700">
                                                New Test Case #{idx + 1}
                                            </h4>
                                            <button
                                                type="button"
                                                onClick={() => removeCaseAt(idx)}
                                                className="text-red-600 hover:text-red-800 text-sm font-medium"
                                            >
                                                Remove
                                            </button>
                                        </div>
                                        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                                            <div>
                                                <label className="block text-sm font-medium text-gray-600 mb-1">
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
                                                    className="w-full border border-gray-300 rounded px-3 py-2 font-mono text-sm focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                                />
                                            </div>
                                            <div>
                                                <label className="block text-sm font-medium text-gray-600 mb-1">
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
                                                    className="w-full border border-gray-300 rounded px-3 py-2 font-mono text-sm focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                                />
                                            </div>
                                        </div>
                                    </div>
                                ))}
                            </div>
                        )}
                    </div>
                )}

                {/* Submit Button */}
                <div className="flex gap-3 pt-4 border-t">
                    <button
                        type="button"
                        onClick={() => navigate(-1)}
                        className="px-6 py-3 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors font-medium"
                    >
                        Cancel
                    </button>
                    <button
                        type="submit"
                        disabled={loading || testcaseloading}
                        className="flex-1 bg-blue-600 text-white py-3 px-6 rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors font-medium"
                    >
                        {loading || testcaseloading ? (
                            <span className="flex items-center justify-center gap-2">
                                <span className="animate-spin">⏳</span>
                                {loading ? "Updating Problem..." : "Adding Test Cases..."}
                            </span>
                        ) : (
                            "Update Problem"
                        )}
                    </button>
                </div>
            </form>
        </div>
    );
};

export default EditProblem;