

CREATE DATABASE epoc_main;

CREATE TABLE IF NOT EXISTS metadata(
                                       id SERIAL PRIMARY KEY,
                                       key text,
                                       value text,
                                       UNIQUE (key)
);

CREATE TABLE IF NOT EXISTS category(
                                       id SERIAL PRIMARY KEY,
                                       name text,
                                       priority int,
                                       description text,
                                       UNIQUE (name)
);

CREATE TABLE IF NOT EXISTS questions
(
    id          SERIAL,
    key         text PRIMARY KEY,
    question    text NOT NULL,
    answer_type text NOT NULL,
    category    int,
    constraint fk_category FOREIGN KEY(category) REFERENCES category (id),
    max         int,
    min         int,
    show        boolean not null,
    display_name text not null,
    is_positive boolean not null,
    UNIQUE (key)
);

CREATE TABLE IF NOT EXISTS options
(
    id         SERIAL PRIMARY KEY,
    name        text,
    question_key text,
    constraint fk_question FOREIGN key(question_key) references questions (key)
);

CREATE TABLE IF NOT EXISTS user_data
(
    id          SERIAL PRIMARY KEY,
    username text,
    email_id text
);

CREATE TABLE IF NOT EXISTS user_question_map
(
    id          SERIAL PRIMARY KEY,
    user_id     int,
    constraint fk_user FOREIGN KEY(user_id) REFERENCES user_data (id),
    question_key text,
    constraint fk_question FOREIGN KEY(question_key) REFERENCES questions (key),
    UNIQUE (user_id, question_key)
);


CREATE TABLE IF NOT EXISTS raw_data (
    id SERIAL PRIMARY KEY,
    timestamp bigint,
    "yearmonth" int,
    "yearweek" int,
    "year" smallint,
    "quarter" smallint,
    "month" smallint,
    "day" smallint,
    "hour" smallint,
    "minute" smallint,
    "week" smallint,
    "key" text,
    "question" text,
    "type" text,
    "value" text,
    "matcheddate" date,
    "source" text
);

CREATE TABLE IF NOT EXISTS last_run (
    id SERIAL PRIMARY KEY,
    command text,
    last_run bigint,
    last_message bigint,
    UNIQUE (command)
);

INSERT INTO category (name, priority, description) VALUES
('Mental Health', 1, 'Health and wellbeing'),
('Physical Health', 2, 'Health and wellbeing'),
('Productivity', 3, 'Work and hobbies'),
('Hobbies', 4, 'Work and hobbies'),
('Social', 5, 'Relationships')

