// TODO:
// [ ] - Backend
//  [X] - Create a SQLite database
//   [X] - Install `sqlx`
//   [X] - Set up an in-memory database
//   [X] - Create a `tasks` table
//   [X] - Insert dummy data into `tasks` table
//   [X] - Get dummy data from `tasks` table
//  [ ] - Database Schema
//   [ ] - Nested tasks
//   [ ] - Deadlines
//   [ ] - Categories
// [ ] - Frontend
//  [ ] - Terminal UI
//   [ ] - Pick a library
//  [ ] - Web UI
//   [ ] - Learn HTMX

use futures::TryStreamExt;
use sqlx::Execute;

#[derive(sqlx::FromRow, Debug)]
struct Task {
    id: i64,
    title: String,
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
    let mut query_builder = sqlx::QueryBuilder::<sqlx::Sqlite>::new("INSERT INTO tasks (title) ");
    let tasks = ["A task", "Another task!", "Woah, I'm busy!"];
    query_builder.push_values(tasks.iter(), |mut b, task| {
        b.push_bind(task);
    });
    let query = query_builder.build();
    println!("{}", query.sql());

    let query_result = query.execute(pool).await?;
    eprintln!("Inserted {} rows!", query_result.rows_affected());
    eprintln!("Last inserted ID is {}!", query_result.last_insert_rowid());

    Ok(())
}

async fn query_data(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    let mut query = sqlx::query_as::<_, Task>("SELECT * FROM tasks").fetch(pool);

    while let Some(task) = query.try_next().await? {
        println!("{} -> {}", task.id, task.title);
    }

    Ok(())
}

async fn schema(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    let _ =
        sqlx::query("CREATE TABLE IF NOT EXISTS tasks (id INTEGER PRIMARY KEY, title TEXT)")
            .execute(pool)
            .await?;
    eprintln!("Created table `tasks`!");

    Ok(())
}
