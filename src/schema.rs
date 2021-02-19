table! {
    categories (id) {
        id -> Uuid,
        name -> Text,
        slug -> Text,
        description -> Text,
        sort_order -> Int2,
    }
}

table! {
    category_products (product_id, category_id) {
        product_id -> Uuid,
        category_id -> Uuid,
    }
}

table! {
    manufacturers (id) {
        id -> Uuid,
        name -> Text,
        slug -> Text,
        website_url -> Text,
    }
}

table! {
    products (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name -> Text,
        slug -> Text,
        description_short -> Text,
        description_long -> Text,
        price -> Int4,
        manufacturer_id -> Nullable<Uuid>,
        status -> Text,
        stock_count -> Int4,
    }
}

joinable!(category_products -> categories (category_id));
joinable!(category_products -> products (product_id));
joinable!(products -> manufacturers (manufacturer_id));

allow_tables_to_appear_in_same_query!(
    categories,
    category_products,
    manufacturers,
    products,
);
