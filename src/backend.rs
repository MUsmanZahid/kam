pub(crate) async fn init() -> Result<sqlx::Pool<impl sqlx::Database>, sqlx::Error> {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await?;
    db::schema_setup(&pool).await?;
    Ok(pool)
}

pub(crate) mod db {
    pub(crate) async fn schema_setup(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
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
        children_query.await?;
        Ok(())
    }
}
