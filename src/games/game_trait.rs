use crossterm::event::KeyCode;
use tui::{backend::Backend, Frame, layout::Rect};
use crate::translation::Language;
use crate::game_manager::CompileLanguage;

pub trait Game {
    fn new() -> Self where Self: Sized;
    fn handle_input(&mut self, key: KeyCode);
    fn update(&mut self);
    fn set_language(&mut self, language: Language);
    fn set_compile_language(&mut self, lang: CompileLanguage);
    fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect);
} 