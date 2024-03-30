/*use std::sync::Mutex;
use rusqlite::{ Connection, Result, Error };

pub struct Database {
  conn: Mutex<Connection>,
}

impl Database {

  pub fn new(database_dir: &str) -> Result<Self, Error> {
    let conn = Connection::open( format!("{}/{}", database_dir, "test.db") )?;
    conn.pragma( None, "journal_mode", "WAL", |f| {
      println!("{:?}", f);
      Ok(())
    }).unwrap();
    Ok(Database { conn: Mutex::new(conn) })
  }

  pub fn create_table(&self) -> Result<(), rusqlite::Error> {

    self.conn.lock().unwrap().execute_batch(
      "CREATE TABLE IF NOT EXISTS tag (
        id INTEGER PRIMARY KEY, icon BLOB,
        name TEXT NOT NULL, parent TEXT default(0),
        child TEXT default(0),
        update_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now','localtime')),
        register_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now','localtime'))
      );
      CREATE TABLE IF NOT EXISTS tag_structure (
        parent_id	INTEGER NOT NULL, child_id	INTEGER NOT NULL,
        FOREIGN KEY(parent_id) REFERENCES tag(id) ON DELETE CASCADE,
        FOREIGN KEY(child_id) REFERENCES tag(id) ON DELETE CASCADE,
        PRIMARY KEY(parent_id, child_id)
      );
      CREATE TABLE IF NOT EXISTS bookmark (
        id INTEGER PRIMARY KEY, type VARCHar(50) NOT NULL,
        content TEXT NOT NULL,
        update_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now','localtime')),
        register_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now','localtime'))
      );
      CREATE TABLE IF NOT EXISTS tag_bookmark (
        tag_id INTEGER NOT NULL, bookmark_id INTEGER NOT NULL,
        PRIMARY KEY (tag_id, bookmark_id),
        FOREIGN KEY (tag_id) REFERENCES folder(id) ON DELETE CASCADE,
        FOREIGN KEY (bookmark_id) REFERENCES bookmark(id) ON DELETE CASCADE
      );"
    )?;

    Ok(())
  }
}*/