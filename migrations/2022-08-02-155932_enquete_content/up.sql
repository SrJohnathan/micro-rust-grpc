-- Your SQL goes here

create table IF NOT EXISTS survey_content(
 id serial primary key,
 id_survey bigint unique  not null,
 content varchar(255) not null,
 path_img varchar(255) not null,
 created_on varchar(50),
 last_login varchar(50),
 CONSTRAINT enquete
      FOREIGN KEY(id_survey)
	  REFERENCES survey(id) ON DELETE CASCADE
);