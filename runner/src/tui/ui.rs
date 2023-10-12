use rand::Rng;
use ratatui::style::Stylize;
use ratatui::text::Span;
use ratatui::text::Line;

use ratatui::prelude::Rect;
use ratatui::prelude::Constraint;
use ratatui::prelude::Direction;
use ratatui::prelude::Layout;

use ratatui::widgets::Paragraph;
use ratatui::widgets::Borders;
use ratatui::widgets::BorderType;
use ratatui::widgets::Block;

use ratatui::layout::Alignment;

use ratatui::style::Color;
use ratatui::style::Style;

use rand;
use ratatui::widgets::Wrap;

use crate::app::app::{App, DatabaseState, ClosedDatabaseAppState};
use crate::tui::tui::Frame;

pub fn render(app: &mut App, f: &mut Frame) {
    match app.get_database_state() {
        DatabaseState::Closed(_) => {
            render_default_screen(f, app);
        },
        DatabaseState::Opened(_) => {
            render_main_screen(f, app);
        },
    }
}

fn render_default_screen_body(f: &mut Frame, layout : Rect, color: Color) {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    f.render_widget(
        Paragraph::new(format!("\n\nWelcome to the DB paradise!\nPress `Ctrl-C` to stop running."))
            .block(
                Block::default()
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(color)),
            )
            .style(Style::default().fg(Color::Rgb(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255))).bold())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center),
            layout,
    )
}

fn render_screen_hood(f: &mut Frame, layout: Rect, color: Color, text: String) {
    f.render_widget(
        Paragraph::new(text)
            .block(
                Block::default()
                    .title(
                        Line::from(vec![
                            Span::styled(" Enter Input Mode ", Style::default().fg(color)),
                            Span::styled("(Press ", Style::default().fg(Color::DarkGray)),
                            Span::styled("/", Style::default().fg(Color::White)),
                            Span::styled(" to enter Hood, ", Style::default().fg(Color::DarkGray)),
                            Span::styled("Esc", Style::default().fg(Color::White)),
                            Span::styled(" to leave) ", Style::default().fg(Color::DarkGray)),
                        ])
                    )
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick),
            )
            .style(Style::default().fg(color))
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center),
            layout,
    )
}

fn render_active_menu(f: &mut Frame, layout: Rect, color: Color, tables: Vec<String>, index: usize) {
    let mut lines: Vec<Line> = Vec::new();
    for i in 0..tables.len() {
        if i == index {
            lines.push(Line::from(Span::styled(&tables[i], Style::default().fg(Color::Cyan).bold())))
        } else {
            lines.push(Line::from(Span::styled(&tables[i], Style::default().fg(Color::DarkGray))))
        }
    }

    if lines.len() == 0 {
        lines.push(Line::from(Span::styled("Whoops, no tables in this database :(", Style::default().fg(Color::Red).bold().italic())))
    }
    
    f.render_widget(
        Paragraph::new(lines)
            .block(
                Block::default()
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(color)),
            )
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left),
            layout,
    )
}

fn render_default_screen(f: &mut Frame, app: &mut App) {
    if let DatabaseState::Closed(state) = app.get_database_state() {
        let layout = 
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(80),
                Constraint::Percentage(20)
            ].as_ref())
            .split(f.size());
        let err_layout = 
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(80)
            ].as_ref())
            .split(f.size());
    
        match state {
            ClosedDatabaseAppState::ActiveHood(e) => {
                if e == "" {
                    render_default_screen_body(f, layout[0], Color::White);
                    render_screen_hood(f, layout[1], Color::Cyan, app.get_buffer());
                } else {
                    render_default_screen_body(f, err_layout[0], Color::White);
                    render_screen_hood(f, err_layout[1], Color::Red, e);
                }
            },
            ClosedDatabaseAppState::None => {
                render_default_screen_body(f, layout[0], Color::White);
                render_screen_hood(f, layout[1], Color::White, "".to_owned());
            },
        }  
    }
}

fn render_main_screen(f: &mut Frame, app: &mut App) {
    if let DatabaseState::Opened(state) = app.get_database_state() {
        let layout = 
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(80)
            ].as_ref())
            .split(f.size());
        
        let inner_layout = 
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(80)
            ].as_ref())
            .split(layout[1]);
        let err_inner_layout = 
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(80),
                Constraint::Percentage(20)
            ].as_ref())
            .split(layout[1]);

        match state {
            crate::app::app::OpenedDatabaseAppState::ActiveHood(e) => {
                render_active_menu(f, layout[0], Color::White, app.get_table_list(), app.get_selected_table_index());
                if e == "" {
                    render_screen_hood(f, inner_layout[0], Color::Cyan, app.get_buffer());
                    render_default_screen_body(f, inner_layout[1], Color::White);
                } else {
                    render_screen_hood(f, err_inner_layout[0], Color::Red, e);
                    render_default_screen_body(f, err_inner_layout[1], Color::White);
                }
            },
            crate::app::app::OpenedDatabaseAppState::ActiveMenu => {
                render_active_menu(f, layout[0], Color::Cyan, app.get_table_list(), app.get_selected_table_index());
                render_screen_hood(f, inner_layout[0], Color::White, "".to_owned());
                render_default_screen_body(f, inner_layout[1], Color::White);
            },
            crate::app::app::OpenedDatabaseAppState::ActiveTable => {
                render_active_menu(f, layout[0], Color::White, app.get_table_list(), app.get_selected_table_index());
                render_screen_hood(f, inner_layout[0], Color::White, "".to_owned());
                render_default_screen_body(f, inner_layout[1], Color::Cyan);
            },
            crate::app::app::OpenedDatabaseAppState::None => {
                render_active_menu(f, layout[0], Color::White, app.get_table_list(), app.get_selected_table_index());
                render_screen_hood(f, inner_layout[0], Color::White, "".to_owned());
                render_default_screen_body(f, inner_layout[1], Color::White);
            },
        } 
    }
}