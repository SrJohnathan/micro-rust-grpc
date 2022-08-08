-- Your SQL goes here
create table IF NOT EXISTS score(
 id serial primary key,
 id_user bigint unique  not null,
 team_pri bigint unique  not null,
 team_secu bigint unique  not null,
 created_on varchar(50),
 last_login varchar(50)

);