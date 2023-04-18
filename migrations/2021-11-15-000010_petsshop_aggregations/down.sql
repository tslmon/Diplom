--
--  Drop PetsShop Aggregations functions and triggers
--

-- user_aggregations_orders
DROP TRIGGER user_aggregations_orders ON orders;
DROP FUNCTION IF EXISTS user_aggregations_orders();
-- category_aggregations_products
DROP TRIGGER category_aggregations_products ON products;
DROP FUNCTION IF EXISTS category_aggregations_products();
