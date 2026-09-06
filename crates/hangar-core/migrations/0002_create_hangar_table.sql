CREATE TABLE hangar (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    admin_password_hash TEXT NOT NULL,
    admin_last_login_at TEXT
);
