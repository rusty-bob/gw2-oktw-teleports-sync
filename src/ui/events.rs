use crate::ui::{App, AppMode};
use crate::Result;
use crossterm::event::{KeyCode, KeyEvent};

pub enum AppEvent {
    Quit,
}

pub async fn handle_key_event(app: &mut App, key: KeyEvent) -> Result<Option<AppEvent>> {
    match app.mode {
        AppMode::Normal => handle_normal_mode(app, key).await,
        AppMode::ConfirmDelete(ref name) => handle_confirm_delete(app, key, name.clone()).await,
        AppMode::ConfirmInstall(ref name) => handle_confirm_install(app, key, name.clone()).await,
    }
}

async fn handle_normal_mode(app: &mut App, key: KeyEvent) -> Result<Option<AppEvent>> {
    match key.code {
        KeyCode::Char('q') => Ok(Some(AppEvent::Quit)),
        KeyCode::Tab => {
            app.switch_pane();
            Ok(None)
        }
        KeyCode::Up => {
            app.navigate_up();
            Ok(None)
        }
        KeyCode::Down => {
            app.navigate_down();
            Ok(None)
        }
        KeyCode::Delete => {
            app.handle_delete();
            Ok(None)
        }
        KeyCode::Char(' ') => {
            app.handle_space();
            Ok(None)
        }
        _ => Ok(None),
    }
}

async fn handle_confirm_delete(
    app: &mut App,
    key: KeyEvent,
    group_name: String,
) -> Result<Option<AppEvent>> {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            app.confirm_delete(group_name).await?;
            Ok(None)
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            app.cancel_confirmation();
            Ok(None)
        }
        _ => Ok(None),
    }
}

async fn handle_confirm_install(
    app: &mut App,
    key: KeyEvent,
    group_name: String,
) -> Result<Option<AppEvent>> {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            app.confirm_install(group_name).await?;
            Ok(None)
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            app.cancel_confirmation();
            Ok(None)
        }
        _ => Ok(None),
    }
}

