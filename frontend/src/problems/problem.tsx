import { useNavigate, useParams, useSearchParams, Link } from "react-router-dom";
import { useAuth } from "../utils/contexts/authcontext";
import { useEffect, useState } from "react";
import Editor from "@monaco-editor/react";
import { getDifficultyColor, getStatusColor } from "../utils/helpers";
import { ViewSubmissions, type Submission } from "../components/view_submissions";

export interface ProblemPayload {
    id: number;
    title: string;
    slug: string;
    statement?: string;
    input_spec?: string;
    output_spec?: string;
    sample_inputs?: string | object;
    sample_outputs?: string | object;
    time_limit: number;
    memory_limit: number;
    difficulty?: string;
    is_public: boolean;
    created_at: string;
    author_id?: number;
}

const STARTER_TEMPLATES: Record<string, string> = {
    python: `import sys

def solve():
    input_data = sys.stdin.read().split()
    if not input_data:
        return
    pass

if __name__ == "__main__":
    solve()
`,
    "c++": `#include <iostream>
#include <vector>
#include <string>
#include <algorithm>

using namespace std;

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    return 0;
}
`,
    java: `import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
    }
}
`,
    rust: `use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
}
`,
    go: `package main

import "fmt"

func main() {
}
`,
    javascript: `const fs = require('fs');

function solve() {
    const input = fs.readFileSync(0, 'utf-8').trim();
    if (!input) return;
}

solve();
`,
};

