CREATE TABLE manufacturers (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name text NOT NULL,
    slug text UNIQUE NOT NULL DEFAULT '',
    website_url text NOT NULL
);

CREATE TABLE products (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    name text NOT NULL,
    slug text NOT NULL,
    description_short text NOT NULL,
    description_long text NOT NULL,
    price integer NOT NULL,
    manufacturer_id uuid REFERENCES manufacturers (id) ON UPDATE RESTRICT ON DELETE RESTRICT,
    status text NOT NULL,
    stock_count integer NOT NULL DEFAULT 0
);
SELECT diesel_manage_updated_at('products');

CREATE TABLE categories (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name text NOT NULL,
    slug text UNIQUE NOT NULL DEFAULT '',
    description text NOT NULL,
    sort_order smallint NOT NULL DEFAULT 0
);

CREATE TABLE category_products (
    product_id uuid REFERENCES products (id) ON UPDATE RESTRICT ON DELETE CASCADE,
    category_id uuid REFERENCES categories (id) ON UPDATE RESTRICT ON DELETE RESTRICT,
    PRIMARY KEY (product_id, category_id)
);