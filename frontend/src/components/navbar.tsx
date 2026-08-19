import { useState } from "react";
import { NavLink, useNavigate } from "react-router-dom";
import { useAuth } from "../utils/contexts/authcontext";

export default function Navbar() {
    const { user, signout } = useAuth();
    const navigate = useNavigate();
    const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);

    const handleSignout = async () => {
        setIsMobileMenuOpen(false);
        await signout();
        navigate("/");
    };

    const closeMobileMenu = () => {
        setIsMobileMenuOpen(false);
    };

    return (
        <nav className="bg-zinc-950 border-b border-zinc-800 text-zinc-100 sticky top-0 z-50 backdrop-blur-md bg-zinc-950/95 shadow-md">
            <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div className="flex justify-between items-center h-16">
                    <div className="flex items-center gap-8">
                        <NavLink
                            to="/"
                            onClick={closeMobileMenu}
                            className="text-xl font-bold tracking-tight text-white flex items-center gap-2 group"
                        >
                            <span className="w-8 h-8 rounded-lg bg-zinc-100 text-zinc-950 flex items-center justify-center text-sm font-black shadow-sm group-hover:bg-white transition">
                                &lt;&gt;
                            </span>
                            <span className="font-bold text-zinc-100 tracking-tight">
                                CodeHeck
                            </span>
                        </NavLink>

                        <ul className="hidden md:flex items-center gap-4 text-sm font-medium">
                            <li>
                                <NavLink
                                    to="/problems"
                                    className={({ isActive }) =>
                                        `px-3 py-1.5 rounded-lg transition ${
                                            isActive
                                                ? "bg-zinc-800 text-white font-semibold border border-zinc-700"
                                                : "text-zinc-400 hover:text-white hover:bg-zinc-900"
                                        }`
                                    }
                                >
                                    Problems
                                </NavLink>
                            </li>
                            <li>
                                <NavLink
                                    to="/contests"
                                    className={({ isActive }) =>
                                        `px-3 py-1.5 rounded-lg transition ${
                                            isActive
                                                ? "bg-zinc-800 text-white font-semibold border border-zinc-700"
                                                : "text-zinc-400 hover:text-white hover:bg-zinc-900"
                                        }`
                                    }
                                >
                                    Contests
                                </NavLink>
                            </li>
                            {user && (
                                <li>
                                    <NavLink
                                        to="/admin"
                                        className={({ isActive }) =>
                                            `px-3 py-1.5 rounded-lg transition ${
                                                isActive
                                                    ? "bg-zinc-800 text-white font-semibold border border-zinc-700"
                                                    : "text-zinc-400 hover:text-white hover:bg-zinc-900"
                                            }`
                                        }
                                    >
                                        Admin
                                    </NavLink>
                                </li>
                            )}
                        </ul>
                    </div>

                    <div className="hidden md:flex items-center gap-3">
                        {user ? (
                            <div className="flex items-center gap-2.5">
                                <NavLink
                                    to="/profile"
                                    className="flex items-center gap-2 px-3 py-1.5 rounded-xl bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 hover:border-zinc-700 transition group"
                                >
                                    <div className="w-6 h-6 rounded-md bg-zinc-800 border border-zinc-700 flex items-center justify-center text-xs font-bold text-zinc-200">
                                        {user.username.charAt(0).toUpperCase()}
                                    </div>
                                    <span className="text-xs font-medium text-zinc-300 group-hover:text-white transition">
                                        {user.username}
                                    </span>
                                </NavLink>

                                <button
                                    onClick={handleSignout}
                                    className="px-3 py-1.5 text-xs font-medium text-zinc-400 hover:text-zinc-100 hover:bg-zinc-900 rounded-lg border border-transparent hover:border-zinc-800 transition cursor-pointer"
                                >
                                    Sign Out
                                </button>
                            </div>
                        ) : (
                            <div className="flex items-center gap-2">
                                <NavLink
                                    to="/signin"
                                    className="px-3.5 py-1.5 text-sm font-medium text-zinc-300 hover:text-white hover:bg-zinc-900 rounded-lg transition"
                                >
                                    Sign In
                                </NavLink>
                                <NavLink
                                    to="/signup"
                                    className="px-4 py-1.5 bg-zinc-100 hover:bg-white text-zinc-950 text-sm font-semibold rounded-xl transition shadow-sm"
                                >
                                    Sign Up
                                </NavLink>
                            </div>
                        )}
                    </div>

                    <div className="flex md:hidden items-center gap-2">
                        {user && (
                            <NavLink
                                to="/profile"
                                onClick={closeMobileMenu}
                                className="w-7 h-7 rounded-lg bg-zinc-800 border border-zinc-700 flex items-center justify-center text-xs font-bold text-zinc-200 shadow-sm"
                                aria-label="Profile"
                            >
                                {user.username.charAt(0).toUpperCase()}
                            </NavLink>
                        )}

                        <button
                            type="button"
                            onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
                            className="p-2 rounded-xl text-zinc-400 hover:text-white hover:bg-zinc-900 focus:outline-none transition"
                            aria-label="Toggle navigation menu"
                            aria-expanded={isMobileMenuOpen}
                        >
                            {isMobileMenuOpen ? (
                                <svg
                                    className="w-5 h-5"
                                    fill="none"
                                    stroke="currentColor"
                                    viewBox="0 0 24 24"
                                >
                                    <path
                                        strokeLinecap="round"
                                        strokeLinejoin="round"
                                        strokeWidth={2}
                                        d="M6 18L18 6M6 6l12 12"
                                    />
                                </svg>
                            ) : (
                                <svg
                                    className="w-5 h-5"
                                    fill="none"
                                    stroke="currentColor"
                                    viewBox="0 0 24 24"
                                >
                                    <path
                                        strokeLinecap="round"
                                        strokeLinejoin="round"
                                        strokeWidth={2}
                                        d="M4 6h16M4 12h16M4 18h16"
                                    />
                                </svg>
                            )}
                        </button>
                    </div>
                </div>
            </div>

            {isMobileMenuOpen && (
                <div className="md:hidden border-t border-zinc-800 bg-zinc-950 px-4 pt-3 pb-5 space-y-3 animate-in fade-in slide-in-from-top-2 duration-150">
                    <div className="space-y-1">
                        <NavLink
                            to="/problems"
                            onClick={closeMobileMenu}
                            className={({ isActive }) =>
                                `flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium transition ${
                                    isActive
                                        ? "bg-zinc-800 text-white border border-zinc-700 font-semibold"
                                        : "text-zinc-400 hover:text-white hover:bg-zinc-900"
                                }`
                            }
                        >
                            <span>Problems</span>
                        </NavLink>

                        <NavLink
                            to="/contests"
                            onClick={closeMobileMenu}
                            className={({ isActive }) =>
                                `flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium transition ${
                                    isActive
                                        ? "bg-zinc-800 text-white border border-zinc-700 font-semibold"
                                        : "text-zinc-400 hover:text-white hover:bg-zinc-900"
                                }`
                            }
                        >
                            <span>Contests</span>
                        </NavLink>

                        {user && (
                            <NavLink
                                to="/admin"
                                onClick={closeMobileMenu}
                                className={({ isActive }) =>
                                    `flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium transition ${
                                        isActive
                                            ? "bg-zinc-800 text-white border border-zinc-700 font-semibold"
                                            : "text-zinc-400 hover:text-white hover:bg-zinc-900"
                                    }`
                                }
                            >
                                <span>Admin Panel</span>
                            </NavLink>
                        )}
                    </div>

                    <div className="pt-3 border-t border-zinc-800 space-y-2">
                        {user ? (
                            <div className="space-y-2">
                                <NavLink
                                    to="/profile"
                                    onClick={closeMobileMenu}
                                    className="flex items-center justify-between px-3 py-2.5 rounded-xl bg-zinc-900 border border-zinc-800 text-sm font-medium text-zinc-200 hover:text-white"
                                >
                                    <div className="flex items-center gap-2.5">
                                        <div className="w-6 h-6 rounded-md bg-zinc-800 border border-zinc-700 flex items-center justify-center text-xs font-bold text-zinc-200">
                                            {user.username.charAt(0).toUpperCase()}
                                        </div>
                                        <span>{user.username}</span>
                                    </div>
                                    <span className="text-xs text-zinc-400 font-medium">View Profile &rarr;</span>
                                </NavLink>

                                <button
                                    onClick={handleSignout}
                                    className="w-full text-left px-3 py-2.5 rounded-xl text-sm font-medium text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200 border border-zinc-800 transition"
                                >
                                    Sign Out
                                </button>
                            </div>
                        ) : (
                            <div className="grid grid-cols-2 gap-2 pt-1">
                                <NavLink
                                    to="/signin"
                                    onClick={closeMobileMenu}
                                    className="text-center px-4 py-2.5 rounded-xl text-sm font-medium text-zinc-300 hover:text-white bg-zinc-900 hover:bg-zinc-800 border border-zinc-800 transition"
                                >
                                    Sign In
                                </NavLink>
                                <NavLink
                                    to="/signup"
                                    onClick={closeMobileMenu}
                                    className="text-center px-4 py-2.5 rounded-xl text-sm font-semibold text-zinc-950 bg-zinc-100 hover:bg-white shadow-sm transition"
                                >
                                    Sign Up
                                </NavLink>
                            </div>
                        )}
                    </div>
                </div>
            )}
        </nav>
    );
}

