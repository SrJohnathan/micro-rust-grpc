// @generated automatically by Diesel CLI.


diesel::table! {
    auth (id) {
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
