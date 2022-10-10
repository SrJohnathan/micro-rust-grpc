-- Your SQL goes here
CREATE TABLE IF NOT EXISTS auth (
  id serial primary key,
 name varchar(50)  not null,
 password varchar(50) not null,
 numero varchar(15),
 serie varchar(15) unique not null,
 email varchar(50) unique not null,
 id_firebase varchar(90) unique not null,
 token_firebase varchar(255) unique,
 created_on varchar(50),
 last_login varchar(50)

)
