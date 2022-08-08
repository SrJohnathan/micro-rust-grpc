-- Your SQL goes here

create table IF NOT EXISTS survey(
 id serial primary key,
 id_team bigint unique  not null,
 title varchar(255) not null,
 created_on varchar(50),
 last_login varchar(50)

);