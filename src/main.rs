#![allow(unused_imports)]
mod rope;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use crossterm::{
    cursor::{DisableBlinking, MoveTo, MoveToNextLine},
    execute,
    terminal::{enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use anyhow::anyhow;
use std::time::Duration;

use std::io::{stdout, Write};

fn launch() -> Result<(), anyhow::Error> {
    enable_raw_mode()?;
    let mut text: Vec<char> = vec![];

    let mut stdout = stdout();

    loop {
        if poll(Duration::from_millis(100))? {
            if let Ok(Event::Key(KeyEvent { code, .. })) = read() {
                match code {
                    KeyCode::Char(c) => {
                        text.push(c);

                        // Clear the screen before re-rendering
                        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
                        for char in &text {
                            print!("{}", char);
                        }

                        stdout.flush()?;
                    }

                    KeyCode::Enter => {
                        // This is fucked up
                        text.push('\n');
                        execute!(stdout, MoveToNextLine(0))?;
                    }

                    KeyCode::Backspace => {
                        text.pop();

                        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
                        for char in &text {
                            print!("{}", char);
                        }

                        stdout.flush()?;
                    }
                    KeyCode::Esc => break,
                    _ => (),
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    execute!(
        stdout(),
        DisableBlinking,
        MoveTo(0, 0),
        Clear(ClearType::All),
        EnterAlternateScreen
    )?;

    let _ = launch();

    execute!(stdout(), LeaveAlternateScreen)
}
