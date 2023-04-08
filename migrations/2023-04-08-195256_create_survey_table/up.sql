CREATE TABLE surveys (
    id UUID PRIMARY KEY,
    question VARCHAR(255) NOT NULL,
    answer VARCHAR(255) NOT NULL,
    ip_address VARCHAR(255),
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL
);
CREATE INDEX answer_idx ON surveys (answer);
CREATE INDEX question_idx ON surveys (question);
