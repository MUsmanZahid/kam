// TODO:
// [ ] - Backend
//  [X] - Create a SQLite database
//   [X] - Install `sqlx`
//   [X] - Set up an in-memory database
//   [X] - Create a `tasks` table
//   [X] - Insert dummy data into `tasks` table
//   [X] - Get dummy data from `tasks` table
//  [-] - Database Schema
//   [X] - Nested tasks
//    [X] - Parent reference in task
//    [X] - New many-to-many table for children references
//   ?[ ] - Deadlines
//   ?[ ] - Categories
// [ ] - Frontend
//  [ ] - Terminal UI
//   [ ] - Pick a library
//  [ ] - Web UI
//   [ ] - Learn HTMX

#[derive(sqlx::FromRow, Debug)]
struct Task {
    id: i64,
    parent: Option<i64>,
    title: String,
    completed: bool,
}

#[derive(sqlx::FromRow, Debug)]
struct Link {
    child: i64,
    parent: i64,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await?;
    schema(&pool).await?;
    demo_data(&pool).await?;
    query_data(&pool).await?;

    Ok(())
}

async fn demo_data(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    let mut query_builder =
        sqlx::QueryBuilder::<sqlx::Sqlite>::new("INSERT INTO tasks (parent, title, completed) ");
    let tasks = [
        (None, "A task", false),
        (Some(1), "Another task!", true),
        (None, "Woah, I'm busy!", false),
    ];
    query_builder.push_values(tasks, |mut b, (parent, title, completed)| {
        b.push_bind(parent);
        b.push_bind(title);
        b.push_bind(completed);
    });
    let query_result = query_builder.build().execute(pool).await?;
    eprintln!("Inserted {} rows!", query_result.rows_affected());
    eprintln!("Last inserted ID is {}!", query_result.last_insert_rowid());

    let mut query_builder =
        sqlx::QueryBuilder::<sqlx::Sqlite>::new("INSERT INTO children (child, parent) ");
    let children = [Link {
        child: 2,
        parent: 1,
    }];
    query_builder.push_values(children, |mut b, link| {
        b.push_bind(link.child);
        b.push_bind(link.parent);
    });
    let query_result = query_builder.build().execute(pool).await?;
    eprintln!("Inserted {} rows!", query_result.rows_affected());
    eprintln!("Last inserted ID is {}!", query_result.last_insert_rowid());

    Ok(())
}

async fn query_data(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    use futures::TryStreamExt;

    let mut query = sqlx::query_as::<_, Task>("SELECT * FROM tasks").fetch(pool);
    while let Some(task) = query.try_next().await? {
        println!("{:#?}", task);
    }

    let mut query = sqlx::query_as::<_, Link>("SELECT * FROM children").fetch(pool);
    while let Some(link) = query.try_next().await? {
        println!("{:#?}", link);
    }

    Ok(())
}

async fn schema(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    let tasks_table = sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY,
    parent INTEGER,
    title TEXT NOT NULL,
    completed BOOLEAN NOT NULL
)
"#,
    );
    let tasks_query = tasks_table.execute(pool);

    let children_table = sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS children(
    child INTEGER,
    parent INTEGER,
    FOREIGN KEY (child)
        REFERENCES tasks (id),
    FOREIGN KEY (parent)
        REFERENCES tasks (id)
)
"#,
    );
    let children_query = children_table.execute(pool);

    tasks_query.await?;
    eprintln!("Created table `tasks`!");

    children_query.await?;
    eprintln!("Created table `children`!");

    Ok(())
}
