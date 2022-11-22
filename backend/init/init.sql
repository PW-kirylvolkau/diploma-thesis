create table users (
    user_id serial primary key,
    email VARCHAR(50) not null,
    password VARCHAR(50) not null
);