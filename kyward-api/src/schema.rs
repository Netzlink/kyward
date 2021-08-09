table! {
    companies (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
    }
}

table! {
    doors (id) {
        id -> Integer,
        name -> Text,
        compartment -> Text,
        level -> Text,
        building -> Text,
        description -> Text,
    }
}

table! {
    groups (id, door_id) {
        id -> Integer,
        name -> Text,
        door_id -> Integer,
        description -> Text,
    }
}

table! {
    persons (id, group_id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
        company_id -> Integer,
        token_id -> Integer,
        group_id -> Integer,
        description -> Text,
    }
}

table! {
    tokens (id) {
        id -> Integer,
        value -> Text,
        reverse -> Text,
        description -> Text,
        ema -> Text,
    }
}

joinable!(groups -> doors (door_id));

allow_tables_to_appear_in_same_query!(
    companies,
    doors,
    groups,
    persons,
    tokens,
);
