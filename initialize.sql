CREATE TABLE IF NOT EXISTS session (
    session_id CHAR(16) COLLATE "C" NOT NULL,

    PRIMARY KEY (session_id)
);

CREATE TABLE IF NOT EXISTS poll (
    poll_id CHAR(8) COLLATE "C" NOT NULL,
    session_id CHAR(16) COLLATE "C" NOT NULL,
    creation_time TIMESTAMPTZ NOT NULL,
    title VARCHAR(128) NOT NULL,
    closed BOOLEAN NOT NULL DEFAULT FALSE,

    PRIMARY KEY (poll_id),

    FOREIGN KEY (session_id)
        REFERENCES session (session_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS poll_session_id_poll_id_idx
    ON poll (session_id, poll_id);

CREATE TABLE IF NOT EXISTS poll_categorical (
    poll_id CHAR(8) COLLATE "C" NOT NULL,
    mutex BOOL NOT NULL,

    PRIMARY KEY (poll_id),

    FOREIGN KEY (poll_id)
        REFERENCES poll (poll_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS poll_categorical_option (
    poll_id CHAR(8) COLLATE "C" NOT NULL,
    sequence INTEGER NOT NULL,
    name VARCHAR(64) NOT NULL,

    PRIMARY KEY (poll_id, sequence),

    FOREIGN KEY (poll_id)
        REFERENCES poll_categorical (poll_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS poll_categorical_response (
    poll_id CHAR(8) COLLATE "C" NOT NULL,
    session_id CHAR(16) COLLATE "C" NOT NULL,
    value INTEGER NOT NULL,

    PRIMARY KEY (session_id, poll_id),

    FOREIGN KEY (poll_id)
        REFERENCES poll_categorical (poll_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE,

    FOREIGN KEY (session_id)
        REFERENCES session (session_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS poll_numerical (
    poll_id CHAR(8) COLLATE "C" NOT NULL,
    minimum DOUBLE PRECISION NOT NULL,
    maximum DOUBLE PRECISION NOT NULL,
    only_integers BOOLEAN NOT NULL,

    PRIMARY KEY (poll_id),

    FOREIGN KEY (poll_id)
        REFERENCES poll (poll_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS poll_numerical_response (
    poll_id CHAR(8) COLLATE "C" NOT NULL,
    session_id CHAR(16) COLLATE "C" NOT NULL,
    value DOUBLE PRECISION NOT NULL,

    PRIMARY KEY (session_id, poll_id),

    FOREIGN KEY (poll_id)
        REFERENCES poll_numerical (poll_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE,

    FOREIGN KEY (session_id)
        REFERENCES session (session_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);

CREATE OR REPLACE FUNCTION purge_polls() RETURNS VOID AS $$
    DELETE FROM poll
    WHERE creation_time <= NOW() - INTERVAL '1 day'
$$ LANGUAGE SQL;

CREATE OR REPLACE FUNCTION purge_sessions() RETURNS VOID AS $$
    DELETE FROM session
    WHERE NOT EXISTS(
        SELECT 1
        FROM poll
        WHERE poll.session_id = session.session_id
    )
    AND NOT EXISTS(
        SELECT 1
        FROM poll_categorical_response
        WHERE poll_categorical_response.session_id = session.session_id
    )
    AND NOT EXISTS(
        SELECT 1
        FROM poll_numerical_response
        WHERE poll_numerical_response.session_id = session.session_id
    )
$$ LANGUAGE SQL;
