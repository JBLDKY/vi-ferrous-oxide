#![allow(unused_imports)]
mod rope;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use crossterm::{
    cursor::{DisableBlinking, MoveTo, MoveToColumn, MoveToNextLine, RestorePosition},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use rope::Rope;

use anyhow::anyhow;
use std::time::Duration;

use std::io::{stdout, Stdout, Write};

#[cfg(test)]
mod test_rope;

#[inline]
fn refresh(stdout: &mut Stdout, text: &mut Rope) {
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).ok();

    for char in text.to_string().iter() {
        print!("{}", char);
    }

    let _ = stdout.flush();
}

fn launch(stdout: &mut Stdout) -> Result<(), anyhow::Error> {
    enable_raw_mode()?;

    let mut text: Rope = Rope::new();

    loop {
        if poll(Duration::from_millis(100))? {
            if let Ok(Event::Key(KeyEvent { code, .. })) = read() {
                match code {
                    KeyCode::Char(c) => {
                        let mut b = [0; 1];
                        let char_str = c.encode_utf8(&mut b);
                        text.append(char_str);
                        refresh(stdout, &mut text);
                    }

                    KeyCode::Enter => {
                        text.append("\n");
                        execute!(stdout, MoveToNextLine(1))?;
                        refresh(stdout, &mut text);
                    }

                    KeyCode::Tab => {
                        dbg!(&text);
                    }

                    KeyCode::Backspace => {
                        text.delete(text.len(), text.len());
                        refresh(stdout, &mut text);
                    }

                    KeyCode::Esc => break,

                    _ => (),
                }
            }
        }
    }

    disable_raw_mode()?;
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
