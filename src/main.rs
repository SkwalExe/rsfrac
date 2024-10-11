use ratatui::crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, execute};
use rsfrac::app::App;
use std::io::{self, stdout};

fn main() -> io::Result<()> {
    // Get a terminal handle on stdout
    let mut term = ratatui::init();
    // Clear the terminal
    term.clear()?;

    // Create an instance of our app
    let mut app = App::default();

    execute!(stdout(), EnableMouseCapture)?;

    // Run the app
    app.run(&mut term)?;

    execute!(stdout(), DisableMouseCapture)?;

    // Restore the terminal
    ratatui::restore();

    // Print the log message history
    app.print_logs(&term);

    Ok(())
}
