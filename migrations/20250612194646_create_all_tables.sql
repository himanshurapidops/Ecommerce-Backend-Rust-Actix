-- Add migration script here

--database

CREATE DATABASE IF NOT EXISTS ecommerce;

USE ecommerce;

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NULL DEFAULT current_timestamp(),
    full_name STRING NOT NULL DEFAULT '',
    mobile STRING NULL DEFAULT '',
    status STRING NOT NULL DEFAULT 'Inactive',
    "role" STRING NOT NULL DEFAULT 'CUSTOMER',
    CONSTRAINT users_pkey PRIMARY KEY (id),
    UNIQUE INDEX users_email_key (email),
    CONSTRAINT check_status CHECK (status IN ('Active', 'Inactive', 'Suspended')),
    CONSTRAINT check_role CHECK ("role" IN ('ADMIN', 'CUSTOMER'))
);

-- Addresses table
CREATE TABLE IF NOT EXISTS addresses (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    address_line1 STRING NOT NULL,
    city STRING NOT NULL,
    state STRING NOT NULL,
    pincode STRING NOT NULL,
    country STRING NOT NULL DEFAULT 'India',
    mobile STRING NOT NULL,
    selected BOOL NULL DEFAULT false,
    user_id UUID NOT NULL,
    created_at TIMESTAMPTZ NULL DEFAULT now(),
    CONSTRAINT addresses_pkey PRIMARY KEY (id),
    CONSTRAINT addresses_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    INDEX idx_addresses_id_user_id (id, user_id)
);

-- Products table
CREATE TABLE IF NOT EXISTS products (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    name STRING NOT NULL,
    description STRING NOT NULL,
    images STRING[] NOT NULL,
    category STRING NOT NULL,
    price FLOAT8 NOT NULL,
    is_available BOOL NULL DEFAULT true,
    count_in_stock INT8 NULL DEFAULT 0,
    created_at TIMESTAMPTZ NULL DEFAULT current_timestamp(),
    brand STRING NULL DEFAULT '',
    CONSTRAINT products_pkey PRIMARY KEY (id),
    INDEX idx_products_id (id)
);

-- Cart Products
CREATE TABLE IF NOT EXISTS cart_products (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    product_id UUID NOT NULL,
    quantity INT8 NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp(),
    CONSTRAINT cart_products_pkey PRIMARY KEY (id),
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_product FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
    UNIQUE INDEX cart_products_user_product_unique (user_id, product_id),
    INDEX idx_cart_products_user_id (user_id),
    INDEX idx_cart_products_product_id (product_id),
    CONSTRAINT check_quantity CHECK (quantity >= 1)
);

-- Orders table
CREATE TABLE IF NOT EXISTS orders (
    id UUID NOT NULL,
    user_id UUID NOT NULL,
    order_id STRING NOT NULL,
    payment_id STRING NOT NULL,
    payment_status STRING NOT NULL,
    delivery_address_id UUID NOT NULL,
    total_amount DECIMAL(10,2) NOT NULL,
    order_status STRING NOT NULL DEFAULT 'Processing',
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp(),
    CONSTRAINT orders_pkey PRIMARY KEY (id),
    CONSTRAINT orders_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT orders_delivery_address_id_fkey FOREIGN KEY (delivery_address_id) REFERENCES addresses(id) ON DELETE RESTRICT,
    UNIQUE INDEX orders_order_id_key (order_id),
    INDEX idx_orders_order_id_user_id (order_id, user_id),
    INDEX idx_orders_order_id (order_id),
    INDEX idx_orders_user_id_created_at (user_id, created_at DESC)
);

-- Order Items
CREATE TABLE IF NOT EXISTS order_items (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL,
    product_id UUID NOT NULL,
    quantity INT8 NOT NULL,
    price_at_order_time FLOAT8 NOT NULL,
    CONSTRAINT order_items_pkey PRIMARY KEY (id),
    CONSTRAINT order_items_order_id_fkey FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE CASCADE,
    CONSTRAINT order_items_product_id_fkey FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE RESTRICT,
    INDEX idx_order_items_order_id (order_id),
    INDEX idx_order_items_product_id (product_id)
);
