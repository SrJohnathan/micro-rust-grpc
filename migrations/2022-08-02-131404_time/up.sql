-- Your SQL goes here

create table IF NOT EXISTS team(
 id serial primary key,
 id_user bigint unique  not null,
 path_img varchar(255),
 created_on varchar(50),
 last_login varchar(50)

);