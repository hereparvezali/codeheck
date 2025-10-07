// import './App.css'
import { BrowserRouter, Route, Routes } from "react-router-dom";
import Navbar from "./components/navbar";
import Signup from "./user/signup";
import Signin from "./user/signin";
import Profile from "./profile/profile";
import Contests from "./contests/contests";
import Problems from "./problems/problems";
import { AuthProvider } from "./utils/contexts/authcontext";
import ProtectedRoute from "./utils/contexts/protected";
import ContestDetail from "./contests/contest";
import Problem from "./problems/problem";
import Dashboard from "./admin/dashboard";
import CreateProblem from "./admin/create_problem";
import CreateContest from "./admin/create_contest";
import EditProblem from "./admin/edit_problem";
import EditContest from "./admin/edit_contest";

const App = () => {
    return (
        <AuthProvider>
            <BrowserRouter>
                <Navbar></Navbar>
                <Routes>
                    <Route path="/signup" element={<Signup />} />
                    <Route path="/signin" element={<Signin />} />
                    <Route
                        path="/"
                        element={
                            <ProtectedRoute>
                                <Profile />
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
                        path="/admin/create_contest"
                        element={
                            <ProtectedRoute>
                                <CreateContest />
                            </ProtectedRoute>
                        }
                    />
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
                        path="/contests/:id"
                        element={
                            <ProtectedRoute>
                                <ContestDetail />
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
                        path="/admin"
                        element={
                            <ProtectedRoute>
                                <Dashboard />
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
                        path="/admin/edit_contest/:id"
                        element={
                            <ProtectedRoute>
                                <EditContest />
                            </ProtectedRoute>
                        }
                    />
                </Routes>
            </BrowserRouter>
        </AuthProvider>
    );
};

export default App;
