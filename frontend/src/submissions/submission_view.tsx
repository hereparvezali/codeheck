import { useEffect, useState } from "react";
import { useParams, Link, useNavigate } from "react-router-dom";
import Editor from "@monaco-editor/react";
import { useAuth } from "../utils/contexts/authcontext";
import { formatUtcToLocal, getStatusColor } from "../utils/helpers";

interface SubmissionDetail {
    id: number;
    user_id: number;
    problem_id: number;
    language: string;
    code: string;
    status: string;
    verdict?: string;
    time?: number;
    memory?: number;
    submitted_at: string;
    contest_id?: number;
}

export default function SubmissionView() {
    const { id } = useParams<{ id: string }>();
    const { authfetch } = useAuth();
    const navigate = useNavigate();
    const [submission, setSubmission] = useState<SubmissionDetail | null>(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const fetchSubmission = async () => {
            if (!id) return;
            setLoading(true);
            setError(null);
            try {
                const res = await authfetch(`/submission?id=${id}`);
                if (!res.ok) {
                    throw new Error(await res.text() || "Failed to load submission");
                }
                const data = await res.json();
                setSubmission(data);
            } catch (err) {
                setError((err as Error).message);
            } finally {
                setLoading(false);
            }
        };

        fetchSubmission();
    }, [id, authfetch]);

    const getMonacoLanguage = (lang: string) => {
        const l = lang.toLowerCase();
        if (l.includes("cpp") || l.includes("c++")) return "cpp";
        if (l.includes("py")) return "python";
        if (l.includes("java")) return "java";
        if (l.includes("rs") || l.includes("rust")) return "rust";
        if (l.includes("go")) return "go";
        if (l.includes("js") || l.includes("javascript")) return "javascript";
        return "plaintext";
    };

    if (loading) {
        return (
            <div className="min-h-screen bg-black flex items-center justify-center text-white">
                <div className="flex flex-col items-center gap-3">
                    <div className="w-7 h-7 border-2 border-zinc-400 border-t-transparent rounded-full animate-spin" />
                    <p className="text-zinc-400 text-sm">Loading submission #{id}...</p>
                </div>
            </div>
        );
    }

    if (error || !submission) {
        return (
            <div className="min-h-screen bg-black flex items-center justify-center p-6">
                <div className="bg-zinc-950 border border-zinc-900 p-8 rounded-2xl max-w-md w-full text-center">
                    <h2 className="text-xl font-bold text-zinc-200 mb-2">Submission Not Found</h2>
                    <p className="text-zinc-400 text-sm mb-6">{error || "Could not find submission details."}</p>
                    <button
                        onClick={() => navigate(-1)}
                        className="px-6 py-2.5 bg-zinc-100 hover:bg-white text-zinc-950 rounded-xl font-semibold transition shadow-sm"
                    >
                        &larr; Go Back
                    </button>
                </div>
            </div>
        );
    }

    return (
        <div className="min-h-screen bg-black text-zinc-100 py-10 px-4 sm:px-6 lg:px-8">
            <div className="max-w-5xl mx-auto space-y-6">
                <div className="bg-zinc-950 border border-zinc-900 rounded-2xl p-6 shadow-lg">
                    <div className="flex flex-wrap justify-between items-center gap-4 mb-6">
                        <div>
                            <span className="text-xs font-mono uppercase tracking-wider text-zinc-500">Submission</span>
                            <h1 className="text-2xl sm:text-3xl font-extrabold text-white flex items-center gap-3 mt-1">
                                #{submission.id}
                                <span className={`text-sm px-3 py-1 rounded-full ${getStatusColor(submission.status)}`}>
                                    {submission.status}
                                </span>
                            </h1>
                        </div>
                        <div className="flex items-center gap-3">
                            <Link
                                to={`/problems/${submission.problem_id}`}
                                className="px-4 py-2 bg-zinc-900 hover:bg-zinc-800 text-zinc-200 border border-zinc-800 rounded-xl text-xs font-semibold transition"
                            >
                                View Problem #{submission.problem_id} &rarr;
                            </Link>
                        </div>
                    </div>

                    <div className="grid grid-cols-2 sm:grid-cols-4 gap-4 p-4 bg-zinc-900/60 rounded-xl border border-zinc-800/80">
                        <div>
                            <div className="text-xs text-zinc-500 mb-1">Language</div>
                            <div className="font-mono font-semibold text-white uppercase text-sm">{submission.language}</div>
                        </div>
                        <div>
                            <div className="text-xs text-zinc-500 mb-1">Time</div>
                            <div className="font-mono text-sm text-zinc-200">
                                {submission.time !== undefined && submission.time !== null ? `${submission.time} ms` : "-"}
                            </div>
                        </div>
                        <div>
                            <div className="text-xs text-zinc-500 mb-1">Memory</div>
                            <div className="font-mono text-sm text-zinc-200">
                                {submission.memory !== undefined && submission.memory !== null ? `${submission.memory} KB` : "-"}
                            </div>
                        </div>
                        <div>
                            <div className="text-xs text-zinc-500 mb-1">Submitted</div>
                            <div className="text-xs text-zinc-300">
                                {formatUtcToLocal(submission.submitted_at)}
                            </div>
                        </div>
                    </div>

                    {submission.verdict && (
                        <div className="mt-4 p-4 rounded-xl bg-zinc-900 border border-zinc-800 font-mono text-xs text-zinc-300 overflow-x-auto whitespace-pre-wrap">
                            <div className="text-xs font-semibold text-zinc-400 mb-1">Verdict:</div>
                            {submission.verdict}
                        </div>
                    )}
                </div>

                <div className="bg-zinc-950 border border-zinc-900 rounded-2xl overflow-hidden shadow-lg">
                    <div className="px-6 py-3.5 bg-zinc-900 border-b border-zinc-800 flex justify-between items-center">
                        <span className="font-medium text-sm text-zinc-200 flex items-center gap-2">
                            <span>Source Code</span>
                            <span className="text-xs text-zinc-500 font-normal">({submission.code.split("\n").length} lines)</span>
                        </span>
                        <button
                            onClick={() => navigator.clipboard.writeText(submission.code)}
                            className="px-3 py-1 bg-zinc-800 hover:bg-zinc-700 text-xs text-zinc-300 rounded-lg border border-zinc-700 transition"
                        >
                            Copy Code
                        </button>
                    </div>
                    <div className="h-[500px]">
                        <Editor
                            height="100%"
                            theme="vs-dark"
                            language={getMonacoLanguage(submission.language)}
                            value={submission.code}
                            options={{
                                readOnly: true,
                                minimap: { enabled: false },
                                fontSize: 14,
                                lineNumbers: "on",
                                scrollBeyondLastLine: false,
                                automaticLayout: true,
                            }}
                        />
                    </div>
                </div>
            </div>
        </div>
    );
}

