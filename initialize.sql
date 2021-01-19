CREATE TABLE IF NOT EXISTS session (
    session_id CHAR(16) COLLATE "C" NOT NULL,

    PRIMARY KEY (session_id)
);

CREATE TABLE IF NOT EXISTS poll_numerical (
    poll_id CHAR(8) COLLATE "C" NOT NULL,
    session_id CHAR(16) COLLATE "C" NOT NULL,
    creation_time TIMESTAMPTZ NOT NULL,
    title VARCHAR(128) NOT NULL,
    minimum DOUBLE PRECISION NOT NULL,
    maximum DOUBLE PRECISION NOT NULL,
    only_integers BOOLEAN NOT NULL,

    PRIMARY KEY (poll_id),

    FOREIGN KEY (session_id)
        REFERENCES session (session_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS poll_numerical_session_id_poll_id_idx
    ON poll_numerical (session_id, poll_id);

CREATE TABLE IF NOT EXISTS poll_numerical_response (
    poll_id CHAR(8) COLLATE "C" NOT NULL,
    session_id CHAR(16) COLLATE "C" NOT NULL,
    value DOUBLE PRECISION NOT NULL,

    FOREIGN KEY (poll_id)
        REFERENCES poll_numerical (poll_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE,

    FOREIGN KEY (session_id)
        REFERENCES session (session_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS poll_numerical_response_session_id_poll_id_idx
    ON poll_numerical_response (session_id, poll_id);

CREATE TABLE IF NOT EXISTS poll_categorical (
    poll_id CHAR(8) COLLATE "C" NOT NULL,
    session_id CHAR(16) COLLATE "C" NOT NULL,
    creation_time TIMESTAMPTZ NOT NULL,
    title VARCHAR(128) NOT NULL,
    mutex BOOL NOT NULL,

    PRIMARY KEY (poll_id),

    FOREIGN KEY (session_id)
        REFERENCES session (session_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS poll_categorical_session_id_poll_id_idx
    ON poll_categorical (session_id, poll_id);

CREATE TABLE IF NOT EXISTS poll_categorical_option (
    poll_id CHAR(8) COLLATE "C" NOT NULL,
    sequence INTEGER NOT NULL,
    name VARCHAR(64) NOT NULL,

    FOREIGN KEY (poll_id)
        REFERENCES poll_categorical (poll_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS poll_categorical_option_poll_id_sequence_idx
    ON poll_categorical_option (poll_id, sequence);

CREATE TABLE IF NOT EXISTS poll_categorical_response (
    poll_id CHAR(8) COLLATE "C" NOT NULL,
    session_id CHAR(16) COLLATE "C" NOT NULL,
    value INTEGER NOT NULL,

    FOREIGN KEY (poll_id)
       REFERENCES poll_categorical (poll_id)
       ON UPDATE NO ACTION
       ON DELETE CASCADE,

    FOREIGN KEY (session_id)
       REFERENCES session (session_id)
       ON UPDATE NO ACTION
       ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS poll_categorical_response_session_id_poll_id_idx
    ON poll_categorical_response (session_id, poll_id);

CREATE OR REPLACE FUNCTION purge_sessions() RETURNS VOID AS $$
    DELETE FROM session
    WHERE NOT EXISTS(
        SELECT 1
        FROM poll_numerical
        WHERE poll_numerical.session_id = session.session_id
    )
    AND NOT EXISTS(
        SELECT 1
        FROM poll_numerical_response
        WHERE poll_numerical_response.session_id = session.session_id
    )
    AND NOT EXISTS(
        SELECT 1
        FROM poll_categorical
        WHERE poll_categorical.session_id = session.session_id
    )
    AND NOT EXISTS(
        SELECT 1
        FROM poll_categorical_response
        WHERE poll_categorical_response.session_id = session.session_id
    )
$$ LANGUAGE SQL;
