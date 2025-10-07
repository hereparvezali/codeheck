import { useNavigate } from "react-router-dom";
import { useAuth } from "../utils/contexts/authcontext";
import { useState, type FormEvent, type ChangeEvent, useEffect } from "react";
import type { ProblemPayload } from "./problem";

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
}

const CreateProblem = () => {
    const navigate = useNavigate();
    const { authfetch } = useAuth();

    const [caseCount, setCaseCount] = useState(0);
    const [formData, setFormData] = useState<CreateProblemPayload>({
        title: "",
        slug: "",
        statement: "",
        input_spec: "",
        output_spec: "",
        sample_inputs: "",
        time_limit: 1000,
        memory_limit: 256,
        difficulty: "easy",
    });
    const [cases, setCases] = useState<Case[]>([]);
    const [loading, setLoading] = useState(false);
    const [testcaseloading, setTestcaseloading] = useState(false);
    const [error, setError] = useState<string | null>(null);

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
            [name]: type === "number" ? Number(value) : value,
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
        setLoading(true);
        setError(null);

        authfetch("/problem", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(formData),
        })
            .then(async (res) => {
                if (!res.ok) {
                    if (res.status === 401) navigate("/signin");
                    const text = await res.text();
                    throw new Error(text || "Failed to create problem");
                }
                return res.json();
            })
            .then(async (data: ProblemPayload) => {
                await handleCaseSubmit(data.id);
            })
            .catch((err) => setError(err.message))
            .finally(() => {
                setLoading(false);
                navigate(`/problems/${formData.slug}`);
            });
    };

    const handleCaseSubmit = async (problem_id: number) => {
        setTestcaseloading(true);
        console.log(JSON.stringify({ problem_id: problem_id, cases: cases }));
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
                navigate(`/problems/${formData.slug}`);
            })
            .catch((e) => console.error(e))
            .finally(() => setTestcaseloading(false));
    };

    return (
        <div className="max-w-2xl mx-auto p-6 bg-white shadow-md rounded-lg">
            <h2 className="text-2xl font-bold mb-2">Create New Problem</h2>
            {error && <p className="text-red-600 mb-4">{error}</p>}

            <form onSubmit={handleSubmit} className="space-y-4">
                {/* Slug */}
                <div>
                    <label className="block font-medium">Slug *</label>
                    <input
                        type="text"
                        name="slug"
                        value={formData.slug}
                        onChange={handleChange}
                        required
                        className="w-full border rounded px-3 py-2"
                    />
                </div>

                {/* Title */}
                <div>
                    <label className="block font-medium">Title *</label>
                    <input
                        type="text"
                        name="title"
                        value={formData.title}
                        onChange={handleChange}
                        required
                        className="w-full border rounded px-3 py-2"
                    />
                </div>

                {/* Statement */}
                <div>
                    <label className="block font-medium">
                        Problem Statement
                    </label>
                    <textarea
                        name="statement"
                        value={formData.statement}
                        onChange={handleChange}
                        className="w-full border rounded px-3 py-2"
                    />
                </div>

                {/* Input Spec */}
                <div>
                    <label className="block font-medium">
                        Input Specification
                    </label>
                    <textarea
                        name="input_spec"
                        value={formData.input_spec}
                        onChange={handleChange}
                        className="w-full border rounded px-3 py-2"
                    />
                </div>

                {/* Output Spec */}
                <div>
                    <label className="block font-medium">
                        Output Specification
                    </label>
                    <textarea
                        name="output_spec"
                        value={formData.output_spec}
                        onChange={handleChange}
                        className="w-full border rounded px-3 py-2"
                    />
                </div>

                {/* Sample Inputs */}
                <div>
                    <label className="block font-medium">Sample Inputs</label>
                    <textarea
                        name="sample_inputs"
                        value={formData.sample_inputs}
                        onChange={handleChange}
                        className="w-full border rounded px-3 py-2"
                    />
                </div>
                {/* Sample Oututs */}
                <div>
                    <label className="block font-medium">Sample Outputs</label>
                    <textarea
                        name="sample_outputs"
                        value={formData.sample_outputs}
                        onChange={handleChange}
                        className="w-full border rounded px-3 py-2"
                    />
                </div>

                {/* Time & Memory Limit */}
                <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                    <div>
                        <label className="block font-medium">
                            Time Limit (ms) *
                        </label>
                        <input
                            type="number"
                            name="time_limit"
                            value={formData.time_limit}
                            onChange={handleChange}
                            required
                            className="w-full border rounded px-3 py-2"
                        />
                    </div>
                    <div>
                        <label className="block font-medium">
                            Memory Limit (MB) *
                        </label>
                        <input
                            type="number"
                            name="memory_limit"
                            value={formData.memory_limit}
                            onChange={handleChange}
                            required
                            className="w-full border rounded px-3 py-2"
                        />
                    </div>
                </div>

                {/* Difficulty */}
                <div>
                    <label className="block font-medium">Difficulty</label>
                    <select
                        name="difficulty"
                        value={formData.difficulty}
                        onChange={handleChange}
                        className="w-full border rounded px-3 py-2"
                    >
                        <option value="easy">Easy</option>
                        <option value="medium">Medium</option>
                        <option value="hard">Hard</option>
                    </select>
                </div>

                {/* Test Cases */}
                <div>
                    <div className="flex items-center gap-2 mb-2">
                        <button
                            type="button"
                            onClick={() =>
                                setCaseCount((prev) => Math.max(0, prev - 1))
                            }
                            className="px-3 py-1 bg-gray-300 rounded"
                        >
                            -
                        </button>
                        <span>{caseCount} test case(s)</span>
                        <button
                            type="button"
                            onClick={() => setCaseCount((prev) => prev + 1)}
                            className="px-3 py-1 bg-gray-300 rounded"
                        >
                            +
                        </button>
                    </div>

                    {cases.map((c, idx) => (
                        <div
                            key={idx}
                            className="grid grid-cols-1 sm:grid-cols-2 gap-2 mb-2"
                        >
                            <input
                                type="text"
                                placeholder={`Input #${idx + 1}`}
                                value={c.input}
                                onChange={(e) =>
                                    handleCaseChange(
                                        idx,
                                        "input",
                                        e.target.value,
                                    )
                                }
                                className="w-full border rounded px-2 py-1"
                            />
                            <input
                                type="text"
                                placeholder={`Output #${idx + 1}`}
                                value={c.output}
                                onChange={(e) =>
                                    handleCaseChange(
                                        idx,
                                        "output",
                                        e.target.value,
                                    )
                                }
                                className="w-full border rounded px-2 py-1"
                            />
                        </div>
                    ))}
                </div>

                {/* Submit */}
                <button
                    type="submit"
                    disabled={loading || testcaseloading}
                    className="w-full bg-blue-600 text-white py-2 rounded hover:bg-blue-700 disabled:opacity-50"
                >
                    {loading ? "Creating..." : "Create Problem"}
                </button>
            </form>
        </div>
    );
};

export default CreateProblem;
