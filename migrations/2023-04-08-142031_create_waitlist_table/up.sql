CREATE TABLE waitlists (
    id UUID PRIMARY KEY,
    ip_address VARCHAR(255),
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL
);
