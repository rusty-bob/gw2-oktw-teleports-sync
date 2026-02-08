use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use std::time::Duration;
use tp_sync::TeleportError;
use tp_sync::config::AppConfig;
use tp_sync::ui::{App, AppEvent, handle_key_event, render};

#[tokio::main]
async fn main() -> tp_sync::Result<()> {
    // Load or create config
    let config = match AppConfig::load()? {
        Some(config) => {
            // Validate existing config
            if let Err(e) = config.validate() {
                eprintln!("Config validation failed: {}", e);
                eprintln!("Please select teleport.json again.");
                setup_config()?
            } else {
                config
            }
        }
        None => {
            // First time setup
            setup_config()?
        }
    };

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app with config
    let mut app = App::new(&config).await?;

    // Run app
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn setup_config() -> tp_sync::Result<AppConfig> {
    println!("Welcome to Teleport Sync Manager!");
    println!("Please select your teleport.json file...");

    let path = rfd::FileDialog::new()
        .add_filter("JSON", &["json"])
        .set_title("Select teleport.json")
        .set_file_name("teleport.json")
        .pick_file()
        .ok_or_else(|| {
            TeleportError::IoError(io::Error::new(io::ErrorKind::NotFound, "No file selected"))
        })?;

    let config = AppConfig::new(path);
    config.validate()?;
    config.save()?;

    println!("Config saved!");

    Ok(config)
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> tp_sync::Result<()> {
    loop {
        terminal.draw(|f| render(f, app)).map_err(|e| {
            tp_sync::TeleportError::IoError(io::Error::new(
                io::ErrorKind::Other,
                format!("Terminal draw error: {}", e),
            ))
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Only process key press events, ignore key release
                if key.kind == KeyEventKind::Press {
                    if let Some(AppEvent::Quit) = handle_key_event(app, key).await? {
                        return Ok(());
                    }
                }
            }
        }
    }
}
