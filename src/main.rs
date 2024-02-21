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

mod tui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tui::init_panic_handler();
    let terminal = tui::startup(std::io::stderr())?;
    let result = run(terminal)?;
    tui::shutdown()?;

    if let Some(pairs) = result {
        let json = serde_json::to_string(&pairs)?;
        println!("{json}");
    }

    Ok(())
}

fn run<B>(mut terminal: ratatui::Terminal<B>) -> std::io::Result<Option<tui::app::Pairs>>
where
    B: ratatui::backend::Backend,
{
    let mut app = tui::app::App::new();
    let mut exiting = false;

    while !exiting {
        terminal.draw(|frame| tui::ui::ui(frame, &app))?;
        exiting = update(&mut app)?;
    }

    terminal.show_cursor()?;
    Ok(Some(app.pairs))
}

fn update(app: &mut tui::app::App) -> Result<bool, std::io::Error> {
    if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
        if key.kind == crossterm::event::KeyEventKind::Press {
            match app.screen {
                tui::app::Screen::Key => match key.code {
                    crossterm::event::KeyCode::Backspace => {
                        app.key.pop();
                    }
                    crossterm::event::KeyCode::Enter => app.screen = tui::app::Screen::Value,
                    crossterm::event::KeyCode::Esc => app.screen = tui::app::Screen::Main,
                    crossterm::event::KeyCode::Tab => app.screen = tui::app::Screen::Value,
                    crossterm::event::KeyCode::Char(c) => app.key.push(c),
                    _ => {}
                },
                tui::app::Screen::Main => match key.code {
                    crossterm::event::KeyCode::Char('e') => {
                        app.screen = tui::app::Screen::Key;
                    }
                    crossterm::event::KeyCode::Char('q') => {
                        return Ok(true);
                    }
                    _ => {}
                },
                tui::app::Screen::Value => match key.code {
                    crossterm::event::KeyCode::Backspace => {
                        app.value.pop();
                    }
                    crossterm::event::KeyCode::Enter => {
                        app.save_pair();
                        app.screen = tui::app::Screen::Main;
                    }
                    crossterm::event::KeyCode::Esc => app.screen = tui::app::Screen::Main,
                    crossterm::event::KeyCode::Tab => app.screen = tui::app::Screen::Key,
                    crossterm::event::KeyCode::Char(c) => app.value.push(c),
                    _ => {}
                },
            }
        }
    }

    Ok(false)
}
