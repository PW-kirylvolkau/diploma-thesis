# drop database test_db with (force);
# create database test_db;
# \c test_db;

create type role as enum (
    'ADMIN', 'TEACHER', 'STUDENT'
);

create table users (
    id integer primary key generated always as identity,
    email VARCHAR(50) not null,
    password VARCHAR(50) not null,
    role role not null
);

create table universities (
    id integer primary key generated always as identity,
    name VARCHAR(100) not null
);

create table university_users (
    user_id integer,
    university_id integer,
    primary key (user_id, university_id),
    foreign key (user_id) references users(id) on delete cascade,
    foreign key (university_id) references universities(id) on delete cascade
);