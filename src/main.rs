#![feature(if_let_guard)]
use std::io::{stdout, Write};
use termion::clear;
use termion::cursor;
use termion::cursor::Goto;
use termion::event::Key;
use termion::{async_stdin, input::TermRead};
use termion::{raw::IntoRawMode, screen::IntoAlternateScreen};
mod app;
mod config;
use app::AppMode::*;
use app::CharStatus;
use app::App ;
fn main() -> Result<(), std::io::Error> {
    let mut stdout = stdout()
        .into_raw_mode()
        .expect("Failed to enter raw mode")
        .into_alternate_screen()
        .expect("Failed to enter alternate mode");
    let mut stdin = async_stdin().keys();

    let mut app = App::new();
    loop {

        if let Some(Ok(key)) = stdin.next() {
            match key {
                Key::Ctrl('c') => {
                    write!(stdout , "{}" , cursor::Restore)?;
                    break
                },
                Key::Char('\n') if let Results = app.mode => app.menu(), //set mode to menu
                Key::Char('t') if let Menu = app.mode => app.typing(), //set mode to typing
                Key::Right if let Menu = app.mode => {
                    break; //doing something later
                }
                Key::Esc if let Typing | Results = app.mode => app.menu(),
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
                    if app.index > 0{
                        let len = app.current_phrase.len();
                        let index=  (app.index - 1).clamp(0 , len);
                        let char = &app.current_phrase[index];

                        if let CharStatus::Wrong = char.1  {
                            app.current_phrase[index] = (char.0 , CharStatus::NotYet);
                            app.index = index;           
                        };
                    }
                }
                e => {
                    println!("{:?}", e);
                }
            }
        }
        write!(
            stdout,
            "{}{}{}{}{}",
            Goto(1,1),
            clear::All,
            cursor::Hide,
            app,
            Goto(app.index as u16 + 1, 1),
        )?;
        stdout.flush()?;
        std::thread::sleep(std::time::Duration::from_millis(19));
    }
    write!(stdout, "{}{}" , clear::All , cursor::Restore)?;
    Ok(())
}
