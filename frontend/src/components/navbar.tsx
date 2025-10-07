import { NavLink } from "react-router-dom";

export default function Navbar() {
    return (
        <nav className="bg-gray-900 text-white px-6 py-3 flex justify-between items-center shadow-md">
            {/* Brand */}
            <NavLink
                to="/"
                className="text-2xl font-bold text-blue-400 hover:text-blue-500"
            >
                CodeHeck
            </NavLink>

            {/* Links */}
            <ul className="flex gap-10">
                <li>
                    <NavLink
                        to="/problems"
                        className={({ isActive }) =>
                            `hover:text-blue-400 ${
                                isActive
                                    ? "text-blue-400 font-semibold"
                                    : "text-white"
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
                            `hover:text-blue-400 ${
                                isActive
                                    ? "text-blue-400 font-semibold"
                                    : "text-white"
                            }`
                        }
                    >
                        Contests
                    </NavLink>
                </li>
                <li>
                    <NavLink
                        to="/admin"
                        className={({ isActive }) =>
                            `hover:text-blue-400 ${
                                isActive
                                    ? "text-blue-400 font-semibold"
                                    : "text-white"
                            }`
                        }
                    >
                        Admin
                    </NavLink>
                </li>
                <li>
                    <NavLink
                        to="/"
                        className={({ isActive }) =>
                            `hover:text-blue-400 ${
                                isActive
                                    ? "text-blue-400 font-semibold"
                                    : "text-white"
                            }`
                        }
                    >
                        Profile
                    </NavLink>
                </li>
            </ul>
        </nav>
    );
}
