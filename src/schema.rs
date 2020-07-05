table! {
    ingredient (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        image_url -> Varchar,
        archived -> Bool,
    }
}

table! {
    ingredient_tag (ingredient_id, name) {
        ingredient_id -> Int4,
        name -> Varchar,
    }
}

table! {
    recipe (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        image_url -> Varchar,
        archived -> Bool,
    }
}

table! {
    recipe_ingredient (id) {
        id -> Int4,
        ingredient_id -> Int4,
        recipe_id -> Int4,
        quantity -> Numeric,
        unit -> Int2,
        archived -> Bool,
    }
}

table! {
    recipe_tag (recipe_id, name) {
        recipe_id -> Int4,
        name -> Varchar,
    }
}

joinable!(ingredient_tag -> ingredient (ingredient_id));
joinable!(recipe_ingredient -> ingredient (ingredient_id));
joinable!(recipe_ingredient -> recipe (recipe_id));
joinable!(recipe_tag -> recipe (recipe_id));

allow_tables_to_appear_in_same_query!(
    ingredient,
    ingredient_tag,
    recipe,
    recipe_ingredient,
    recipe_tag,
);
