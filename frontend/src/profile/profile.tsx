import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { useAuth } from "../utils/contexts/authcontext";

interface User {
    username: string;
    email: string;
    rating: number;
    created_at: string;
}

const Profile = () => {
    const navigator = useNavigate();
    const { authfetch, signout } = useAuth();
    const [loading, setLoading] = useState(true);
    const [userinfo, setUserinfo] = useState<User | null>(null);

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
        <div className="max-w-4xl mx-auto p-6">
            {/* Profile Card */}
            <div className="bg-white shadow-lg rounded-xl p-6 flex items-center gap-6">
                {/* Avatar Placeholder */}
                <div className="w-20 h-20 bg-gray-200 rounded-full flex items-center justify-center text-2xl font-bold text-gray-500">
                    {userinfo?.username[0]?.toUpperCase()}
                </div>

                {/* User Info */}
                <div>
                    <h1 className="text-2xl font-bold">{userinfo?.username}</h1>
                    <p className="text-gray-600">{userinfo?.email}</p>
                    <p className="mt-2">
                        <span className="font-semibold">Rating:</span>{" "}
                        {userinfo?.rating}
                    </p>
                    <p>
                        <span className="font-semibold">Joined:</span>{" "}
                        {new Date(
                            userinfo?.created_at || "",
                        ).toLocaleDateString()}
                    </p>
                </div>
            </div>

            {/* Stats Section */}
            <div className="grid grid-cols-3 gap-4 mt-6">
                <div className="bg-white p-4 shadow rounded-lg text-center">
                    <p className="text-2xl font-bold">23</p>
                    <p className="text-gray-500">Problems Solved</p>
                </div>
                <div className="bg-white p-4 shadow rounded-lg text-center">
                    <p className="text-2xl font-bold">5</p>
                    <p className="text-gray-500">Contests Joined</p>
                </div>
                <div className="bg-white p-4 shadow rounded-lg text-center">
                    <p className="text-2xl font-bold">3</p>
                    <p className="text-gray-500">Badges</p>
                </div>
            </div>

            {/* Logout */}
            <div className="mt-6 text-right">
                <button
                    onClick={handle_signout}
                    className="px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg"
                >
                    Logout
                </button>
            </div>
        </div>
    );
};

export default Profile;
