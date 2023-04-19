table! {
    cards (id) {
        id -> Varchar,
        user_id -> Varchar,
        product_id -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

table! {
    categories (id) {
        id -> Varchar,
        name -> Varchar,
        parent -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

table! {
    category_aggregations (id) {
        id -> Varchar,
        category_id -> Varchar,
        products -> Int8,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

table! {
    comments (id) {
        id -> Varchar,
        user_id -> Varchar,
        product_id -> Varchar,
        comment -> Nullable<Text>,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

table! {
    identifies (id) {
        id -> Varchar,
        user_id -> Varchar,
        usr_name -> Varchar,
        usr_pwd -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

table! {
    order_items (id) {
        id -> Varchar,
        order_id -> Varchar,
        product_id -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

table! {
    payments (id) {
        id -> Varchar,
        order_id -> Varchar,
        amount -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

table! {
    products (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        price -> Int8,
        quantity -> Int8,
        category_id -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

table! {
    user_aggregations (id) {
        id -> Varchar,
        user_id -> Varchar,
        orders -> Int8,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Varchar,
        fname -> Varchar,
        lname -> Varchar,
        gender -> Varchar,
        email -> Varchar,
        phone_number -> Varchar,
        address -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Varchar,
        user_name -> Varchar,
        pwd -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

table! {
    usr_orders (id) {
        id -> Varchar,
        user_id -> Varchar,
        order_date -> Timestamp,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

joinable!(cards -> products (product_id));
joinable!(cards -> users (user_id));
joinable!(category_aggregations -> categories (category_id));
joinable!(comments -> products (product_id));
joinable!(comments -> users (user_id));
joinable!(identifies -> users (user_id));
joinable!(order_items -> products (product_id));
joinable!(order_items -> usr_orders (order_id));
joinable!(payments -> usr_orders (order_id));
joinable!(products -> categories (category_id));
joinable!(user_aggregations -> users (user_id));
joinable!(usr_orders -> users (user_id));

allow_tables_to_appear_in_same_query!(
    cards,
    categories,
    category_aggregations,
    comments,
    identifies,
    order_items,
    payments,
    products,
    user_aggregations,
    users,
    usr_orders,
);
