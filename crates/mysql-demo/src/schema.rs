use diesel::table;

table! {
    article (id) {
        id -> Integer,
        title -> Varchar,
        description -> Varchar,
        content -> Longtext,
        is_deleted -> Bool,
    }
}

table! {
    auth_group (id) {
        id -> Integer,
        name -> Varchar,
    }
}
