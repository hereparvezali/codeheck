interface PaginationProps {
    page: number;
    cursor?: number;
    loading?: boolean;
    error?: string;
    goPrev: () => void;
    goNext: () => void;
}
export function Pagination({
    page,
    loading,
    cursor,
    goPrev,
    goNext,
}: PaginationProps) {
    return (
        <div className="flex items-center justify-center gap-2">
            <button
                className="px-3.5 py-1.5 bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 hover:border-zinc-700 text-zinc-300 hover:text-white text-xs font-semibold rounded-lg disabled:opacity-40 disabled:cursor-not-allowed transition"
                disabled={page <= 1 || loading}
                onClick={goPrev}
            >
                &larr; Prev
            </button>
            <span className="px-3 py-1 text-xs text-zinc-400 font-mono">Page {page}</span>
            <button
                className="px-3.5 py-1.5 bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 hover:border-zinc-700 text-zinc-300 hover:text-white text-xs font-semibold rounded-lg disabled:opacity-40 disabled:cursor-not-allowed transition"
                disabled={!cursor || loading}
                onClick={goNext}
            >
                Next &rarr;
            </button>
        </div>
    );
}
