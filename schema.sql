-- -- users table
-- CREATE TABLE users (
--     id INT AUTO_INCREMENT PRIMARY KEY,
--     username VARCHAR(50) NOT NULL UNIQUE,
--     email VARCHAR(100) NOT NULL UNIQUE,
--     password_hash TEXT NOT NULL,
--     role ENUM('user', 'admin') DEFAULT 'user',
--     rating INT DEFAULT 1500,
--     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
-- );

-- -- problems table
-- CREATE TABLE problems (
--     id INT AUTO_INCREMENT PRIMARY KEY,
--     title VARCHAR(255) NOT NULL,
--     slug VARCHAR(255) NOT NULL UNIQUE,
--     statement TEXT NOT NULL,
--     input_spec TEXT,
--     output_spec TEXT,
--     constraints TEXT,
--     sample_inputs JSON,
--     difficulty ENUM('easy', 'medium', 'hard') NOT NULL,
--     is_public BOOLEAN DEFAULT TRUE,
--     author_id INT,
--     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
--     FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE SET NULL
-- );

-- -- testcases table
-- CREATE TABLE test_cases (
--     id INT AUTO_INCREMENT PRIMARY KEY,
--     problem_id INT NOT NULL,
--     input_data TEXT NOT NULL,
--     expected_output TEXT NOT NULL,
--     is_sample BOOLEAN DEFAULT FALSE,
--     FOREIGN KEY (problem_id) REFERENCES problems(id) ON DELETE CASCADE
-- );

-- -- submissions table
-- CREATE TABLE submissions (
--     id INT AUTO_INCREMENT PRIMARY KEY,
--     user_id INT NOT NULL,
--     problem_id INT NOT NULL,
--     language VARCHAR(20) NOT NULL,
--     code TEXT NOT NULL,
--     status ENUM(
--         'pending',
--         'accepted',
--         'wrong_answer',
--         'runtime_error',
--         'compile_error',
--         'time_limit_exceeded'
--     ) DEFAULT 'pending',
--     verdict TEXT,
--     time_taken_ms INT,
--     memory_kb INT,
--     submitted_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
--     contest_id INT,
--     FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
--     FOREIGN KEY (problem_id) REFERENCES problems(id) ON DELETE CASCADE,
--     FOREIGN KEY (contest_id) REFERENCES contests(id) ON DELETE SET NULL
-- );

-- -- contests table
-- CREATE TABLE contests (
--     id INT AUTO_INCREMENT PRIMARY KEY,
--     title VARCHAR(255) NOT NULL,
--     slug VARCHAR(255) NOT NULL UNIQUE,
--     description TEXT,
--     start_time DATETIME NOT NULL,
--     end_time DATETIME NOT NULL,
--     is_public BOOLEAN DEFAULT TRUE,
--     created_by INT,
--     FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE SET NULL
-- );

-- -- contest_problems table
-- CREATE TABLE contest_problems (
--     contest_id INT NOT NULL,
--     problem_id INT NOT NULL,
--     label VARCHAR(5), -- A, B, C...
--     PRIMARY KEY (contest_id, problem_id),
--     FOREIGN KEY (contest_id) REFERENCES contests(id) ON DELETE CASCADE,
--     FOREIGN KEY (problem_id) REFERENCES problems(id) ON DELETE CASCADE
-- );

-- -- contest_registrations table
-- CREATE TABLE contest_registrations (
--     id INT AUTO_INCREMENT PRIMARY KEY,
--     user_id INT NOT NULL,
--     contest_id INT NOT NULL,
--     registered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
--     FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
--     FOREIGN KEY (contest_id) REFERENCES contests(id) ON DELETE CASCADE
-- );

-- ratings_history table
CREATE TABLE ratings_history (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL,
    contest_id INT NOT NULL,
    old_rating INT,
    new_rating INT,
    delta INT,
    changed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (contest_id) REFERENCES contests(id) ON DELETE CASCADE
);

-- problem_tags table
CREATE TABLE problem_tags (
    id INT AUTO_INCREMENT PRIMARY KEY,
    problem_id INT NOT NULL,
    tag VARCHAR(50),
    FOREIGN KEY (problem_id) REFERENCES problems(id) ON DELETE CASCADE
);

-- editorials table
CREATE TABLE editorials (
    id INT AUTO_INCREMENT PRIMARY KEY,
    problem_id INT NOT NULL,
    content TEXT NOT NULL,
    created_by INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (problem_id) REFERENCES problems(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE SET NULL
);
