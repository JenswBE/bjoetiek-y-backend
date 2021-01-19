ALTER TABLE products
    DROP image_url,
    ADD stock_count integer NOT NULL DEFAULT 0;