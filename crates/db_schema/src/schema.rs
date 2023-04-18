// @generated automatically by Diesel CLI.

diesel::table! {
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

diesel::table! {
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

diesel::table! {
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

diesel::table! {
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

diesel::table! {
    orders (id) {
        id -> Varchar,
        user_id -> Varchar,
        order_date -> Timestamp,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
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

diesel::table! {
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

diesel::table! {
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

diesel::table! {
    users (id) {
        id -> Varchar,
        fname -> Varchar,
        lname -> Varchar,
        gender -> Varchar,
        email -> Varchar,
        address -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(category_aggregations -> categories (category_id));
diesel::joinable!(identifies -> users (user_id));
diesel::joinable!(order_items -> orders (order_id));
diesel::joinable!(order_items -> products (product_id));
diesel::joinable!(orders -> users (user_id));
diesel::joinable!(payments -> orders (order_id));
diesel::joinable!(products -> categories (category_id));
diesel::joinable!(user_aggregations -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    category_aggregations,
    identifies,
    order_items,
    orders,
    payments,
    products,
    user_aggregations,
    users,
);
