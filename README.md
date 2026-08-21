# CodeHeck

**CodeHeck** is a high-performance, distributed Online Judge and Competitive Programming platform built with Rust, Axum, SeaORM, PostgreSQL, RabbitMQ, Isolate Sandbox Isolation, and React 19 with TypeScript and Monaco Editor.

---

## 🏛 System Architecture

```mermaid
graph TD
    User([Browser Client]) <-->|React 19 + TypeScript + Monaco| Frontend[frontend (Vite on :5173)]
    Frontend <-->|REST API + JWT Bearer| APIServer[api_server (Axum on :8080)]
    APIServer <-->|SeaORM 2.0| Postgres[(PostgreSQL DB on :5432)]
    APIServer -->|Publish Submissions| RabbitMQ[[RabbitMQ Queue on :5672]]
    RabbitMQ -->|Consume Submissions| Worker[worker (Rust Engine)]
    Worker -->|Two-Stage Isolate Sandbox Execution| IsolateSandbox[Isolate Sandboxes (g++20, rustc, openjdk, python3, go, node)]
    Worker -->|RabbitMQ 'incomming' Queue| APIServer
```

---

## 🚀 Key Features

- **Multi-Language Judge Engine**: Supports C++ (g++ 20), Python 3, Rust, Java (OpenJDK), Go, and Node.js.
- **Two-Stage Isolate Sandbox Execution**: Isolated compilation stage (clean `CE` diagnostics) followed by microsecond-precise, resource-limited runtime execution inside Linux namespaces and cgroups (`--processes=64`, `--open-files=64`, `--time`, `--mem`).
- **Granular Verdict Reporting**: `AC` (Accepted), `WA` (Wrong Answer), `TLE` (Time Limit Exceeded), `MLE` (Memory Limit Exceeded), `CE` (Compilation Error), `RE` (Runtime Error), `PENDING` (Judging).
- **Public Browsing & Optional Auth**: Guests can freely explore problems, view contests, and inspect live standings; authentication seamlessly unlocks code submission, rating tracking, and authoring tools.
- **Competitive Contests Arena**: Live countdown timers, registration management, contest problem sets, and real-time automated penalty leaderboards.
- **In-Browser IDE**: Monaco Editor workspace with starter templates per language, problem statement viewer, and real-time verdict polling.
- **Comprehensive User Profile & Statistics**: Solved problems breakdown by difficulty (Easy, Medium, Hard), acceptance rate, contest rating, and full submission history with source code viewer.
- **Admin Management Panel**: Full problem authoring with sample cases and hidden evaluation testcases, as well as contest creation and problem linking.

---

## 🛠 Tech Stack

| Layer | Technologies |
| :--- | :--- |
| **Backend API** | Rust 2024, Axum 0.8, SeaORM 2.0-rc, Lapin (RabbitMQ), Tokio, Bcrypt, JWT |
| **Judging Worker** | Rust 2024, Lapin, Isolate Sandbox Engine, Tokio |
| **Frontend** | React 19, TypeScript, Vite, Tailwind CSS v4, Monaco Editor, React Router v7 |
| **Infrastructure** | PostgreSQL 16 (Alpine), RabbitMQ (Alpine), Isolate Sandbox |

---

## 🏃 Getting Started

### 1. Prerequisites
- [Rust](https://www.rust-lang.org/) (stable)
- [Node.js](https://nodejs.org/) (v18+) & `npm`
- [Docker](https://www.docker.com/) & Docker Compose

### 2. Start Database & RabbitMQ
```bash
docker compose up -d
```

### 3. Start the API Server
```bash
cd api_server
cargo run
```

### 4. Start the Evaluation Worker
```bash
cd worker
cargo run
```

### 5. Start the Frontend
```bash
cd frontend
npm install
npm run dev
```

Visit `http://localhost:5173` to explore CodeHeck!

---

## 📁 Repository Structure

```
codeheck/
├── api_server/          # Rust Axum Backend & SeaORM migrations
│   ├── migration/       # Database migrations (users, problems, testcases, contests, submissions)
│   └── src/             # API routes, entities, security, auth middleware, and handlers
├── worker/              # Rust Submissions Consumer & Docker Judge Engine
│   └── src/             # Queue handler, docker runner, compilers, and language matrix
├── frontend/            # React 19 + TypeScript frontend application
│   └── src/             # Home, Problems, Contests, Submissions, Profiles, Admin, Components
└── docker-compose.yml   # PostgreSQL and RabbitMQ orchestration
```
