export const getStatusColor = (status: string) => {
    switch (status.toUpperCase()) {
        case "AC":
            return "text-green-700 bg-green-100";
        case "WA":
            return "text-red-700 bg-red-100";
        case "TLE":
            return "text-yellow-700 bg-yellow-100";
        case "MLE":
            return "text-orange-700 bg-orange-100";
        case "RE/CE":
            return "text-purple-700 bg-purple-100";
        case "CE/RE":
            return "text-purple-700 bg-purple-100";
        case "PENDING":
            return "text-blue-700 bg-blue-100 animate-pulse";
        default:
            return "text-gray-700 bg-gray-100";
    }
};
