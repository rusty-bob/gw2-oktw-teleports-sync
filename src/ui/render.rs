use crate::ui::{App, AppMode, Pane};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    render_title(f, chunks[0]);
    render_main_content(f, chunks[1], app);
    render_status_bar(f, chunks[2], app);

    if !matches!(app.mode, AppMode::Normal) {
        render_confirmation_dialog(f, app);
    }
}

fn render_title(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("Teleport Sync Manager")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, area);
}

fn render_main_content(f: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    render_local_pane(f, chunks[0], app);
    render_remote_pane(f, chunks[1], app);
}

fn render_local_pane(f: &mut Frame, area: Rect, app: &mut App) {
    let is_active = matches!(app.active_pane, Pane::Local);

    let items: Vec<ListItem> = app
        .local_groups
        .iter()
        .map(|name| ListItem::new(Line::from(vec![Span::raw("✓ "), Span::raw(name)])))
        .collect();

    let border_style = if is_active {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };

    let list = List::new(items)
        .block(
            Block::default()
                .title("Local Groups (DEL to remove)")
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, area, &mut app.local_list_state);
}

fn render_remote_pane(f: &mut Frame, area: Rect, app: &mut App) {
    let is_active = matches!(app.active_pane, Pane::Remote);

    let items: Vec<ListItem> = app
        .remote_groups
        .iter()
        .map(|name| ListItem::new(Line::from(vec![Span::raw("○ "), Span::raw(name)])))
        .collect();

    let border_style = if is_active {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };

    let list = List::new(items)
        .block(
            Block::default()
                .title("Available Remote Groups (SPACE to install)")
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, area, &mut app.remote_list_state);
}

fn render_status_bar(f: &mut Frame, area: Rect, app: &App) {
    let status_text = if app.is_loading {
        "Loading..."
    } else if let Some(msg) = &app.status_message {
        msg.as_str()
    } else {
        "TAB: Switch pane | ↑↓: Navigate | SPACE: Install | DEL: Remove | q: Quit"
    };

    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(status, area);
}

fn render_confirmation_dialog(f: &mut Frame, app: &App) {
    let (title, message) = match &app.mode {
        AppMode::ConfirmDelete(name) => (
            "Confirm Deletion",
            format!("Delete '{}'?\n\nPress 'y' to confirm, 'n' to cancel", name),
        ),
        AppMode::ConfirmInstall(name) => (
            "Confirm Installation",
            format!("Install '{}'?\n\nPress 'y' to confirm, 'n' to cancel", name),
        ),
        AppMode::Normal => return,
    };

    let area = centered_rect(60, 20, f.area());

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    let paragraph = Paragraph::new(message)
        .block(block)
        .style(Style::default().fg(Color::White));

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
