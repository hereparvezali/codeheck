import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { useAuth } from "../utils/contexts/authcontext";
import { ViewSubmissions, type Submission } from "../components/view_submissions";

interface SubmissionsResponse {
    cursor?: number;
    submissions: Submission[];
}

interface ProfileSubmissionsProps {
    onAcceptedCountUpdate?: (count: number) => void;
}

const ProfileSubmissions = ({ onAcceptedCountUpdate }: ProfileSubmissionsProps) => {
    const navigator = useNavigate();
    const { authfetch, user } = useAuth();

    const [submissions, setSubmissions] = useState<Submission[]>([]);
    const [submissionsLoading, setSubmissionsLoading] = useState(false);
    const [submissionsCursor, setSubmissionsCursor] = useState<number | undefined>(undefined);
    const [submissionsPage, setSubmissionsPage] = useState(1);
    const [submissionsCursors, setSubmissionsCursors] = useState<(number | undefined)[]>([]);

    useEffect(() => {
        if (submissions.length === 0) {
            fetchSubmissions();
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    const fetchSubmissions = async (cursor?: number) => {
        if (!user) return;

        setSubmissionsLoading(true);
        try {
            const params = new URLSearchParams({ limit: "20" });
            if (cursor) params.append("cursor", cursor.toString());
            params.append("user_id", user.id.toString());

            const res = await authfetch(`/submissions?${params.toString()}`);

            if (!res.ok) {
                if (res.status === 401) navigator("/signin");
                throw new Error(await res.text());
            }

            const data: SubmissionsResponse = await res.json();
            setSubmissions(data.submissions || []);
            setSubmissionsCursor(data.cursor);
            
            // Update accepted count for parent component
            if (onAcceptedCountUpdate) {
                const acceptedCount = (data.submissions || []).filter(s => s.status === "AC").length;
                onAcceptedCountUpdate(acceptedCount);
            }
        } catch (e) {
            console.error("Failed to fetch submissions:", e);
        } finally {
            setSubmissionsLoading(false);
        }
    };

    const goNextSubmissions = () => {
        if (!submissionsCursor) return;
        setSubmissionsPage((p) => p + 1);
        setSubmissionsCursors((prev) => [...prev, submissionsCursor]);
        fetchSubmissions(submissionsCursor);
    };

    const goPrevSubmissions = () => {
        if (submissionsCursors.length === 0) return;
        const prevCursor = submissionsCursors[submissionsCursors.length - 2];
        setSubmissionsPage((p) => p - 1);
        fetchSubmissions(prevCursor);
        setSubmissionsCursors((prev) => prev.slice(0, -1));
    };

    return (
        <div>
            <div className="flex items-center justify-between mb-4">
                <h2 className="text-2xl font-bold">Submission History</h2>
                {submissions.length > 0 && (
                    <div className="flex gap-2">
                        <button
                            onClick={goPrevSubmissions}
                            disabled={submissionsPage === 1 || submissionsLoading}
                            className="px-3 py-1 border rounded hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            ← Previous
                        </button>
                        <span className="px-3 py-1 text-gray-600">
                            Page {submissionsPage}
                        </span>
                        <button
                            onClick={goNextSubmissions}
                            disabled={!submissionsCursor || submissionsLoading}
                            className="px-3 py-1 border rounded hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            Next →
                        </button>
                    </div>
                )}
            </div>

            {submissionsLoading ? (
                <div className="text-center py-12">
                    <div className="animate-spin text-4xl mb-4">⏳</div>
                    <p className="text-gray-600">Loading submissions...</p>
                </div>
            ) : submissions.length === 0 ? (
                <div className="bg-gray-50 rounded-lg p-12 text-center">
                    <p className="text-gray-500 text-lg mb-4">
                        No submissions yet
                    </p>
                    <button
                        onClick={() => navigator("/problems")}
                        className="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
                    >
                        Submit Your First Solution
                    </button>
                </div>
            ) : (
                <div className="overflow-x-auto">
                    <ViewSubmissions
                        submissions={submissions}
                        status={true}
                        id={true}
                        language={true}
                        view_code={true}
                        time={true}
                        memory={true}
                    />
                </div>
            )}
        </div>
    );
};

export default ProfileSubmissions;