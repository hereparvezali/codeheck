import { useNavigate } from "react-router-dom";

interface ProfileOverviewProps {
    userinfo: {
        username: string;
        email: string;
        rating: number;
        created_at: string;
    } | null;
    solvedCount: number;
    acceptedSubmissionsCount: number;
}

const ProfileOverview = ({ 
    userinfo, 
    solvedCount, 
    acceptedSubmissionsCount 
}: ProfileOverviewProps) => {
    const navigator = useNavigate();

    return (
        <div>
            <h2 className="text-2xl font-bold mb-4">Statistics</h2>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div className="bg-gradient-to-br from-green-50 to-green-100 p-6 rounded-lg border border-green-200">
                    <p className="text-4xl font-bold text-green-700">
                        {solvedCount}
                    </p>
                    <p className="text-gray-700 mt-2">Problems Solved</p>
                </div>
                <div className="bg-gradient-to-br from-blue-50 to-blue-100 p-6 rounded-lg border border-blue-200">
                    <p className="text-4xl font-bold text-blue-700">
                        {userinfo?.rating}
                    </p>
                    <p className="text-gray-700 mt-2">Current Rating</p>
                </div>
                <div className="bg-gradient-to-br from-purple-50 to-purple-100 p-6 rounded-lg border border-purple-200">
                    <p className="text-4xl font-bold text-purple-700">
                        {acceptedSubmissionsCount}
                    </p>
                    <p className="text-gray-700 mt-2">Accepted Submissions</p>
                </div>
            </div>

            <div className="mt-8">
                <h3 className="text-xl font-bold mb-4">Recent Activity</h3>
                <div className="bg-gray-50 rounded-lg p-6 text-center text-gray-500">
                    <p>Start solving problems to see your activity here!</p>
                    <button
                        onClick={() => navigator("/problems")}
                        className="mt-4 px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
                    >
                        Browse Problems
                    </button>
                </div>
            </div>
        </div>
    );
};

export default ProfileOverview;