export default function Problem() {
    const { id } = useParams<{ id: string }>();
    const [searchParams] = useSearchParams();
    const contest_id = searchParams.get("contest_id") ? parseInt(searchParams.get("contest_id")!) : undefined;
    const navigate = useNavigate();
    const { authfetch, user } = useAuth();

    const [problem, setProblem] = useState<ProblemPayload | null>(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [language, setLanguage] = useState<string>("c++");
    const [code, setCode] = useState<string>(STARTER_TEMPLATES["c++"]);
    const [submitting, setSubmitting] = useState<boolean>(false);
    const [currentSub, setCurrentSub] = useState<Submission | null>(null);
    const [activeTab, setActiveTab] = useState<"statement" | "submissions">("statement");
    const [recentSubmissions, setRecentSubmissions] = useState<Submission[]>([]);

    const [editorTheme, setEditorTheme] = useState<string>("vs-dark");
    const [fontSize, setFontSize] = useState<number>(14);
    const [wordWrap, setWordWrap] = useState<"on" | "off">("on");

    useEffect(() => {
        const loadProblem = async () => {
            if (!id) return;
            setLoading(true);
            try {
                const param = isNaN(Number(id)) ? `slug=${id}` : `id=${id}`;
                const res = await authfetch(`/problem?${param}`);
                if (!res.ok) {
                    let errMsg = "Problem not found";
                    try {
                        const jsonErr = await res.json();
                        if (jsonErr.message) errMsg = jsonErr.message;
                    } catch {
                        const text = await res.text();
                        if (text) errMsg = text;
                    }
                    throw new Error(errMsg);
                }
                const data = await res.json();
                setProblem(data);
            } catch (err) {
                setError((err as Error).message);
            } finally {
                setLoading(false);
            }
        };

        loadProblem();
    }, [id, authfetch]);

    const loadSubmissions = async () => {
        if (!problem || !user) return;
        try {
            const res = await authfetch(`/submissions?user_id=${user.id}&problem_id=${problem.id}&limit=10`);
            if (res.ok) {
                const data = await res.json();
                setRecentSubmissions(data.submissions || []);
            }
        } catch {

        }
    };

    useEffect(() => {
        if (activeTab === "submissions") {
            loadSubmissions();
        }
    }, [activeTab, problem]);

    const handleLanguageChange = (newLang: string) => {
        const currentStarter = STARTER_TEMPLATES[language];
        if (code === "" || code === currentStarter) {
            setCode(STARTER_TEMPLATES[newLang] || "");
        }
        setLanguage(newLang);
    };

    const handleResetCode = () => {
        if (window.confirm("Reset editor to default template?")) {
            setCode(STARTER_TEMPLATES[language] || "");
        }
    };

    useEffect(() => {
        if (!currentSub || currentSub.status.toUpperCase() !== "PENDING") {
            return;
        }

        let isMounted = true;
        const baseUrl = import.meta.env.VITE_BASE || "http://localhost:8000/api";
        const tokenParam = user?.access_token ? `&token=${encodeURIComponent(user.access_token)}` : "";
        const wsUrl = baseUrl.replace(/^http/, "ws") + `/submission/ws?id=${currentSub.id}${tokenParam}`;

        let ws: WebSocket | null = null;
        let pollTimer: ReturnType<typeof setInterval> | null = null;

        const applyUpdate = (update: {
            submission_id?: number;
            status: string;
            verdict?: string | null;
            time?: number | null;
            memory?: number | null;
        }) => {
            if (!isMounted) return;
            setCurrentSub((prev) => prev ? {
                ...prev,
                status: update.status,
                verdict: update.verdict ?? prev.verdict,
                time: update.time ?? prev.time,
                memory: update.memory ?? prev.memory,
            } : null);

            if (update.status.toUpperCase() !== "PENDING") {
                setSubmitting(false);
                loadSubmissions();
                if (pollTimer) clearInterval(pollTimer);
                if (ws && ws.readyState === WebSocket.OPEN) ws.close();
            }
        };

        const pollOnce = async () => {
            if (!isMounted) return;
            try {
                const res = await authfetch(`/submission?id=${currentSub.id}`);
                if (res.ok) {
                    const data: Submission = await res.json();
                    if (data.status.toUpperCase() !== "PENDING") {
                        applyUpdate(data);
                    }
                }
            } catch {

            }
        };

        try {
            ws = new WebSocket(wsUrl);
            ws.onmessage = (event) => {
                try {
                    const data = JSON.parse(event.data);
                    applyUpdate(data);
                } catch {
                    // ignore
                }
            };
            ws.onerror = () => {
                if (!pollTimer && currentSub?.status.toUpperCase() === "PENDING") {
                    pollTimer = setInterval(pollOnce, 2000);
                }
            };
            ws.onclose = () => {
                if (isMounted && currentSub?.status.toUpperCase() === "PENDING" && !pollTimer) {
                    pollTimer = setInterval(pollOnce, 2000);
                }
            };
        } catch {
            pollTimer = setInterval(pollOnce, 2000);
        }

        return () => {
            isMounted = false;
            if (ws) ws.close();
            if (pollTimer) clearInterval(pollTimer);
        };
    }, [currentSub?.id, currentSub?.status, user?.access_token]);

    const handleSubmit = async () => {
        if (!user) {
            navigate("/signin");
            return;
        }
        if (!problem || submitting) return;

        setSubmitting(true);
        try {
            const res = await authfetch("/submission", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
                    problem_id: problem.id,
                    language: language === "c++" ? "cpp" : language,
                    code,
                    contest_id,
                }),
            });

            if (!res.ok) {
                throw new Error(await res.text() || "Submission failed");
            }

            const data = await res.json();
            setCurrentSub(data);
        } catch (err) {
            alert((err as Error).message);
            setSubmitting(false);
        }
    };

    const renderContent = (content?: string | object) => {
        if (!content) return "None";
        if (typeof content === "string") {
            try {
                const parsed = JSON.parse(content);
                return JSON.stringify(parsed, null, 2);
            } catch {
                return content;
            }
        }
        return JSON.stringify(content, null, 2);
    };

    if (loading) {
        return (
            <div className="h-[calc(100vh-60px)] bg-black flex items-center justify-center text-white">
                <div className="flex flex-col items-center gap-3">
                    <div className="w-7 h-7 border-2 border-zinc-400 border-t-transparent rounded-full animate-spin" />
                    <p className="text-zinc-400 text-sm">Loading problem statement...</p>
                </div>
            </div>
        );
    }

    if (error || !problem) {
        return (
            <div className="h-[calc(100vh-60px)] bg-black flex items-center justify-center p-6">
                <div className="bg-zinc-950 border border-zinc-900 p-8 rounded-2xl max-w-md w-full text-center">
                    <h2 className="text-xl font-bold text-zinc-200 mb-2">Problem Not Found</h2>
                    <p className="text-zinc-400 text-sm mb-6">{error || "Could not load problem data."}</p>
                    <Link
                        to="/problems"
                        className="px-6 py-2.5 bg-zinc-100 hover:bg-white text-zinc-950 rounded-xl font-semibold transition shadow-sm"
                    >
                        Back to Problem List
                    </Link>
                </div>
            </div>
        );
    }

    return (
        <div className="h-[calc(100vh-60px)] flex flex-col bg-black text-zinc-100 overflow-hidden">
            {contest_id && (
                <div className="bg-zinc-900 border-b border-zinc-800 px-6 py-2 flex justify-between items-center text-xs text-zinc-300">
                    <span className="font-semibold flex items-center gap-2">
                        <span>Contest #{contest_id}</span>
                    </span>
                    <Link to={`/contests/${contest_id}`} className="text-zinc-400 hover:text-white transition">
                        &larr; Return to Contest
                    </Link>
                </div>
            )}

            <div className="flex-1 flex flex-col md:flex-row overflow-hidden">
                <div className="w-full md:w-1/2 border-r border-zinc-900 flex flex-col bg-black overflow-hidden">
                    <div className="flex items-center gap-2 px-6 py-3 border-b border-zinc-900 bg-zinc-950">
                        <button
                            onClick={() => setActiveTab("statement")}
                            className={`px-3 py-1.5 rounded-lg text-xs font-semibold transition ${
                                activeTab === "statement"
                                    ? "bg-zinc-800 text-white shadow-sm border border-zinc-700"
                                    : "text-zinc-400 hover:text-zinc-200"
                            }`}
                        >
                            Description
                        </button>
                        <button
                            onClick={() => setActiveTab("submissions")}
                            className={`px-3 py-1.5 rounded-lg text-xs font-semibold transition ${
                                activeTab === "submissions"
                                    ? "bg-zinc-800 text-white shadow-sm border border-zinc-700"
                                    : "text-zinc-400 hover:text-zinc-200"
                            }`}
                        >
                            My Submissions
                        </button>
                    </div>

                    <div className="flex-1 overflow-y-auto p-6 space-y-6">
                        {activeTab === "statement" ? (
                            <>
                                <div>
                                    <div className="flex items-center gap-3 mb-2">
                                        <span className="text-zinc-500 font-mono text-sm">#{problem.id}</span>
                                        <h1 className="text-2xl font-bold text-white">{problem.title}</h1>
                                    </div>
                                    <div className="flex items-center gap-3 text-xs">
                                        <span
                                            className={`px-2.5 py-0.5 rounded-md border text-xs capitalize ${getDifficultyColor(
                                                problem.difficulty
                                            )}`}
                                        >
                                            {problem.difficulty || "Unrated"}
                                        </span>
                                        <span className="text-zinc-400">Limit: {problem.time_limit} ms</span>
                                        <span className="text-zinc-400">Memory: {problem.memory_limit} MB</span>
                                    </div>
                                </div>

                                <section className="space-y-2">
                                    <h3 className="font-semibold text-xs text-zinc-400 uppercase tracking-wider">Problem Statement</h3>
                                    <div className="p-4 bg-zinc-950 border border-zinc-900 rounded-xl text-sm leading-relaxed whitespace-pre-wrap text-zinc-300">
                                        {problem.statement || "No statement provided."}
                                    </div>
                                </section>

                                {problem.input_spec && (
                                    <section className="space-y-2">
                                        <h3 className="font-semibold text-xs text-zinc-400 uppercase tracking-wider">Input Specification</h3>
                                        <div className="p-4 bg-zinc-950 border border-zinc-900 rounded-xl text-sm whitespace-pre-wrap text-zinc-300">
                                            {problem.input_spec}
                                        </div>
                                    </section>
                                )}

                                {problem.output_spec && (
                                    <section className="space-y-2">
                                        <h3 className="font-semibold text-xs text-zinc-400 uppercase tracking-wider">Output Specification</h3>
                                        <div className="p-4 bg-zinc-950 border border-zinc-900 rounded-xl text-sm whitespace-pre-wrap text-zinc-300">
                                            {problem.output_spec}
                                        </div>
                                    </section>
                                )}

                                {problem.sample_inputs && (
                                    <section className="space-y-2">
                                        <h3 className="font-semibold text-xs text-zinc-400 uppercase tracking-wider">Sample Inputs</h3>
                                        <pre className="p-4 bg-zinc-950 border border-zinc-900 font-mono text-xs text-zinc-300 rounded-xl whitespace-pre-wrap overflow-x-auto">
                                            {renderContent(problem.sample_inputs)}
                                        </pre>
                                    </section>
                                )}

                                {problem.sample_outputs && (
                                    <section className="space-y-2">
                                        <h3 className="font-semibold text-xs text-zinc-400 uppercase tracking-wider">Sample Outputs</h3>
                                        <pre className="p-4 bg-zinc-950 border border-zinc-900 font-mono text-xs text-zinc-300 rounded-xl whitespace-pre-wrap overflow-x-auto">
                                            {renderContent(problem.sample_outputs)}
                                        </pre>
                                    </section>
                                )}
                            </>
                        ) : (
                            <div className="space-y-4">
                                <h3 className="font-semibold text-xs text-zinc-400 uppercase tracking-wider">Recent Submissions</h3>
                                {recentSubmissions.length === 0 ? (
                                    <p className="text-zinc-500 text-sm text-center py-8">No submissions yet for this problem.</p>
                                ) : (
                                    <div className="bg-zinc-950 border border-zinc-900 rounded-xl overflow-hidden">
                                        <ViewSubmissions submissions={recentSubmissions} view_code={true} />
                                    </div>
                                )}
                            </div>
                        )}
                    </div>
                </div>

                <div className="w-full md:w-1/2 flex flex-col bg-black overflow-hidden">
                    <div className="flex flex-wrap justify-between items-center px-4 py-2.5 bg-zinc-950 border-b border-zinc-900 gap-2 text-xs">
                        <div className="flex items-center gap-2 flex-wrap">
                            <select
                                id="lang-select"
                                value={language}
                                onChange={(e) => handleLanguageChange(e.target.value)}
                                className="bg-zinc-900 border border-zinc-800 text-white rounded-lg px-2.5 py-1 focus:border-zinc-500 outline-none"
                            >
                                <option value="c++">C++ (g++ 20)</option>
                                <option value="python">Python 3</option>
                                <option value="java">Java (OpenJDK)</option>
                                <option value="rust">Rust</option>
                                <option value="go">Go</option>
                                <option value="javascript">JavaScript (Node.js)</option>
                            </select>

                            <select
                                value={editorTheme}
                                onChange={(e) => setEditorTheme(e.target.value)}
                                className="bg-zinc-900 border border-zinc-800 text-zinc-300 rounded-lg px-2 py-1 outline-none"
                                title="Editor Theme"
                            >
                                <option value="vs-dark">Dark Theme</option>
                                <option value="light">Light Theme</option>
                                <option value="hc-black">High Contrast</option>
                            </select>

                            <select
                                value={fontSize}
                                onChange={(e) => setFontSize(Number(e.target.value))}
                                className="bg-zinc-900 border border-zinc-800 text-zinc-300 rounded-lg px-2 py-1 outline-none"
                                title="Font Size"
                            >
                                <option value={12}>12px</option>
                                <option value={14}>14px</option>
                                <option value={16}>16px</option>
                                <option value={18}>18px</option>
                            </select>

                            <button
                                onClick={() => setWordWrap(wordWrap === "on" ? "off" : "on")}
                                className={`px-2.5 py-1 rounded-lg border text-xs transition ${
                                    wordWrap === "on"
                                        ? "bg-zinc-800 text-white border-zinc-700"
                                        : "bg-zinc-900 text-zinc-400 border-zinc-800"
                                }`}
                                title="Toggle Word Wrap"
                            >
                                Wrap
                            </button>

                            <button
                                onClick={handleResetCode}
                                className="px-2.5 py-1 bg-zinc-900 hover:bg-zinc-800 text-zinc-300 rounded-lg border border-zinc-800 transition"
                                title="Reset to default template"
                            >
                                ↺ Reset
                            </button>
                        </div>

                        <div className="flex items-center gap-2">
                            {currentSub && (
                                <span
                                    className={`px-2.5 py-1 text-xs rounded-lg ${getStatusColor(
                                        currentSub.status
                                    )}`}
                                >
                                    {currentSub.status.toUpperCase() === "PENDING"
                                        ? "Judging..."
                                        : `${currentSub.status} ${
                                              currentSub.time !== undefined && currentSub.time !== null
                                                  ? `(${currentSub.time}ms)`
                                                  : ""
                                          }`}
                                </span>
                            )}

                            <button
                                onClick={handleSubmit}
                                disabled={submitting || currentSub?.status.toUpperCase() === "PENDING"}
                                className={`px-4 py-1.5 text-xs font-semibold rounded-xl transition flex items-center gap-2 ${
                                    submitting || currentSub?.status.toUpperCase() === "PENDING"
                                        ? "bg-zinc-800 text-zinc-500 cursor-not-allowed border border-zinc-700"
                                        : "bg-zinc-100 hover:bg-white text-zinc-950 shadow-sm"
                                }`}
                            >
                                {submitting || currentSub?.status.toUpperCase() === "PENDING" ? (
                                    <>
                                        <div className="w-3 h-3 border-2 border-zinc-400 border-t-transparent rounded-full animate-spin" />
                                        Evaluating...
                                    </>
                                ) : (
                                    "Submit Solution"
                                )}
                            </button>
                        </div>
                    </div>

                    <div className="flex-1 overflow-hidden relative">
                        <Editor
                            height="100%"
                            theme={editorTheme}
                            language={language === "c++" ? "cpp" : language}
                            value={code}
                            onChange={(val) => setCode(val || "")}
                            options={{
                                minimap: { enabled: false },
                                fontSize,
                                wordWrap,
                                lineNumbers: "on",
                                scrollBeyondLastLine: false,
                                automaticLayout: true,
                                tabSize: 4,
                                renderWhitespace: "selection",
                                bracketPairColorization: { enabled: true },
                            }}
                        />
                    </div>

                    {currentSub && currentSub.status.toUpperCase() !== "PENDING" && currentSub.verdict && (
                        <div className="border-t border-zinc-900 bg-zinc-950 p-3.5 font-mono text-xs">
                            <div className="flex justify-between items-center mb-1.5">
                                <span className="font-medium text-zinc-300">Execution Result:</span>
                                <Link
                                    to={`/submissions/${currentSub.id}`}
                                    className="text-zinc-400 hover:text-white transition"
                                >
                                    Details &rarr;
                                </Link>
                            </div>
                            <div className="p-2.5 rounded-lg bg-zinc-900 text-zinc-200 border border-zinc-800 whitespace-pre-wrap">
                                {currentSub.verdict}
                            </div>
                        </div>
                    )}
                </div>
            </div>

        </div>
    );
}
