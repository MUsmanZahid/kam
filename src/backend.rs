pub mod db {
    fn insert<'a, DB, F, T>(
        table: &'a str,
        fields: &'a str,
        tuples: T,
        mapper: F,
    ) -> sqlx::QueryBuilder<'a, DB>
    where
        DB: sqlx::Database,
        F: FnMut(sqlx::query_builder::Separated<'_, 'a, DB, &'static str>, T::Item),
        T: IntoIterator,
    {
        let query = format!("INSERT INTO {} ({}) ", table, fields);
        let mut builder = sqlx::QueryBuilder::<DB>::new(query);
        builder.push_values(tuples, mapper);

        builder
    }

    pub async fn demo_data(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
        let tasks = [
            (None, "A task", false),
            (Some(1), "Another task!", true),
            (None, "Woah, I'm busy!", false),
        ];
        let mut query_builder = insert("tasks", "parent, title, completed", tasks, |mut b, task| {
            let (parent, title, completed) = task;

            b.push_bind(parent);
            b.push_bind(title);
            b.push_bind(completed);
        });
        let q1 = query_builder.build().execute(pool);

        let children = [crate::Link {
            child: 2,
            parent: 1,
        }];
        let mut query_builder = insert("children", "child, parent", children, |mut b, link| {
            b.push_bind(link.child);
            b.push_bind(link.parent);
        });
        let q2 = query_builder.build().execute(pool);

        q1.await?;
        q2.await?;

        Ok(())
    }

    pub async fn schema_setup(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
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
