CREATE TABLE manufacturers (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name text NOT NULL,
    website_url text NOT NULL,
    logo_url text NOT NULL
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
    manufacturer_id uuid REFERENCES manufacturers (id),
    status text NOT NULL,
    image_url text NOT NULL
);
SELECT diesel_manage_updated_at('products');

CREATE TABLE categories (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name text NOT NULL,
    description text NOT NULL,
    image_url text NOT NULL,
    sort_order smallint NOT NULL DEFAULT 0
);

CREATE TABLE category_products (
    product_id uuid REFERENCES products (id),
    category_id uuid REFERENCES categories (id),
    PRIMARY KEY (product_id, category_id)
);