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

struct Task {
    id: i64,
    name: Box<str>,
    complete: bool,
    parent: Option<i64>,
}

struct App {
    stack: Vec<i64>,
    tasks: Vec<Task>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = rusqlite::Connection::open_in_memory()?;

    conn.execute(
        "
        CREATE TABLE task (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            complete BOOLEAN NOT NULL DEFAULT 'FALSE' CHECK (complete IN (0, 1)),
            parent INTEGER REFERENCES task (id)
        );",
        (),
    )?;

    tui::init_panic_handler();
    let terminal = tui::startup(std::io::stderr())?;
    run(terminal)?;
    tui::shutdown()?;
    Ok(())
}

fn run<B>(mut terminal: ratatui::Terminal<B>) -> std::io::Result<()>
where
    B: ratatui::backend::Backend,
{
    let app = ();
    let mut exiting = false;

    while !exiting {
        terminal.draw(|f| ui(f, app))?;
        exiting = update()?;
    }

    terminal.show_cursor()?;
    Ok(())
}

fn update() -> Result<bool, std::io::Error> {
    if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
        if key.kind == crossterm::event::KeyEventKind::Press
            && key.code == crossterm::event::KeyCode::Char('q')
        {
            return Ok(true);
        }
    }

    Ok(false)
}

fn ui(f: &mut ratatui::Frame, _app: ()) {
    let chunks = ratatui::prelude::Layout::default()
        .direction(ratatui::prelude::Direction::Vertical)
        .constraints([
            ratatui::prelude::Constraint::Length(3),
            ratatui::prelude::Constraint::Min(2),
            ratatui::prelude::Constraint::Length(3),
        ])
        .split(f.size());

    f.render_widget(title("kam"), chunks[0]);
    f.render_widget(content("SAMPLE TEXT"), chunks[1]);
    f.render_widget(status("(q) to quit"), chunks[2]);
}

fn content<'a, T>(text: T) -> ratatui::widgets::Paragraph<'a>
where
    T: Into<std::borrow::Cow<'a, str>>,
{
    let block = ratatui::widgets::Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .style(ratatui::style::Style::default());

    ratatui::widgets::Paragraph::new(ratatui::text::Text::styled(
        text,
        ratatui::style::Style::default().fg(ratatui::style::Color::White),
    ))
    .block(block)
}

fn status<'a, T>(text: T) -> ratatui::widgets::Paragraph<'a>
where
    T: Into<std::borrow::Cow<'a, str>>,
{
    let block = ratatui::widgets::Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .style(ratatui::style::Style::default());

    ratatui::widgets::Paragraph::new(ratatui::text::Text::styled(
        text,
        ratatui::style::Style::default().fg(ratatui::style::Color::White),
    ))
    .block(block)
}

fn title<'a, T>(text: T) -> ratatui::widgets::Paragraph<'a>
where
    T: Into<std::borrow::Cow<'a, str>>,
{
    let block = ratatui::widgets::Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .style(ratatui::style::Style::default());

    ratatui::widgets::Paragraph::new(ratatui::text::Text::styled(
        text,
        ratatui::style::Style::default().fg(ratatui::style::Color::White),
    ))
    .block(block)
    .centered()
}

struct TreeTask<'a> {
    content: &'a str,
    id: i64,
    depth: u16,
    complete: bool,
}

struct Tree<'a> {
    block: Option<ratatui::widgets::Block<'a>>,
    highlight: ratatui::style::Style,
    indicator: &'a str,
    items: Vec<TreeTask<'a>>,
    style: ratatui::style::Style,
}

impl<'a> Tree<'a> {
    fn new(tasks: &'a [Task], stack: &'_ mut Vec<i64>) -> Self {
        let indent = 2;

        Self {
            block: None,
            highlight: ratatui::style::Style::default(),
            indicator: ">",
            items: Self::task_tree(indent, tasks, stack),
            style: ratatui::style::Style::default(),
        }
    }

    fn task_tree<'t>(indent: u16, tasks: &'t [Task], stack: &mut Vec<i64>) -> Vec<TreeTask<'t>> {
        let mut tree = Vec::new();

        for task in tasks {
            match task.parent {
                Some(id) => match stack.iter().position(|&x| x == id) {
                    Some(idx) => stack.truncate(idx + 1),
                    None => stack.push(id),
                },
                None => stack.clear(),
            }

            let depth = stack.len() as u16 * indent;
            let task = TreeTask {
                content: &task.name,
                id: task.id,
                depth,
                complete: task.complete,
            };
            tree.push(task);
        }

        tree
    }
}

struct TreeState {
    offset: usize,
    selected: usize,
}

impl<'t> ratatui::widgets::StatefulWidgetRef for Tree<'t> {
    type State = TreeState;

    fn render_ref(
        &self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        use ratatui::widgets::Widget;

        let blank = " ".repeat(self.indicator.width());

        buf.set_style(area, self.style);
        self.block.render(area, buf);

        let tree_area = self.block.as_ref().map_or(area, |block| block.inner(area));
        if tree_area.is_empty() || self.items.is_empty() {
            return;
        }

        let tree_height = tree_area.height as usize;
        let (first_visible_index, last_visible_index) = (0, 1);
        // self.get_item_bounds(state.selected, state.offset, tree_height);

        // NOTE: Change the state's offset to be the beginning of the visible change
        state.offset = first_visible_index;

        let mut current_height = 0;
        for (i, item) in self
            .items
            .iter()
            .enumerate()
            .skip(state.offset)
            .take(last_visible_index - first_visible_index)
        {
            let item_height = item.height() as u16;

            let (x, y) = {
                let height = current_height;
                current_height += item_height;
                (tree_area.left(), tree_area.top() + height)
            };

            let row_area = ratatui::layout::Rect {
                x,
                y,
                width: tree_area.width,
                height: item_height,
            };

            let item_style = self.style.patch(item.style);
            let is_selected = i == state.selected;
            let item_area = {
                let indicator_width = self.indicator.width() as u16;
                ratatui::layout::Rect {
                    x: row_area.x + indicator_width,
                    width: row_area.width - indicator_width,
                    ..row_area
                }
            };
            item.content.render(item_area, buf);

            for j in 0..item.content.height() {
                let symbol = if is_selected && j == 0 {
                    self.indicator
                } else {
                    &blank
                };

                buf.set_stringn(x, y + j as u16, symbol, tree_area.width as usize, item_style);
            }

            if is_selected {
                buf.set_style(row_area, self.highlight);
            }
        }
    }
}
