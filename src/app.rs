use rand::Rng;
use std::time::Duration;
use std::time::Instant;
use termion::color::{Fg, Green, Red, Reset};
use termion::cursor;
use termion::style::Faint;
const PHRASES: [&str; 2] = [
    "balls on a tree",
    "the quick brown fox jumped over the lazy dog",
];
pub struct App {
    pub mode: AppMode,
    pub current_phrase: Vec<(char, CharStatus)>,
    pub index: usize,
    pub start: Instant,
    pub end: Instant,
    pub history: Vec<History>,
}
pub struct History {
    accuracy: f32,
    wpm: f32,
}
#[derive(Debug, Clone, Copy)]
pub enum CharStatus {
    Correct,
    Wrong,
    NotYet,
}
pub enum AppMode {
    Typing,
    Menu,
    Results,
}
impl App {
    pub fn new() -> Self {
        Self {
            mode: AppMode::Menu,
            current_phrase: vec![],
            index: 0,
            start: Instant::now(),
            end: Instant::now(),
            history: vec![],
        }
    }
    pub fn typing(&mut self) {
        self.mode = AppMode::Typing;

        if self.current_phrase.len() <= 0 {
            self.current_phrase = PHRASES[rand::thread_rng().gen_range(0..PHRASES.len())]
                .chars()
                .map(|c| (c, CharStatus::NotYet))
                .collect();
        }
    }

    pub fn results(&mut self) {
        self.mode = AppMode::Results;
        self.end = Instant::now();
        let (wpm, accuracy) = wpm(self.current_phrase.clone(), self.end - self.start);
        self.history.push(History { accuracy, wpm })
    }
    pub fn menu(&mut self) {
        self.mode = AppMode::Menu;
        self.current_phrase = vec![];
        self.index = 0;
    }
}
impl std::fmt::Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match &self.mode {
            AppMode::Typing => {
                write!(f, "{}", cursor::Show)?;

                for char in &self.current_phrase {
                    match char.1 {
                        CharStatus::Correct => write!(f, "{}{}{}", Fg(Green), char.0, Fg(Reset))?,
                        CharStatus::Wrong => write!(f, "{}{}{}", Fg(Red), char.0, Fg(Reset))?,
                        CharStatus::NotYet => write!(f, "{}{}{}", Faint, char.0, Fg(Reset))?,
                    }
                }
                if self.index != 0 {
                    write!(
                        f,
                        "\r\n{}",
                        ((Instant::now() - self.start).as_secs_f64() * 100.0).round() / 100.0
                    )?;
                } else {
                    write!(f, "\r\n0s")?;
                }
            }
            AppMode::Menu => {
                write!(f, "Press T to start typing\r\n")?;
                for n in &self.history {
                    write!(f, "Wpm : {} Accuracy : {}%\r\n", n.wpm, n.accuracy)?;
                }
            }
            AppMode::Results => {
                write!(f, "{}", cursor::Hide)?;
                let (wpm, accuracy) = wpm(self.current_phrase.clone(), self.end - self.start);
                write!(
                    f,
                    "Press Enter to go back to menu\r\nWpm : {:?}\r\nAccuracy : {}%",
                    wpm, accuracy
                )?;
            }
        };
        Ok(())
    }
}
pub fn wpm(words: Vec<(char, CharStatus)>, duration: Duration) -> (f32, f32) {
    let accuracy = words
        .iter()
        .filter(|(_, s)| match s {
            CharStatus::Correct => true,
            _ => false,
        })
        .count() as f32
        / words.len() as f32;

    let word_count = words
        .split(|(c, _)| if c == &' ' { return true } else { return false })
        .count() as f32;

    (
        (((word_count / (duration.as_secs_f32() / 60.0)) * accuracy) * 100.0).round() / 100.0,
        (accuracy * 10000.0).round() / 100.0,
    )
}
