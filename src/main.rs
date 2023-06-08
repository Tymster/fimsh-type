#![feature(if_let_guard)]
use std::io::{stdout, Write};
use termion::clear;
use termion::cursor;
use termion::cursor::Goto;
use termion::event::Key;
use termion::{async_stdin, input::TermRead};
use termion::{raw::IntoRawMode, screen::IntoAlternateScreen};
mod app;
use app::AppMode::*;
use app::CharStatus;
use app::{App, AppMode};
fn main() -> Result<(), std::io::Error> {
    let mut stdout = stdout()
        .into_raw_mode()
        .expect("Failed to enter raw mode")
        .into_alternate_screen()
        .expect("Failed to enter alternate mode");
    let mut stdin = async_stdin().keys();
    write!(stdout, "{}", clear::All)?;

    let mut app = App::new();
    loop {

        if let Some(Ok(key)) = stdin.next() {
            match key {
                Key::Ctrl('c') => break,
                Key::Char('\n') if let Results = app.mode => app.menu(), //set mode to menu
                Key::Char('t') if let Menu = app.mode => app.typing(), //set mode to typing
                Key::Right if let Menu = app.mode => {
                    break; //doing something later
                }
                Key::Char(c) if let Typing = app.mode => {
                    if app.index == 0 {
                        app.start =std::time::Instant::now(); //start the timer when types first char
                    }
                    
                    let char = &app.current_phrase[app.index];

                    if char.0 == c {
                        app.current_phrase[app.index] = (char.0 , CharStatus::Correct);
                    }else{
                        app.current_phrase[app.index] = (char.0 , CharStatus::Wrong);
                    }
                    app.index = (app.index + 1).clamp(0 , app.current_phrase.len());

                    if app.index >= app.current_phrase.len(){
                        app.results();
                    }
                    
                },
                Key::Backspace if let Typing = app.mode => {

                    let len = app.current_phrase.len();

                    let char = &app.current_phrase[(app.index - 1).clamp(0 , len)];
                   if let CharStatus::Wrong =  char.1  {
                        app.current_phrase[(app.index - 1).clamp(0 , len)] = (char.0 , CharStatus::NotYet);
                        app.index -= 1;
                            
                    };
                }
                e => {
                    println!("{:?}", e);
                }
            }
        }
        write!(
            stdout,
            "{}{}{}{}{}{}{}",
            Goto(1,1),
            clear::All,
            cursor::Hide,
            app,
            cursor::Show,
            Goto(app.index as u16 + 1, 1),
            cursor::SteadyBar
        )?;
        stdout.flush()?;
        std::thread::sleep(std::time::Duration::from_millis(17));
    }

    Ok(())
}
