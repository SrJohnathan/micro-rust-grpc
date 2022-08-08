table! {
    score (id) {
        id -> Int4,
        id_user -> Int8,
        team_pri -> Int8,
        team_secu -> Int8,
        created_on -> Nullable<Varchar>,
        last_login -> Nullable<Varchar>,
    }
}

table! {
    survey (id) {
        id -> Int4,
        id_team -> Int8,
        title -> Varchar,
        created_on -> Nullable<Varchar>,
        last_login -> Nullable<Varchar>,
    }
}

table! {
    survey_content (id) {
        id -> Int4,
        id_survey -> Int8,
        content -> Varchar,
        path_img -> Varchar,
        created_on -> Nullable<Varchar>,
        last_login -> Nullable<Varchar>,
    }
}

table! {
    survey_res (id) {
        id -> Int4,
        id_survey -> Int8,
        id_survey_content -> Int8,
        id_user -> Int8,
        created_on -> Nullable<Varchar>,
        last_login -> Nullable<Varchar>,
    }
}

table! {
    team (id) {
        id -> Int4,
        id_user -> Int8,
        path_img -> Nullable<Varchar>,
        created_on -> Nullable<Varchar>,
        last_login -> Nullable<Varchar>,
    }
}

table! {
    usuario (id) {
        id -> Int4,
        name -> Varchar,
        password -> Varchar,
        numero -> Nullable<Varchar>,
        serie -> Varchar,
        email -> Varchar,
        id_firebase -> Varchar,
        token_firebase -> Nullable<Varchar>,
        created_on -> Nullable<Varchar>,
        last_login -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    score,
    survey,
    survey_content,
    survey_res,
    team,
    usuario,
);
