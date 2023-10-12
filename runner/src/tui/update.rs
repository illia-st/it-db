use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::app::{App, DatabaseState, ClosedDatabaseAppState, OpenedDatabaseAppState};

pub fn update(app: &mut App, key_event: KeyEvent) {
    if let KeyCode::Char(char) = key_event.code {
        app.add_char_to_buffer(char);
    }

    match key_event.code {
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        },
        KeyCode::Char('/') => {
            if let DatabaseState::Closed(ClosedDatabaseAppState::None) = app.get_database_state() {
                app.activete_closed_database_hood();
                app.clear_buffer();
            }
            if let DatabaseState::Opened(OpenedDatabaseAppState::None) = app.get_database_state() {
                app.activete_opened_database_hood();
                app.clear_buffer();
            }
        },
        | KeyCode::Char('s') | KeyCode::Char('S') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                if let DatabaseState::Opened(_) = app.get_database_state() {
                    app.close_database(true);
                }
            }
        }
        KeyCode::Esc => {
            if let DatabaseState::Closed(ClosedDatabaseAppState::ActiveHood(_)) = app.get_database_state() {
                app.deactivete_closed_database_hood();
                app.clear_buffer();
            }
            if let DatabaseState::Opened(OpenedDatabaseAppState::ActiveHood(_)) = app.get_database_state() {
                app.deactivete_opened_database_hood();
                app.clear_buffer();
            }
        }
        KeyCode::Enter => {
            if let DatabaseState::Closed(ClosedDatabaseAppState::ActiveHood(_)) = app.get_database_state() {
                app.open_database(app.get_buffer());
                app.clear_buffer();
            }
        },
        KeyCode::Left => {
            if let DatabaseState::Opened(_) = app.get_database_state() {
                app.activete_opened_database_active_menu();
            }
        },
        KeyCode::Right => {
            if let DatabaseState::Opened(_) = app.get_database_state() {
                app.activete_opened_database_hood();
            }
        },
        KeyCode::Up => {
            if let DatabaseState::Opened(_) = app.get_database_state() {
                app.activete_opened_database_hood();
            }
        },
        KeyCode::Down => {
            if let DatabaseState::Opened(_) = app.get_database_state() {
                app.activete_opened_database_active_table();
            }
        },
        _ => {},
    };
}