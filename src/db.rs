use rusqlite::{params, Connection, Result as DBResult};

use crate::Todo;

pub fn add_todo(conn: &Connection, content: String) -> DBResult<()> {
    conn.execute("
        INSERT INTO todo (content) VALUES (?1)
    ", params![content])?;
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
        Ok(
            Todo {
                id: row.get(0)?,
                content: row.get(1)?,
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
            content TEXT NOT NULL
        )
        ",
    [])?;

    Ok(conn)
}