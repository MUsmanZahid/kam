/// A task manager
#[derive(clap::Parser, Debug)]
struct Args {
    /// Display all tasks
    #[arg(short, long)]
    all: bool,
    /// Complete a task
    #[arg(short, long, id = "ID")]
    complete: Option<u64>,
    /// ID of the task to query
    id: Option<u64>,
    /// Create a new task
    #[arg(short, long, name = "TITLE")]
    new: Option<String>,
    /// Specify the parent of a new task
    #[arg(short, long, name = "PARENT")]
    parent: Option<u64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = <Args as clap::Parser>::parse();

    let conn = rusqlite::Connection::open("kam.db")?;
    schema(&conn)?;

    if let Some(title) = args.new {
        task::new(&conn, title, args.parent)?;
    } else if let Some(id) = args.complete {
        task::complete(&conn, id)?;
    } else {
        task::list(&conn, args.id, args.all)?;
    }

    Ok(())
}

fn schema(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS task (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            complete BOOLEAN NOT NULL DEFAULT 0 CHECK (complete IN (0, 1)),
            parent INTEGER REFERENCES task (id)
        );",
        (),
    )?;

    Ok(())
}

mod task {
    pub fn complete(conn: &rusqlite::Connection, id: u64) -> rusqlite::Result<()> {
        conn.query_row(
            "UPDATE task SET complete = 1 WHERE id = ? RETURNING name;",
            [id],
            |row| {
                let title: String = row.get_unwrap(0);
                Ok(println!("\u{1B}[9m{id}. {title}\u{1B}[0m"))
            },
        )
    }

    pub fn list(conn: &rusqlite::Connection, id: Option<u64>, all: bool) -> rusqlite::Result<()> {
        let id = id.map(|i| i as i64);
        let sql = match (id, all) {
            (Some(_), true) => "SELECT id, name, complete FROM task WHERE parent = ?;",
            (Some(_), false) => {
                "SELECT id, name, complete FROM task WHERE complete = 0 AND parent = ?;"
            }
            (None, true) => "SELECT id, name, complete FROM task WHERE parent IS NULL;",
            (None, false) => {
                "SELECT id, name, complete FROM task WHERE complete = 0 AND parent IS NULL;"
            }
        };

        let mut statement = conn.prepare(sql)?;
        // let mut rows = statement.query([id])?;
        let mut rows = match id {
            Some(id) => statement.query([id])?,
            None => statement.query([])?,
        };

        while let Some(row) = rows.next()? {
            let id: i64 = row.get_unwrap(0);
            let name: String = row.get_unwrap(1);
            let complete: bool = row.get_unwrap(2);

            if complete {
                println!("\u{1B}[9m{id}. {name}\u{1B}[0m");
            } else {
                println!("{id}. {name}");
            }
        }

        Ok(())
    }

    pub fn new(
        conn: &rusqlite::Connection,
        title: String,
        parent: Option<u64>,
    ) -> rusqlite::Result<()> {
        let parent = parent.map(|p| p as i64);
        conn.query_row(
            "INSERT INTO task (name, parent) VALUES (?, ?) RETURNING id;",
            rusqlite::params![&title, parent],
            |row| {
                let id: i64 = row.get_unwrap(0);
                Ok(println!("{id}. {title}"))
            },
        )
    }
}
