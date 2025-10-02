# Code Structure Documentation

## Overview
This document describes the restructured codebase architecture for better maintainability, separation of concerns, and scalability.

## Directory Structure

```
src/
├── main.rs              # Application entry point
├── app.rs               # Main application router and middleware setup
├── routes.rs            # Centralized route definitions
├── error.rs             # Error types and handling
├── dto.rs               # Data Transfer Objects
├── entity/              # Database entities (SeaORM models)
├── user/                # User domain
│   ├── mod.rs          # User router (protected routes)
│   ├── auth_router.rs  # Auth sub-router (public routes)
│   ├── signin.rs       # Sign in handler
│   ├── signup.rs       # Sign up handler
│   ├── signout.rs      # Sign out handler
│   ├── refresh_access_token.rs
│   ├── retrieve_*.rs   # Various user retrieve handlers
│   └── dto.rs          # User-specific DTOs
├── problem/             # Problem domain
│   ├── mod.rs          # Problem router
│   ├── create_*.rs     # Creation handlers
│   ├── retrieve_*.rs   # Retrieval handlers
│   └── dto.rs          # Problem-specific DTOs
├── contest/             # Contest domain
│   ├── mod.rs          # Contest router
│   ├── create_*.rs     # Creation handlers
│   ├── retrieve_*.rs   # Retrieval handlers
│   └── dto.rs          # Contest-specific DTOs
├── submission/          # Submission domain
│   ├── mod.rs          # Submission router
│   ├── create_*.rs     # Creation handlers
│   ├── retrieve_*.rs   # Retrieval handlers
│   ├── update_*.rs     # Update handlers
│   └── dto.rs          # Submission-specific DTOs
└── utils/               # Utility modules
    ├── mod.rs
    ├── app_state.rs    # Application state (DB, MQ, etc.)
    ├── config.rs       # Configuration loading
    ├── hashing.rs      # Password hashing utilities
    ├── jwt.rs          # JWT token handling
    ├── logger.rs       # Request logging middleware
    └── middlewares.rs  # Authentication middleware
```

## Key Improvements

### 1. **Separated Error Handling** (`src/error.rs`)
- Moved `MyErr` (now `AppError`) to dedicated error module
- Better separation from DTOs
- Easier to extend with new error types
- Re-exported as `MyErr` for backward compatibility

### 2. **Auth Sub-Module** (`src/user/auth.rs`)
- Authentication routes separated from protected user routes
- Clear distinction between public and protected endpoints
- Easier to apply different middleware to different route groups

### 3. **Centralized Routing** (`src/routes.rs`)
- All route definitions in one place
- Clear separation between protected and public routes
- Easier to understand the API structure at a glance
- Simplified `app.rs`

### 4. **Improved App Structure** (`src/app.rs`)
- Cleaner and more concise
- Clear middleware layering
- Added `/health` endpoint for health checks
- Better organized with comments

### 5. **Request Logging** (`src/utils/logger.rs`)
- Simple custom logger middleware
- Logs: method, URI, status code, and request duration
- Visual indicators (emojis) for different status codes
- No external dependencies required

## API Route Structure

All routes are now prefixed with `/api`:

### Public Routes (No Authentication Required)
- `POST /api/user/signin` - User sign in
- `POST /api/user/signup` - User sign up
- `GET /api/user/signout` - User sign out
- `GET /api/user/refresh` - Refresh access token

### Protected Routes (Authentication Required)

#### User Routes
- `GET /api/user/retrieve` - Get current user info
- `GET /api/user/retrieve_user` - Get user details
- `GET /api/user/retrieve_problems` - Get user's problems
- `GET /api/user/retrieve_contests` - Get user's contests
- `GET /api/user/retrieve_solved` - Get user's solved problems

#### Problem Routes
- `POST /api/problem/create` - Create new problem
- `POST /api/problem/create_testcases` - Add test cases
- `GET /api/problem/retrieve` - Get problem details
- `GET /api/problem/retrieve_problems` - List all problems

#### Contest Routes
- `POST /api/contest/create` - Create new contest
- `GET /api/contest/retrieve` - Get contest details
- `GET /api/contest/retrieve_contests` - List all contests
- `GET /api/contest/add_problem` - Add problem to contest
- `GET /api/contest/retrieve_problems` - Get contest problems
- `POST /api/contest/create_register/{contest_id}` - Register for contest
- `DELETE /api/contest/delete_register/{id}` - Unregister from contest

#### Submission Routes
- `POST /api/submission/create` - Submit solution
- `GET /api/submission/retrieve/{id}` - Get submission details
- `GET /api/submissions/retrieve` - List submissions
- `PUT /api/submission/update` - Update submission status

### Utility Routes
- `GET /health` - Health check endpoint

## Middleware Stack

The middleware is applied in the following order (outer to inner):

1. **Logger** - Logs all requests with timing
2. **Timeout** - 5 second timeout for all requests
3. **Cookie Manager** - Handles cookies
4. **CORS** - Very permissive CORS policy
5. **Authorizer** - JWT authentication (only on protected routes)

## Benefits of This Structure

### 1. **Modularity**
- Each domain (user, problem, contest, submission) is self-contained
- Easy to add new domains or remove existing ones
- Changes in one domain don't affect others

### 2. **Maintainability**
- Clear separation of concerns
- Easy to locate specific functionality
- Consistent structure across all domains

### 3. **Scalability**
- Easy to add new routes or handlers
- Simple to apply domain-specific middleware
- Can split into microservices later if needed

### 4. **Security**
- Clear distinction between public and protected routes
- Authentication middleware applied only where needed
- Easy to audit security measures

### 5. **Developer Experience**
- Intuitive project structure
- Easy onboarding for new developers
- Self-documenting code organization

## Configuration

The application uses environment variables loaded via `dotenvy`:

Required variables:
- `DATABASE_URL` - PostgreSQL connection string
- `RABBITMQ_URL` - RabbitMQ connection string
- `SECRET` - JWT secret key

## Running the Application

```bash
# Set up environment variables
cp .env.example .env
# Edit .env with your configuration

# Run migrations
cargo run -p migration

# Start the server
cargo run

# Server will listen on 0.0.0.0:8000
```

## Testing Endpoints

```bash
# Health check
curl http://localhost:8000/health

# Sign up
curl -X POST http://localhost:8000/api/user/signup \
  -H "Content-Type: application/json" \
  -d '{"username":"test","email":"test@example.com","password":"password123"}'

# Sign in
curl -X POST http://localhost:8000/api/user/signin \
  -H "Content-Type: application/json" \
  -d '{"username_or_email":"test","password":"password123"}'
```

## Future Improvements

1. **Add proper logging framework** (tracing, env_logger)
2. **Rate limiting middleware**
3. **Request validation middleware**
4. **API versioning** (/api/v1, /api/v2)
5. **Swagger/OpenAPI documentation**
6. **Unit and integration tests**
7. **Graceful shutdown handling**
8. **Database connection pooling optimization**
9. **Caching layer** (Redis)
10. **Metrics and monitoring** (Prometheus)

## Notes

- All handlers follow the same pattern for consistency
- Error handling is centralized through `AppError`
- State is shared across all routes via `AppState`
- JWT tokens are used for authentication
- Refresh tokens are stored in HTTP-only cookies
