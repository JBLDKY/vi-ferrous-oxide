#![allow(unused_imports)]
#![feature(iter_collect_into)]
mod rope;
use crossterm::cursor::MoveToPreviousLine;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use crossterm::style::{self, style, Attribute, Color, Print, PrintStyledContent, Stylize};
use crossterm::{
    cursor::{
        DisableBlinking, MoveTo, MoveToColumn, MoveToNextLine, RestorePosition, SavePosition,
    },
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

    for char in text.to_string().unwrap().chars() {
        if &char == &'\n' {
            let _ = execute!(stdout, MoveToNextLine(1));
            continue;
        };

        execute!(stdout, Print(&char));
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
                        refresh(stdout, &mut text);
                    }

                    KeyCode::Esc => {
                        break;
                    }

                    KeyCode::Backspace => {
                        if text.len() == 0 {
                            continue;
                        }

                        text.delete(text.len() - 1, text.len());
                        refresh(stdout, &mut text);
                    }

                    KeyCode::F(1) => {
                        dbg!(&text.to_string().unwrap());
                    }

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
