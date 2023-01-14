create database test_db;
\c test_db;

create type role as enum (
    'ADMIN', 'TEACHER', 'STUDENT'
);

create table users (
    id integer primary key generated always as identity,
    email text not null,
    password varchar(50) not null,
    role role not null
);

-- create table university_users (
--     user_id integer,
--     university_id integer,
--     primary key (user_id, university_id),
--     foreign key (user_id) references users(id) on delete cascade,
--     foreign key (university_id) references universities(id) on delete cascade
-- );

create type lesson_type as enum (
    'VIDEO', 'ATTACHMENT', 'TEXT'
);

create table lessons (
    id integer primary key generated always as identity,
    name text not null,
    text text not null,
    type lesson_type not null,
    resource_url text
);

create table courses (
    id integer primary key generated always as identity,
    name text not null,
    tile_url text not null,
    is_public boolean not null
);

create table universities (
    id integer primary key generated always as identity,
    name varchar not null
);

create table university_courses (
    uni_id integer not null,
    course_id integer not null,
    primary key (uni_id, course_id),
    foreign key (uni_id) references universities(id) on delete cascade,
    foreign key (course_id) references courses(id) on delete cascade
);

create table course_lessons (
    course_id integer not null,
    lesson_id integer not null,
    seq integer not null,
    primary key (course_id, lesson_id, seq),
    foreign key (course_id) references courses(id) on delete cascade,
    foreign key (lesson_id) references lessons(id) on delete cascade
);