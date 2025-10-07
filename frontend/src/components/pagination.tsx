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
        <div className="flex justify-center mt-4 gap-2">
            <button
                className="px-3 py-1 bg-gray-200 rounded disabled:opacity-50"
                disabled={page <= 1 || loading}
                onClick={goPrev}
            >
                Prev
            </button>
            <span className="px-3 py-1">Page {page}</span>
            <button
                className="px-3 py-1 bg-gray-200 rounded disabled:opacity-50"
                disabled={!cursor || loading}
                onClick={goNext}
            >
                Next
            </button>
        </div>
    );
}
