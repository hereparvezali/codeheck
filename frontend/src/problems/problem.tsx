import { useNavigate, useParams } from "react-router-dom";
import { useAuth } from "../utils/contexts/authcontext";
import { useEffect, useState } from "react";
import Editor from "@monaco-editor/react";

export interface ProblemPayload {
    id: number;
    title: string;
    slug: string;
    statement?: string;
    input_spec?: string;
    output_spec?: string;
    sample_inputs?: object;
    sample_outputs?: object;
    time_limit: number;
    memory_limit: number;
    difficulty?: string;
    is_public: boolean;
    created_at: string;
    author_id?: number;
}

export interface CreateSubmissionPayload {
    user_id: number;
    problem_id: number;
    language: string;
    code: string;
    contest_id?: number;
}

export interface RetrieveSubmissionsWithCursor {
    cursor?: number;
    submissions: RetrieveSubmissionsResponse[];
}
export interface RetrieveSubmissionsResponse {
    id: number;
    user_id: number;
    problem_id: number;
    language: string;
    code?: string;
    status: string;
    verdict?: string;
    time?: number;
    memory?: number;
    submitted_at: string;
    contest_id?: number;
}

const Problem = () => {
    const { id } = useParams<{ id: string }>();
    const navigate = useNavigate();
    const { authfetch, user } = useAuth();

    const [problem, setProblem] = useState<ProblemPayload | null>(null);
    const [language, setLanguage] = useState<string>("python");
    const [code, setCode] = useState<string>("");
    const [loading, setLoading] = useState(true);
    const [loaded_sub, setLoadedSub] = useState<
        RetrieveSubmissionsResponse | undefined
    >();

    const load_problem = async () => {
        setLoading(true);
        authfetch(`/problem?id=${id}`, {
            method: "GET",
        })
            .then(async (res) => {
                if (!res.ok) {
                    if (res.status === 401) navigate("/signin");
                    throw new Error(await res.text().catch(() => "Failed"));
                }
                return res.json();
            })
            .then((data) => setProblem(data))
            .catch(console.error)
            .finally(() => setLoading(false));
    };
    const load_status = async () => {
        if (!problem) return;
        authfetch(
            `/submissions?user_id=${user?.id}&problem_id=${problem?.id}&limit=1`,
        )
            .then(async (res) => {
                if (!res.ok) {
                    if (res.status === 401) {
                        navigate("/signin");
                        return;
                    }
                    throw new Error(await res.text());
                }
                return res.json();
            })
            .then((data: RetrieveSubmissionsWithCursor) => {
                if (data.submissions.length > 0) {
                    setLoadedSub(data.submissions[0]);
                }
            })
            .catch((e) => {
                console.error(e);
            });
    };
    useEffect(() => {
        load_problem();
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    useEffect(() => {
        load_status();
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [problem]);
    useEffect(() => {
        if (loaded_sub?.status != "pending") {
            return;
        }
        const interval = setInterval(() => {
            load_status();
        }, 500); // 5 seconds

        return () => clearInterval(interval);
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [loaded_sub]);

    const renderContent = (content?: string | object, fallback = "No data") => {
        if (!content) return fallback;
        return typeof content === "string"
            ? content
            : JSON.stringify(content, null, 2);
    };
    const handleSubmit = () => {
        if (!user || !problem) return;
        const payload: CreateSubmissionPayload = {
            user_id: user?.id,
            problem_id: problem.id,
            code: code,
            language: language,
        };
        console.log(payload);
        authfetch("/submission", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(payload),
        })
            .then(async (res) => {
                if (!res.ok) {
                    if (res.status === 401) {
                        navigate("/signin");
                        return;
                    }
                    throw new Error(await res.text());
                }
                return res.json();
            })
            .then((data) => {
                setLoadedSub(data);
                console.log(data);
            })
            .catch((e) => {
                console.error(e);
            });
    };

    if (loading) return <p className="p-4">Loading...</p>;
    if (!problem) return <p className="p-4">Problem not found</p>;
    return (
        <div className="flex h-screen">
            {/* Left panel: Problem details */}
            <div className="w-1/2 border-r overflow-y-auto p-6 bg-white">
                <h1 className="text-2xl font-bold mb-2">{problem.title}</h1>
                <p className="text-sm text-gray-500 mb-4">
                    Difficulty: {problem.difficulty || "N/A"}
                </p>

                <section className="mb-4">
                    <h2 className="font-semibold mb-1">Problem Statement</h2>
                    <pre className="bg-gray-50 p-3 rounded text-sm whitespace-pre-wrap">
                        {problem.statement || "N/A"}
                    </pre>
                </section>

                <section className="mb-4">
                    <h2 className="font-semibold mb-1">Input Specification</h2>
                    <pre className="bg-gray-50 p-3 rounded text-sm whitespace-pre-wrap">
                        {problem.input_spec || "N/A"}
                    </pre>
                </section>

                <section className="mb-4">
                    <h2 className="font-semibold mb-1">Output Specification</h2>
                    <pre className="bg-gray-50 p-3 rounded text-sm whitespace-pre-wrap">
                        {problem.output_spec || "N/A"}
                    </pre>
                </section>

                <section className="mb-4">
                    <h2 className="font-semibold mb-1">Sample Inputs</h2>
                    <pre className="bg-gray-50 p-3 rounded text-sm whitespace-pre-wrap">
                        {renderContent(problem.sample_inputs, "N/A")}
                    </pre>
                </section>

                <section className="mb-4">
                    <h2 className="font-semibold mb-1">Sample Outputs</h2>
                    <pre className="bg-gray-50 p-3 rounded text-sm whitespace-pre-wrap">
                        {renderContent(problem.sample_outputs, "N/A")}
                    </pre>
                </section>

                <section className="text-sm text-gray-500">
                    <p>
                        <strong>Time Limit:</strong> {problem.time_limit} ms
                    </p>
                    <p>
                        <strong>Memory Limit:</strong> {problem.memory_limit} MB
                    </p>
                    <p>
                        <strong>Visibility:</strong>{" "}
                        {problem.is_public ? "Public" : "Private"}
                    </p>
                </section>
            </div>

            {/* Right panel: Code editor + actions */}
            <div className="w-1/2 flex flex-col">
                <div className="flex items-center gap-2 p-3 border-b bg-gray-50">
                    <select
                        id="language"
                        onChange={(e) => {
                            setLanguage(e.target.value);
                        }}
                        className="border p-1 rounded"
                    >
                        <option>python</option>
                        <option>c++</option>
                        <option>java</option>
                        <option>rust</option>
                        <option>go</option>
                        <option>javascript</option>
                    </select>
                    <button className="bg-green-500 text-white px-3 py-1 rounded">
                        Run
                    </button>
                    <button
                        onClick={handleSubmit}
                        className={`${loaded_sub?.status == "pending" ? "bg-gray-500" : "bg-blue-500"} text-white px-3 py-1 rounded`}
                        disabled={
                            loaded_sub?.status == "pending" ? true : false
                        }
                    >
                        Submit
                    </button>
                    <button
                        className={`${
                            loaded_sub?.status === "AC"
                                ? "text-green-600"
                                : loaded_sub?.status === "WA"
                                  ? "text-red-600"
                                  : loaded_sub?.status === "TLE"
                                    ? "text-yellow-600"
                                    : loaded_sub?.status === "pending"
                                      ? "text-gray-500 animate-pulse"
                                      : "text-blue-500"
                        }`}
                    >
                        {loaded_sub?.status} {loaded_sub?.verdict}
                    </button>
                </div>

                <Editor
                    height="100%"
                    language={language}
                    value={code}
                    onChange={(value) => setCode(value || "")}
                    theme="vs-light"
                    options={{
                        minimap: { enabled: false },
                        fontSize: 14,
                        scrollBeyondLastLine: false,
                        automaticLayout: true,
                    }}
                />
            </div>
        </div>
    );
};

export default Problem;
