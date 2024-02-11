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
// [ ] - Frontend
//  [ ] - Terminal UI
//   [X] - Pick a library = Ratatui/Crossterm
//   [ ] - Finish tutorials
//    [X] - Hello World
//    [ ] - Counter
//    [ ] - JSON Editor
//    [ ] - Async Counter
//  [ ] - Web UI
//   [ ] - Learn HTMX

mod backend;
mod tui;

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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await?;
    backend::db::schema_setup(&pool).await?;
    backend::db::demo_data(&pool).await?;

    tui::init_panic_handler();
    let mut terminal = tui::startup()?;

    loop {
        // Draw the UI
        terminal.draw(|frame| {
            use ratatui::prelude::Stylize;

            let area = frame.size();
            frame.render_widget(
                ratatui::widgets::Paragraph::new("Hello, Ratatui! (press 'q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        })?;

        // Handle events
        if crossterm::event::poll(std::time::Duration::from_millis(16))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == crossterm::event::KeyEventKind::Press
                    && key.code == crossterm::event::KeyCode::Char('q')
                {
                    break;
                }
            }
        }
    }

    tui::shutdown()?;
    Ok(())
}

