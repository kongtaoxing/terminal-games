use crate::game_manager::CompileLanguage;
use crate::translation::Language;
use crossterm::event::KeyCode;
use std::io::Stdout;
use tui::{backend::CrosstermBackend, layout::Rect, Frame};

pub trait Game {
    fn new() -> Self
    where
        Self: Sized;
    fn handle_input(&mut self, key: KeyCode);
    fn update(&mut self);
    fn set_language(&mut self, language: Language);
    fn set_compile_language(&mut self, lang: CompileLanguage);
    fn render(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect);
}
