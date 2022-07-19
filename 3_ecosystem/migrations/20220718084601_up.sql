-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "user" (
    id          UUID        DEFAULT uuid_generate_v4(),
    name        varchar(50) UNIQUE NOT NULL,
    password    TEXT        NOT NULL,

    PRIMARY KEY(id)
);

CREATE TABLE friend (
    user_1          UUID     NOT NULL,
    user_2          UUID     NOT NULL,

    PRIMARY KEY(user_1, user_2),
    UNIQUE (user_1, user_2),

    CONSTRAINT fk_user_1 FOREIGN KEY(user_1) REFERENCES "user"(id) ON DELETE CASCADE,
    CONSTRAINT fk_user_2 FOREIGN KEY(user_2) REFERENCES "user"(id) ON DELETE CASCADE
);

