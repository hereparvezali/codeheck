import { BrowserRouter, Route, Routes } from "react-router-dom";
import Navbar from "./components/navbar";
import Home from "./home/home";
import Signup from "./user/signup";
import Signin from "./user/signin";
import Profile from "./profile/profile";
import Contests from "./contests/contests";
import ContestDetail from "./contests/contest";
import Problems from "./problems/problems";
import Problem from "./problems/problem";
import SubmissionView from "./submissions/submission_view";
import Dashboard from "./admin/dashboard";
import CreateProblem from "./admin/create_problem";
import CreateContest from "./admin/create_contest";
import EditProblem from "./admin/edit_problem";
import EditContest from "./admin/edit_contest";
import { AuthProvider } from "./utils/contexts/authcontext";
import ProtectedRoute from "./utils/contexts/protected";

export default function App() {
    return (
        <AuthProvider>
            <BrowserRouter>
                <div className="min-h-screen bg-black flex flex-col font-sans text-zinc-100 selection:bg-zinc-800 selection:text-white">
                    <Navbar />
                    <main className="flex-1">
                        <Routes>
                            {}
                            <Route path="/" element={<Home />} />
                            <Route path="/signup" element={<Signup />} />
                            <Route path="/signin" element={<Signin />} />

                            {}
                            <Route
                                path="/problems"
                                element={
                                    <ProtectedRoute>
                                        <Problems />
                                    </ProtectedRoute>
                                }
                            />
                            <Route
                                path="/problems/:id"
                                element={
                                    <ProtectedRoute>
                                        <Problem />
                                    </ProtectedRoute>
                                }
                            />
                            <Route
                                path="/contests"
                                element={
                                    <ProtectedRoute>
                                        <Contests />
                                    </ProtectedRoute>
                                }
                            />
                            <Route
                                path="/contests/:id"
                                element={
                                    <ProtectedRoute>
                                        <ContestDetail />
                                    </ProtectedRoute>
                                }
                            />
                            <Route
                                path="/submissions/:id"
                                element={
                                    <ProtectedRoute>
                                        <SubmissionView />
                                    </ProtectedRoute>
                                }
                            />
                            <Route
                                path="/profile/:username"
                                element={
                                    <ProtectedRoute>
                                        <Profile />
                                    </ProtectedRoute>
                                }
                            />
                            <Route
                                path="/profile"
                                element={
                                    <ProtectedRoute>
                                        <Profile />
                                    </ProtectedRoute>
                                }
                            />

                            {}
                            <Route
                                path="/admin"
                                element={
                                    <ProtectedRoute>
                                        <Dashboard />
                                    </ProtectedRoute>
                                }
                            />
                            <Route
                                path="/admin/create_problem"
                                element={
                                    <ProtectedRoute>
                                        <CreateProblem />
                                    </ProtectedRoute>
                                }
                            />
                            <Route
                                path="/admin/edit_problem/:id"
                                element={
                                    <ProtectedRoute>
                                        <EditProblem />
                                    </ProtectedRoute>
                                }
                            />
                            <Route
                                path="/admin/create_contest"
                                element={
                                    <ProtectedRoute>
                                        <CreateContest />
                                    </ProtectedRoute>
                                }
                            />
                            <Route
                                path="/admin/edit_contest/:id"
                                element={
                                    <ProtectedRoute>
                                        <EditContest />
                                    </ProtectedRoute>
                                }
                            />
                        </Routes>
                    </main>
                </div>
            </BrowserRouter>
        </AuthProvider>
    );
}
