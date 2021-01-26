-- Based on https://til.hashrocket.com/posts/07139c566b-add-on-delete-cascade-to-foreign-key-constraint

-- Update products.manufacturer_id constraint
ALTER TABLE products
DROP CONSTRAINT products_manufacturer_id_fkey;

ALTER TABLE products
ADD CONSTRAINT products_manufacturer_id_fkey
FOREIGN KEY (manufacturer_id)
REFERENCES manufacturers (id)
ON UPDATE RESTRICT
ON DELETE RESTRICT;

-- Update category_products.product_id constraint
ALTER TABLE category_products
DROP CONSTRAINT category_products_product_id_fkey;

ALTER TABLE category_products
ADD CONSTRAINT category_products_product_id_fkey
FOREIGN KEY (product_id)
REFERENCES products (id)
ON UPDATE RESTRICT
ON DELETE CASCADE;

-- Update category_products.category_id constraint
ALTER TABLE category_products
DROP CONSTRAINT category_products_category_id_fkey;

ALTER TABLE category_products
ADD CONSTRAINT category_products_category_id_fkey
FOREIGN KEY (category_id)
REFERENCES categories (id)
ON UPDATE RESTRICT
ON DELETE RESTRICT;