use rand::Rng;
use std::time::{Duration, Instant};
use termion::color::{Fg, Green, Red, Reset};
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
}
#[derive(Debug, Clone, Copy)]
pub enum CharStatus {
    Correct,
    Wrong,
    NotYet,
}
pub enum AppMode {
    Typing, //Index into phrase , starting
    Menu,
    Results, // (How long took to write , how many words in phrase)
}
impl App {
    pub fn new() -> Self {
        Self {
            mode: AppMode::Menu,
            current_phrase: vec![],
            index: 0,
            start: Instant::now(),
            end: Instant::now(),
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

    pub fn results(&mut self) -> f32 {
        self.mode = AppMode::Results;
        self.end = Instant::now();

        self.current_phrase
            .iter()
            .map(|f| f.0)
            .collect::<String>()
            .split(" ")
            .collect::<Vec<&str>>()
            .len() as f32
            * ((self.start - Instant::now()).as_secs() * 60) as f32
    }
    pub fn menu(&mut self) {
        self.mode = AppMode::Menu;
        self.current_phrase = vec![];
    }
}
impl std::fmt::Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match &self.mode {
            AppMode::Typing => {
                for char in &self.current_phrase {
                    match char.1 {
                        CharStatus::Correct => write!(f, "{}{}{}", Fg(Green), char.0, Fg(Reset))?,
                        CharStatus::Wrong => write!(f, "{}{}{}", Fg(Red), char.0, Fg(Reset))?,
                        CharStatus::NotYet => write!(f, "{}{}{}", Faint, char.0, Fg(Reset))?,
                    }
                }
            }
            AppMode::Menu => {}
            AppMode::Results => {
                let mut temp = self.current_phrase.clone();
                let mut words = vec![];
                loop {
                    if !temp.iter().any(|c| c.0 == ' ') {
                        break;
                    }
                    let word: Vec<(char, CharStatus)> =
                        temp.iter().take_while(|c| c.0 != ' ').cloned().collect();
                    let skip = word.len();
                    temp = temp
                        .iter()
                        .skip(skip + 1)
                        .skip_while(|c| c.0 == ' ')
                        .cloned()
                        .collect();

                    words.push(word);
                }

                let mut correct_count: f32 = 0.0;
                for word in words.iter() {
                    if word.iter().all(|(_, s)| match s {
                        CharStatus::Correct => true,
                        _ => false,
                    }) {
                        correct_count += 1.0;
                    }
                }
                let seconds = (self.end - self.start).as_secs_f32() / 60.0;
                let wpm = correct_count / seconds;
                write!(f, "You wpm was : {:?}", (wpm * 100.0).round() / 100.0)?;
            }
        };
        Ok(())
    }
}
