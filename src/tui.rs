pub type Terminal = ratatui::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>;

pub fn init_panic_handler() {
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = shutdown();
        original_hook(panic_info);
    }));
}

pub fn shutdown() -> std::io::Result<()> {
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()
}

pub fn startup() -> std::io::Result<Terminal> {
    use crossterm::ExecutableCommand;

    let mut stdout = std::io::stdout();
    stdout.execute(crossterm::terminal::EnterAlternateScreen)?;
    crossterm::terminal::enable_raw_mode()?;

    let mut terminal = ratatui::Terminal::new(ratatui::prelude::CrosstermBackend::new(stdout))?;
    terminal.clear()?;

    Ok(terminal)
}
