export const getStatusColor = (status: string) => {
    switch (status.toUpperCase()) {
        case "AC":
        case "ACCEPTED":
            return "text-zinc-100 bg-zinc-800 border border-zinc-600 font-medium";
        case "WA":
        case "WRONG ANSWER":
            return "text-zinc-300 bg-zinc-900 border border-zinc-800";
        case "TLE":
        case "TIME LIMIT EXCEEDED":
            return "text-zinc-300 bg-zinc-900 border border-zinc-800";
        case "MLE":
        case "MEMORY LIMIT EXCEEDED":
            return "text-zinc-300 bg-zinc-900 border border-zinc-800";
        case "RE":
        case "RUNTIME ERROR":
        case "RE/CE":
        case "CE/RE":
        case "CE":
        case "COMPILATION ERROR":
            return "text-zinc-400 bg-zinc-900 border border-zinc-800";
        case "PENDING":
        case "RUNNING":
        case "IN QUEUE":
            return "text-zinc-300 bg-zinc-900 border border-zinc-700 animate-pulse";
        default:
            return "text-zinc-400 bg-zinc-900 border border-zinc-800";
    }
};


export function parseUtcDate(dateStr?: string | null): Date {
    if (!dateStr) return new Date();
    const normalized = dateStr.endsWith("Z") || dateStr.includes("+") ? dateStr : `${dateStr}Z`;
    return new Date(normalized);
}


export function formatUtcToLocal(dateStr?: string | null): string {
    if (!dateStr) return "";
    return parseUtcDate(dateStr).toLocaleString();
}


export function toUtcIsoString(datetimeLocal: string): string {
    if (!datetimeLocal) return "";
    const date = new Date(datetimeLocal);
    return date.toISOString();
}


export function toLocalDatetimeInput(utcString: string): string {
    if (!utcString) return "";
    const date = parseUtcDate(utcString);
    const pad = (n: number) => String(n).padStart(2, "0");
    const year = date.getFullYear();
    const month = pad(date.getMonth() + 1);
    const day = pad(date.getDate());
    const hours = pad(date.getHours());
    const minutes = pad(date.getMinutes());

    return `${year}-${month}-${day}T${hours}:${minutes}`;
}

