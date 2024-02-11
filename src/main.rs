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
//    [X] - Counter
//    [ ] - JSON Editor
//    [ ] - Async Counter
//  [ ] - Web UI
//   [ ] - Learn HTMX

mod backend;
mod tui;

#[derive(sqlx::FromRow, Debug)]
pub(crate) struct Task {
    pub(crate) id: i64,
    pub(crate) parent: Option<i64>,
    pub(crate) title: String,
    pub(crate) completed: bool,
}

#[derive(sqlx::FromRow, Debug)]
pub(crate) struct Link {
    pub(crate) child: i64,
    pub(crate) parent: i64,
}

struct App {
    counter: i64,
    should_quit: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let pool = backend::init().await?;

    tui::init_panic_handler();
    let terminal = tui::startup()?;
    run(terminal)?;
    Ok(tui::shutdown()?)
}

fn run(mut terminal: tui::Terminal) -> Result<(), std::io::Error> {
    let mut app = App {
        counter: 0,
        should_quit: false,
    };

    while !app.should_quit {
        terminal.draw(|frame| ui(&app, frame))?;
        update(&mut app)?;
    }

    Ok(())
}

fn ui(app: &App, f: &mut ratatui::Frame) {
    f.render_widget(
        ratatui::widgets::Paragraph::new(format!("Counter: {}", app.counter)),
        f.size(),
    );
}

fn update(app: &mut App) -> Result<(), std::io::Error> {
    if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
        if key.kind == crossterm::event::KeyEventKind::Press {
            match key.code {
                crossterm::event::KeyCode::Char('j') => app.counter -= 1,
                crossterm::event::KeyCode::Char('k') => app.counter += 1,
                crossterm::event::KeyCode::Char('q') => app.should_quit = true,
                _ => {},
            }
        }
    }

    Ok(())
}
