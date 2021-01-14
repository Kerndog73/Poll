CREATE TABLE IF NOT EXISTS poll_numerical (
    poll_id CHAR(8) NOT NULL,
    creation_time TIMESTAMPTZ NOT NULL,
    title VARCHAR(128) NOT NULL,
    minimum DOUBLE PRECISION NOT NULL,
    maximum DOUBLE PRECISION NOT NULL,
    only_integers BOOLEAN NOT NULL,

    PRIMARY KEY (poll_id)
);

CREATE TABLE IF NOT EXISTS poll_numerical_response (
    poll_id CHAR(8) NOT NULL,
    value DOUBLE PRECISION NOT NULL,

    FOREIGN KEY (poll_id)
        REFERENCES poll_numerical (poll_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);
