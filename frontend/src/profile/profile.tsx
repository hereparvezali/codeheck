import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { useAuth } from "../utils/contexts/authcontext";
import ProfileOverview from "./ProfileOverview";
import ProfileSolved from "./ProfileSolved";
import ProfileSubmissions from "./ProfileSubmissions";

interface User {
    username: string;
    email: string;
    rating: number;
    created_at: string;
}

const Profile = () => {
    const navigator = useNavigate();
    const { authfetch, signout, user } = useAuth();
    const [loading, setLoading] = useState(true);
    const [userinfo, setUserinfo] = useState<User | null>(null);
    const [activeTab, setActiveTab] = useState<
        "overview" | "solved" | "submissions"
    >("overview");

    // Stats for overview
    const [solvedCount, setSolvedCount] = useState(0);
    const [acceptedSubmissionsCount, setAcceptedSubmissionsCount] = useState(0);

    useEffect(() => {
        const controller = new AbortController();
        const callback = async () => {
            const res = await authfetch("/user");

            if (!res.ok) {
                navigator("/signin");
            } else {
                setUserinfo(await res.json());
                setLoading(false);
            }
        };
        callback();
        return () => controller.abort();
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    // Fetch counts for overview
    useEffect(() => {
        if (user && activeTab === "overview") {
            fetchCounts();
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [activeTab, user]);

    const fetchCounts = async () => {
        if (!user) return;

        try {
            // Fetch solved count
            const solvedRes = await authfetch(
                `/problems?user_id=${user.id}&status=AC&limit=1`,
            );
            if (solvedRes.ok) {
                const solvedData = await solvedRes.json();
                if (solvedData.count !== undefined) {
                    setSolvedCount(solvedData.count);
                }
            }

            // Fetch accepted submissions count
            const submissionsRes = await authfetch(
                `/submissions?user_id=${user.id}&status=AC&limit=1`,
            );
            if (submissionsRes.ok) {
                const submissionsData = await submissionsRes.json();
                if (submissionsData.count !== undefined) {
                    setAcceptedSubmissionsCount(submissionsData.count);
                }
            }
        } catch (e) {
            console.error("Failed to fetch counts:", e);
        }
    };

    const handle_signout = async () => {
        await signout();
        navigator("/signin");
    };

    if (loading) {
        return (
            <div className="flex justify-center items-center h-screen">
                <div className="text-gray-500">Loading profile...</div>
            </div>
        );
    }

    return (
        <div className="max-w-6xl mx-auto p-6">
            {/* Profile Header */}
            <div className="bg-white shadow-lg rounded-xl p-6 flex items-center justify-between mb-6">
                <div className="flex items-center gap-6">
                    {/* Avatar */}
                    <div className="w-20 h-20 bg-gradient-to-br from-blue-500 to-purple-600 rounded-full flex items-center justify-center text-3xl font-bold text-white shadow-lg">
                        {userinfo?.username[0]?.toUpperCase()}
                    </div>

                    {/* User Info */}
                    <div>
                        <h1 className="text-3xl font-bold">
                            {userinfo?.username}
                        </h1>
                        <p className="text-gray-600">{userinfo?.email}</p>
                        <div className="flex gap-4 mt-2">
                            <p>
                                <span className="font-semibold">Rating:</span>{" "}
                                <span className="text-blue-600 font-bold">
                                    {userinfo?.rating}
                                </span>
                            </p>
                            <p>
                                <span className="font-semibold">Joined:</span>{" "}
                                {new Date(
                                    userinfo?.created_at || "",
                                ).toLocaleDateString()}
                            </p>
                        </div>
                    </div>
                </div>

                {/* Logout Button */}
                <button
                    onClick={handle_signout}
                    className="px-6 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg transition-colors"
                >
                    Logout
                </button>
            </div>

            {/* Tab Navigation */}
            <div className="bg-white shadow rounded-xl mb-6">
                <div className="flex border-b">
                    <button
                        onClick={() => setActiveTab("overview")}
                        className={`flex-1 px-6 py-4 font-medium transition-colors ${
                            activeTab === "overview"
                                ? "border-b-2 border-blue-600 text-blue-600 bg-blue-50"
                                : "text-gray-600 hover:text-gray-800 hover:bg-gray-50"
                        }`}
                    >
                        Overview
                    </button>
                    <button
                        onClick={() => setActiveTab("solved")}
                        className={`flex-1 px-6 py-4 font-medium transition-colors ${
                            activeTab === "solved"
                                ? "border-b-2 border-blue-600 text-blue-600 bg-blue-50"
                                : "text-gray-600 hover:text-gray-800 hover:bg-gray-50"
                        }`}
                    >
                        Solved Problems {solvedCount > 0 && `(${solvedCount})`}
                    </button>
                    <button
                        onClick={() => setActiveTab("submissions")}
                        className={`flex-1 px-6 py-4 font-medium transition-colors ${
                            activeTab === "submissions"
                                ? "border-b-2 border-blue-600 text-blue-600 bg-blue-50"
                                : "text-gray-600 hover:text-gray-800 hover:bg-gray-50"
                        }`}
                    >
                        Submissions
                    </button>
                </div>

                {/* Tab Content */}
                <div className="p-6">
                    {activeTab === "overview" && (
                        <ProfileOverview
                            userinfo={userinfo}
                            solvedCount={solvedCount}
                            acceptedSubmissionsCount={acceptedSubmissionsCount}
                        />
                    )}

                    {activeTab === "solved" && (
                        <ProfileSolved
                            onCountUpdate={(count) => setSolvedCount(count)}
                        />
                    )}

                    {activeTab === "submissions" && (
                        <ProfileSubmissions
                            onAcceptedCountUpdate={(count) =>
                                setAcceptedSubmissionsCount(count)
                            }
                        />
                    )}
                </div>
            </div>
        </div>
    );
};

export default Profile;