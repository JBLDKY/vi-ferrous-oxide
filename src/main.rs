#![allow(unused_imports)]
mod rope;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use crossterm::{
    cursor::{DisableBlinking, MoveTo, MoveToColumn, MoveToNextLine, RestorePosition},
    execute,
    terminal::{enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use anyhow::anyhow;
use std::time::Duration;

use std::io::{stdout, Stdout, Write};

#[inline]
fn refresh(stdout: &mut Stdout, text: &Vec<char>) {
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).ok();

    for char in text {
        print!("{}", char);
    }

    let _ = stdout.flush();
}

fn launch(stdout: &mut Stdout) -> Result<(), anyhow::Error> {
    enable_raw_mode()?;
    let mut text: Vec<char> = vec![];

    loop {
        if poll(Duration::from_millis(100))? {
            if let Ok(Event::Key(KeyEvent { code, .. })) = read() {
                match code {
                    KeyCode::Char(c) => {
                        text.push(c);
                        refresh(stdout, &text);
                    }

                    KeyCode::Enter => {
                        text.push('\n');
                        execute!(stdout, MoveToNextLine(1))?;
                        refresh(stdout, &text);
                    }

                    KeyCode::Tab => {
                        dbg!(&text);
                    }

                    KeyCode::Backspace => {
                        text.pop();
                        refresh(stdout, &text);
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
    let mut stdout = stdout();
    execute!(
        stdout,
        DisableBlinking,
        MoveTo(0, 0),
        Clear(ClearType::All),
        EnterAlternateScreen
    )?;

    let _ = launch(&mut stdout);

    execute!(stdout, LeaveAlternateScreen)
}
