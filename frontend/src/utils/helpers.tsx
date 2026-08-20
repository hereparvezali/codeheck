export const getStatusColor = (status: string) => {
    switch (status.toUpperCase()) {
        case "AC":
        case "ACCEPTED":
            return "text-emerald-400 bg-emerald-500/10 border border-emerald-500/25 font-medium";
        case "WA":
        case "WRONG ANSWER":
            return "text-rose-400 bg-rose-500/10 border border-rose-500/25 font-medium";
        case "TLE":
        case "TIME LIMIT EXCEEDED":
            return "text-amber-400 bg-amber-500/10 border border-amber-500/25 font-medium";
        case "MLE":
        case "MEMORY LIMIT EXCEEDED":
            return "text-purple-400 bg-purple-500/10 border border-purple-500/25 font-medium";
        case "RE":
        case "RUNTIME ERROR":
        case "SIGSEGV":
        case "SIGXFSZ":
        case "SIGFPE":
        case "SIGABRT":
        case "RTE":
            return "text-orange-400 bg-orange-500/10 border border-orange-500/25 font-medium";
        case "CE":
        case "COMPILATION ERROR":
        case "RE/CE":
        case "CE/RE":
            return "text-yellow-400 bg-yellow-500/10 border border-yellow-500/25 font-medium";
        case "PENDING":
        case "RUNNING":
        case "IN QUEUE":
        case "JUDGING":
            return "text-sky-400 bg-sky-500/10 border border-sky-500/25 animate-pulse font-medium";
        default:
            return "text-zinc-400 bg-zinc-900 border border-zinc-800";
    }
};

export const getDifficultyColor = (diff?: string) => {
    switch (diff?.toLowerCase()) {
        case "easy":
            return "text-emerald-400 bg-emerald-500/10 border border-emerald-500/25 font-medium";
        case "medium":
            return "text-amber-400 bg-amber-500/10 border border-amber-500/25 font-medium";
        case "hard":
            return "text-rose-400 bg-rose-500/10 border border-rose-500/25 font-medium";
        default:
            return "text-zinc-400 bg-zinc-900 border border-zinc-800";
    }
};

export const getContestStatusColor = (status: "ongoing" | "upcoming" | "past" | "ended") => {
    switch (status) {
        case "ongoing":
            return "text-emerald-400 bg-emerald-500/10 border border-emerald-500/30 font-semibold";
        case "upcoming":
            return "text-sky-400 bg-sky-500/10 border border-sky-500/25 font-medium";
        case "past":
        case "ended":
        default:
            return "text-zinc-500 bg-zinc-900/60 border border-zinc-800/80";
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

export function getWsUrl(path: string): string {
    const base = import.meta.env.VITE_BASE || "/api";
    const cleanPath = path.startsWith("/") ? path : `/${path}`;
    if (base.startsWith("http://") || base.startsWith("https://")) {
        return base.replace(/^http/, "ws") + cleanPath;
    }
    const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
    const cleanBase = base.replace(/\/$/, "");
    return `${protocol}//${window.location.host}${cleanBase}${cleanPath}`;
}


