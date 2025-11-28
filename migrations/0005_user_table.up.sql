create table users (
    id serial primary key,
    username text not null,
    password text not null,
    email text not null
);