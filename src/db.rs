use rusqlite::{params, Connection, Result as DBResult};

use crate::Todo;

pub fn add_todo(conn: &Connection, content: String) -> DBResult<()> {
    conn.execute("
        INSERT INTO todo (content, done) VALUES (?1, ?2)
    ", params![content, 0])?;
    Ok(())
}

pub fn remove_todo(conn: &Connection, id: u32) -> DBResult<()> {
    conn.execute("
        DELETE FROM todo WHERE id=?1 
    ", params![id])?;
    Ok(())
}

pub fn get_todos(conn: &Connection) -> DBResult<Vec<Todo>> {
    let mut stmt = conn.prepare("SELECT * FROM todo")?;
    let r = stmt.query_map([], |row| {
        let raw_done: u32 = row.get(2)?;
        Ok(
            Todo {
                id: row.get(0)?,
                content: row.get(1)?,
                done: if raw_done == 0 { false } else { true } 
            }
        )
    })?;

    Ok(r.map(|it| it.unwrap()).collect())
}

pub fn init_db() -> DBResult<Connection> {
    let conn = Connection::open("todo_db.db")?;

    conn.execute("
        CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL,
            done INTEGER NOT NULL CHECK (done IN (0, 1))
        );
        ",
    [])?;

    Ok(conn)
}

pub fn done_todo(conn: &Connection, id: u32) -> DBResult<()> {
    conn.execute("UPDATE todo SET done=true WHERE id=?1", params![id])?;
    Ok(())
}