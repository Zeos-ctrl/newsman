DROP DATABASE IF EXISTS newsman;
CREATE DATABASE IF NOT EXISTS newsman;

USE newsman;

CREATE TABLE IF NOT EXISTS mailing_list (
    token varchar(255) NOT NULL PRIMARY KEY,
    email varchar(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS jobs (
    newsletter varchar(255) NOT NULL,
    time BIGINT NOT NULL,
    subject varchar(255) NOT NULL
);
