pub(crate) fn ui(f: &mut ratatui::Frame, app: &super::app::App) {
    let chunks = ratatui::prelude::Layout::default()
        .direction(ratatui::prelude::Direction::Vertical)
        .constraints([
            ratatui::prelude::Constraint::Length(3),
            ratatui::prelude::Constraint::Min(1),
            ratatui::prelude::Constraint::Length(3),
        ])
        .split(f.size());

    let title_block = ratatui::widgets::Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .style(ratatui::style::Style::default());

    let title = ratatui::widgets::Paragraph::new(ratatui::text::Text::styled(
        "Create New JSON",
        ratatui::style::Style::default().fg(ratatui::style::Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, chunks[0]);

    let items: Vec<_> = app
        .pairs
        .iter()
        .map(|(key, value)| {
            ratatui::widgets::ListItem::new(ratatui::text::Line::from(ratatui::text::Span::styled(
                format!("{: <25} : {}", key, value),
                ratatui::style::Style::default().fg(ratatui::style::Color::Yellow),
            )))
        })
        .collect();

    f.render_widget(ratatui::widgets::List::new(items), chunks[1]);

    let nav_text = vec![
        match app.screen {
            super::app::Screen::Main => ratatui::text::Span::styled(
                "Normal Mode",
                ratatui::style::Style::default().fg(ratatui::style::Color::Green),
            ),
            _ => ratatui::text::Span::styled(
                "Editing mode",
                ratatui::style::Style::default().fg(ratatui::style::Color::Yellow),
            ),
        },
        ratatui::text::Span::styled(
            " | ",
            ratatui::style::Style::default().fg(ratatui::style::Color::White),
        ),
        match app.screen {
            super::app::Screen::Key => ratatui::text::Span::styled(
                "Editing JSON Key",
                ratatui::style::Style::default().fg(ratatui::style::Color::Green),
            ),
            super::app::Screen::Main => ratatui::text::Span::styled(
                "Not Editing Anything",
                ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray),
            ),
            super::app::Screen::Value => ratatui::text::Span::styled(
                "Editing JSON Value",
                ratatui::style::Style::default().fg(ratatui::style::Color::LightGreen),
            ),
        },
    ];

    let mode = ratatui::widgets::Paragraph::new(ratatui::text::Line::from(nav_text))
        .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL));

    let key_hint = match app.screen {
        super::app::Screen::Main => ratatui::text::Span::styled(
            "(q) to quit / (e) to make new pair",
            ratatui::style::Style::default().fg(ratatui::style::Color::Red),
        ),
        _ => ratatui::text::Span::styled(
            "(ESC) to cancel / (TAB) to switch / (ENTER) to accept",
            ratatui::style::Style::default().fg(ratatui::style::Color::Red),
        ),
    };
    let hint = ratatui::widgets::Paragraph::new(ratatui::text::Line::from(key_hint))
        .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL));

    let footer_chunks = ratatui::prelude::Layout::default()
        .direction(ratatui::prelude::Direction::Horizontal)
        .constraints([
            ratatui::prelude::Constraint::Percentage(50),
            ratatui::prelude::Constraint::Percentage(50),
        ])
        .split(chunks[2]);

    f.render_widget(mode, footer_chunks[0]);
    f.render_widget(hint, footer_chunks[1]);

    match app.screen {
        s @ super::app::Screen::Key | s @ super::app::Screen::Value => {
            let popup = ratatui::widgets::Block::default()
                .title("Enter a new key-value pair")
                .borders(ratatui::widgets::Borders::NONE)
                .style(ratatui::style::Style::default().bg(ratatui::style::Color::DarkGray));

            let area = centered_rect(f.size(), 60, 25);
            f.render_widget(popup, area);

            let popup_chunks = ratatui::prelude::Layout::default()
                .direction(ratatui::prelude::Direction::Horizontal)
                .margin(1)
                .constraints([
                    ratatui::prelude::Constraint::Percentage(50),
                    ratatui::prelude::Constraint::Percentage(50),
                ])
                .split(area);

            let mut key_block = ratatui::widgets::Block::default()
                .title("Key")
                .borders(ratatui::widgets::Borders::ALL);
            let mut value_block = ratatui::widgets::Block::default()
                .title("Value")
                .borders(ratatui::widgets::Borders::ALL);

            let active_style = ratatui::style::Style::default().bg(ratatui::style::Color::LightYellow).fg(ratatui::style::Color::Black);
            if s == super::app::Screen::Key {
                key_block = key_block.style(active_style);
            } else {
                value_block = value_block.style(active_style);
            }

            let key_text = ratatui::widgets::Paragraph::new(app.key.clone()).block(key_block);
            f.render_widget(key_text, popup_chunks[0]);

            let value_text = ratatui::widgets::Paragraph::new(app.value.clone()).block(value_block);
            f.render_widget(value_text, popup_chunks[1]);
        }
        _ => {}
    }
}

fn centered_rect(rect: ratatui::prelude::Rect, x: u16, y: u16) -> ratatui::prelude::Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = ratatui::prelude::Layout::default()
        .direction(ratatui::prelude::Direction::Vertical)
        .constraints([
            ratatui::prelude::Constraint::Percentage((100 - y) / 2),
            ratatui::prelude::Constraint::Percentage(y),
            ratatui::prelude::Constraint::Percentage((100 - y) / 2),
        ])
        .split(rect);

    // Then cut the middle vertical piece into three width-wise pieces
    ratatui::prelude::Layout::default()
        .direction(ratatui::prelude::Direction::Horizontal)
        .constraints([
            ratatui::prelude::Constraint::Percentage((100 - x) / 2),
            ratatui::prelude::Constraint::Percentage(x),
            ratatui::prelude::Constraint::Percentage((100 - x) / 2),
        ])
        .split(popup_layout[1])[1]
}
