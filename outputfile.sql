--
-- PostgreSQL database dump
--

-- Dumped from database version 17.5 (Debian 17.5-1.pgdg120+1)
-- Dumped by pg_dump version 17.5

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: contest_problems; Type: TABLE; Schema: public; Owner: parvez
--

CREATE TABLE public.contest_problems (
    contest_id bigint NOT NULL,
    problem_id bigint NOT NULL,
    label character varying(1)
);


ALTER TABLE public.contest_problems OWNER TO parvez;

--
-- Name: contest_registrations; Type: TABLE; Schema: public; Owner: parvez
--

CREATE TABLE public.contest_registrations (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    contest_id bigint NOT NULL,
    registered_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.contest_registrations OWNER TO parvez;

--
-- Name: contest_registrations_id_seq; Type: SEQUENCE; Schema: public; Owner: parvez
--

CREATE SEQUENCE public.contest_registrations_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.contest_registrations_id_seq OWNER TO parvez;

--
-- Name: contest_registrations_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: parvez
--

ALTER SEQUENCE public.contest_registrations_id_seq OWNED BY public.contest_registrations.id;


--
-- Name: contests; Type: TABLE; Schema: public; Owner: parvez
--

CREATE TABLE public.contests (
    id bigint NOT NULL,
    title character varying(128) NOT NULL,
    slug character varying(128) NOT NULL,
    description text,
    start_time timestamp without time zone NOT NULL,
    end_time timestamp without time zone NOT NULL,
    is_public boolean DEFAULT true NOT NULL,
    author_id bigint
);


ALTER TABLE public.contests OWNER TO parvez;

--
-- Name: contests_id_seq; Type: SEQUENCE; Schema: public; Owner: parvez
--

CREATE SEQUENCE public.contests_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.contests_id_seq OWNER TO parvez;

--
-- Name: contests_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: parvez
--

ALTER SEQUENCE public.contests_id_seq OWNED BY public.contests.id;


--
-- Name: problems; Type: TABLE; Schema: public; Owner: parvez
--

CREATE TABLE public.problems (
    id bigint NOT NULL,
    title character varying(128) NOT NULL,
    slug character varying(128) NOT NULL,
    statement text,
    input_spec text,
    output_spec text,
    sample_inputs json,
    time_limit smallint DEFAULT 1000 NOT NULL,
    memory_limit smallint DEFAULT 256 NOT NULL,
    difficulty character varying(8),
    is_public boolean DEFAULT false NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    author_id bigint
);


ALTER TABLE public.problems OWNER TO parvez;

--
-- Name: problems_id_seq; Type: SEQUENCE; Schema: public; Owner: parvez
--

CREATE SEQUENCE public.problems_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.problems_id_seq OWNER TO parvez;

--
-- Name: problems_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: parvez
--

ALTER SEQUENCE public.problems_id_seq OWNED BY public.problems.id;


--
-- Name: seaql_migrations; Type: TABLE; Schema: public; Owner: parvez
--

CREATE TABLE public.seaql_migrations (
    version character varying NOT NULL,
    applied_at bigint NOT NULL
);


ALTER TABLE public.seaql_migrations OWNER TO parvez;

--
-- Name: submissions; Type: TABLE; Schema: public; Owner: parvez
--

CREATE TABLE public.submissions (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    problem_id bigint NOT NULL,
    language character varying(16) NOT NULL,
    code text NOT NULL,
    status character varying(20) DEFAULT 'pending'::character varying NOT NULL,
    verdict text,
    "time" smallint,
    memory smallint,
    submitted_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    contest_id bigint
);


ALTER TABLE public.submissions OWNER TO parvez;

--
-- Name: submissions_id_seq; Type: SEQUENCE; Schema: public; Owner: parvez
--

CREATE SEQUENCE public.submissions_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.submissions_id_seq OWNER TO parvez;

--
-- Name: submissions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: parvez
--

ALTER SEQUENCE public.submissions_id_seq OWNED BY public.submissions.id;


--
-- Name: testcases; Type: TABLE; Schema: public; Owner: parvez
--

CREATE TABLE public.testcases (
    id bigint NOT NULL,
    problem_id bigint NOT NULL,
    input text,
    output text
);


ALTER TABLE public.testcases OWNER TO parvez;

--
-- Name: testcases_id_seq; Type: SEQUENCE; Schema: public; Owner: parvez
--

CREATE SEQUENCE public.testcases_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.testcases_id_seq OWNER TO parvez;

--
-- Name: testcases_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: parvez
--

ALTER SEQUENCE public.testcases_id_seq OWNED BY public.testcases.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: parvez
--

CREATE TABLE public.users (
    id bigint NOT NULL,
    email character varying(64) NOT NULL,
    username character varying(32) NOT NULL,
    password character varying(60) NOT NULL,
    rating integer DEFAULT 0 NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.users OWNER TO parvez;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: parvez
--

CREATE SEQUENCE public.users_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.users_id_seq OWNER TO parvez;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: parvez
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: contest_registrations id; Type: DEFAULT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.contest_registrations ALTER COLUMN id SET DEFAULT nextval('public.contest_registrations_id_seq'::regclass);


--
-- Name: contests id; Type: DEFAULT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.contests ALTER COLUMN id SET DEFAULT nextval('public.contests_id_seq'::regclass);


--
-- Name: problems id; Type: DEFAULT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.problems ALTER COLUMN id SET DEFAULT nextval('public.problems_id_seq'::regclass);


--
-- Name: submissions id; Type: DEFAULT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.submissions ALTER COLUMN id SET DEFAULT nextval('public.submissions_id_seq'::regclass);


--
-- Name: testcases id; Type: DEFAULT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.testcases ALTER COLUMN id SET DEFAULT nextval('public.testcases_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Name: contest_problems contest_problems_pkey; Type: CONSTRAINT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.contest_problems
    ADD CONSTRAINT contest_problems_pkey PRIMARY KEY (contest_id, problem_id);


--
-- Name: contest_registrations contest_registrations_pkey; Type: CONSTRAINT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.contest_registrations
    ADD CONSTRAINT contest_registrations_pkey PRIMARY KEY (id);


--
-- Name: contests contests_pkey; Type: CONSTRAINT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.contests
    ADD CONSTRAINT contests_pkey PRIMARY KEY (id);


--
-- Name: contests contests_slug_key; Type: CONSTRAINT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.contests
    ADD CONSTRAINT contests_slug_key UNIQUE (slug);


--
-- Name: problems problems_pkey; Type: CONSTRAINT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.problems
    ADD CONSTRAINT problems_pkey PRIMARY KEY (id);


--
-- Name: problems problems_slug_key; Type: CONSTRAINT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.problems
    ADD CONSTRAINT problems_slug_key UNIQUE (slug);


--
-- Name: seaql_migrations seaql_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.seaql_migrations
    ADD CONSTRAINT seaql_migrations_pkey PRIMARY KEY (version);


--
-- Name: submissions submissions_pkey; Type: CONSTRAINT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.submissions
    ADD CONSTRAINT submissions_pkey PRIMARY KEY (id);


--
-- Name: testcases testcases_pkey; Type: CONSTRAINT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.testcases
    ADD CONSTRAINT testcases_pkey PRIMARY KEY (id);


--
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (email);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: users users_username_key; Type: CONSTRAINT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_username_key UNIQUE (username);


--
-- Name: contest_problems_contest_id_idx; Type: INDEX; Schema: public; Owner: parvez
--

CREATE INDEX contest_problems_contest_id_idx ON public.contest_problems USING btree (contest_id);


--
-- Name: contest_problems_problem_id_idx; Type: INDEX; Schema: public; Owner: parvez
--

CREATE INDEX contest_problems_problem_id_idx ON public.contest_problems USING btree (problem_id);


--
-- Name: contest_registrations_contest_id_idx; Type: INDEX; Schema: public; Owner: parvez
--

CREATE INDEX contest_registrations_contest_id_idx ON public.contest_registrations USING btree (contest_id);


--
-- Name: contest_registrations_user_id_idx; Type: INDEX; Schema: public; Owner: parvez
--

CREATE INDEX contest_registrations_user_id_idx ON public.contest_registrations USING btree (user_id);


--
-- Name: contests_author_id_idx; Type: INDEX; Schema: public; Owner: parvez
--

CREATE INDEX contests_author_id_idx ON public.contests USING btree (author_id);


--
-- Name: problems_author_id_idx; Type: INDEX; Schema: public; Owner: parvez
--

CREATE INDEX problems_author_id_idx ON public.problems USING btree (author_id);


--
-- Name: submissions_contest_id_idx; Type: INDEX; Schema: public; Owner: parvez
--

CREATE INDEX submissions_contest_id_idx ON public.submissions USING btree (contest_id);


--
-- Name: submissions_problem_id_idx; Type: INDEX; Schema: public; Owner: parvez
--

CREATE INDEX submissions_problem_id_idx ON public.submissions USING btree (problem_id);


--
-- Name: submissions_user_id_idx; Type: INDEX; Schema: public; Owner: parvez
--

CREATE INDEX submissions_user_id_idx ON public.submissions USING btree (user_id);


--
-- Name: testcases_problem_id_idx; Type: INDEX; Schema: public; Owner: parvez
--

CREATE INDEX testcases_problem_id_idx ON public.testcases USING btree (problem_id);


--
-- Name: contest_problems contest_problems_contest_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.contest_problems
    ADD CONSTRAINT contest_problems_contest_id_fkey FOREIGN KEY (contest_id) REFERENCES public.contests(id) ON DELETE CASCADE;


--
-- Name: contest_problems contest_problems_problem_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.contest_problems
    ADD CONSTRAINT contest_problems_problem_id_fkey FOREIGN KEY (problem_id) REFERENCES public.problems(id) ON DELETE CASCADE;


--
-- Name: contest_registrations contest_registrations_contest_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.contest_registrations
    ADD CONSTRAINT contest_registrations_contest_id_fkey FOREIGN KEY (contest_id) REFERENCES public.contests(id) ON DELETE CASCADE;


--
-- Name: contest_registrations contest_registrations_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.contest_registrations
    ADD CONSTRAINT contest_registrations_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: contests contests_author_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.contests
    ADD CONSTRAINT contests_author_id_fkey FOREIGN KEY (author_id) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: problems problems_author_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.problems
    ADD CONSTRAINT problems_author_id_fkey FOREIGN KEY (author_id) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: submissions submissions_contest_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.submissions
    ADD CONSTRAINT submissions_contest_id_fkey FOREIGN KEY (contest_id) REFERENCES public.contests(id) ON DELETE SET NULL;


--
-- Name: submissions submissions_problem_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: parvez
--
b(Conte
ALTER TABLE ONLY public.submissions
    ADD CONSTRAINT submissions_problem_id_fkey FOREIGN KEY (problem_id) REFERENCES public.problems(id) ON DELETE CASCADE;


--
-- Name: submissions submissions_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.submissions
    ADD CONSTRAINT submissions_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: testcases testcases_problem_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: parvez
--

ALTER TABLE ONLY public.testcases
    ADD CONSTRAINT testcases_problem_id_fkey FOREIGN KEY (problem_id) REFERENCES public.problems(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

