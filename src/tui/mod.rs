pub(crate) mod app;
pub(crate) mod ui;

pub type Terminal<W> = ratatui::Terminal<ratatui::prelude::CrosstermBackend<W>>;

pub fn init_panic_handler() {
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = shutdown();
        original_hook(panic_info);
    }));
}

pub fn shutdown() -> std::io::Result<()> {
    crossterm::terminal::disable_raw_mode()?;

    crossterm::execute!(
        std::io::stderr(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture,
    )
}

pub fn startup<W>(mut output: W) -> std::io::Result<Terminal<W>>
where
    W: std::io::Write,
{
    crossterm::execute!(
        output,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;
    crossterm::terminal::enable_raw_mode()?;

    let mut terminal = ratatui::Terminal::new(ratatui::prelude::CrosstermBackend::new(output))?;
    terminal.clear()?;

    Ok(terminal)
}
