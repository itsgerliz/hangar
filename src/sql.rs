pub(crate) const CREATE_SCHEMA: &str = r#"
   	CREATE TABLE stashdeck (
  		id INTEGER PRIMARY KEY CHECK (id = 1),
    	version TEXT NOT NULL
   	);

   	CREATE TABLE users (
  		id INTEGER PRIMARY KEY AUTOINCREMENT,
    	email TEXT NOT NULL UNIQUE COLLATE NOCASE,
       	password TEXT NOT NULL,
       	name TEXT,
       	active BOOLEAN NOT NULL DEFAULT TRUE,
        created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
        last_login DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
   	);
"#;

pub(crate) const INSERT_STASHDESK_VERSION: &str = r#"
	INSERT INTO stashdeck (id, version) VALUES (1, $1)
"#;
