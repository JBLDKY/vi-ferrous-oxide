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
        LeaveAlternateScreen, SetTitle,
    },
};
use rope::Rope;
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use std::time::Duration;

use std::io::{stdout, Read, Stdout, Write};

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

fn launch(
    stdout: &mut Stdout,
    rope: Option<Rope>,
    mut file_name: Option<&Path>,
) -> Result<(), anyhow::Error> {
    enable_raw_mode()?;

    if file_name.is_none() {
        file_name = Some(Path::new("new_file.txt"));
    }

    let mut text: Rope = Rope::new();

    if let Some(rope) = rope {
        text = rope;
    }

    refresh(stdout, &mut text);

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
                        let mut file = File::create(&file_name.unwrap())?;
                        let _ = file.write(text.to_string().unwrap().as_bytes());
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

    let args: Vec<String> = env::args().collect();

    let mut file_name = None;
    if let Some(p) = args.last() {
        let path = Path::new(p);
        if path.exists() {
        } else if !path.exists() {
            File::create(path)?;
        }
        file_name = Some(path)
    };

    execute!(
        stdout,
        DisableBlinking,
        SetTitle(args.last().unwrap()),
        MoveTo(0, 0),
        Clear(ClearType::All),
        EnterAlternateScreen
    )?;

    if file_name.is_some() {
        let mut file = File::open(file_name.unwrap())?;
        let mut text = String::new();
        let _ = file.read_to_string(&mut text);
        let opened_file = Rope::from_str(&text);

        let _ = launch(&mut stdout, Some(opened_file), file_name.clone());
    } else {
        let _ = launch(&mut stdout, None, None);
    }

    execute!(stdout, LeaveAlternateScreen)
}
