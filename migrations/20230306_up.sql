CREATE DATABASE IF NOT EXISTS newsman;

CREATE TABLE IF NOT EXISTS mailing_list (
    email varchar(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS jobs (
    newsletter varchar(255) NOT NULL,
    time DATETIME NOT NULL,
    subject varchar(255) NOT NULL
);
