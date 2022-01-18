-- Your SQL goes here

CREATE TYPE role as ENUM('conseilier','client','admin');

CREATE TABLE customers(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    surname TEXT NOT NULL,
    email TEXT NOT NULL,
    role role DEFAULT 'client'
);

CREATE TABLE addresses(
    id SERIAL PRIMARY KEY,
    city TEXT NOT NULL,
    street TEXT NOT NULL,
    number TEXT NOT NULL,
    UNIQUE(city, street, number)
);

CREATE TABLE housings(
    id SERIAL PRIMARY KEY,
    id_address INTEGER NOT NULL REFERENCES addresses(id),
    surface NUMERIC(8,2) NOT NULL
);

CREATE TABLE folders(
    id SERIAL PRIMARY KEY,
    folder_num TEXT NOT NULL,
    id_customer INTEGER NOT NULL REFERENCES customers(id),
    id_housing INTEGER NOT NULL REFERENCES housings(id),
    tax_income NUMERIC(20,2) NOT NULL,
    situation TEXT NOT NULL
);

CREATE TABLE appointements(
    id SERIAL PRIMARY KEY,
    id_folder INTEGER NOT NULL REFERENCES folders(id),
    date TIMESTAMP NOT NULL DEFAULT now(),
    note TEXT
);

CREATE TABLE projects(
    id SERIAL PRIMARY KEY,
    id_folder INTEGER NOT NULL REFERENCES folders(id),
    slug TEXT,
    name TEXT NOT NULL,
    reason TEXT
);

