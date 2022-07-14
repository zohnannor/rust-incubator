-- Add migration script here
CREATE TABLE article (
    id      BIGINT      NOT NULL PRIMARY KEY,
    title   VARCHAR     NOT NULL,
    body    TEXT        NOT NULL
);

CREATE TABLE label (
    id      BIGINT      NOT NULL PRIMARY KEY,
    name    VARCHAR     NOT NULL UNIQUE
);

CREATE TABLE article_label (
    article_id  BIGINT     NOT NULL,
    label_id    BIGINT     NOT NULL,
    FOREIGN KEY(article_id) REFERENCES article(id) ON DELETE CASCADE,
    FOREIGN KEY(label_id)   REFERENCES label(id)
);
