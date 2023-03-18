

CREATE DATABASE IF NOT EXISTS epoc_main;

CREATE TABLE IF NOT EXISTS questions
(
    id          SERIAL PRIMARY KEY,
    key         text PRIMARY KEY,
    question    text NOT NULL,
    answer_type text NOT NULL,
    category    int FOREIGN KEY REFERENCES category (id),
    max         int,
    min         int,
    show        boolean,
    UNIQUE (key)

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
    "source" text,
    "importedat" timestamp,
    "importid" text
);

-- Table Definition ----------------------------------------------

CREATE TABLE IF NOT EXISTS last_run (
    id SERIAL PRIMARY KEY,
    command text,
    last_run bigint,
    last_message bigint,
    UNIQUE (command)
);


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

INSERT INTO category (name, priority, description) VALUES
('Mental Health', 1, 'Health and wellbeing'),
('Physical Health', 2, 'Health and wellbeing'),
('Productivity', 3, 'Work and hobbies'),
('Hobbies', 4, 'Work and hobbies'),
('Social', 5, 'Relationships')

