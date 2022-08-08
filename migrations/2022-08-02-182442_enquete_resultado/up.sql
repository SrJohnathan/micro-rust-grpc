-- Your SQL goes here

create table IF NOT EXISTS survey_res(
 id serial primary key,
 id_survey bigint unique  not null,
 id_survey_content bigint unique not null,
 id_user bigint unique  not null,
 created_on varchar(50),
 last_login varchar(50),
 CONSTRAINT enquete
      FOREIGN KEY(id_survey)
	  REFERENCES survey(id) ON DELETE CASCADE

);