use crossterm::event::KeyCode;
use std::{cell::RefCell, io::Stdout, time::Instant};
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::translation::{Language, Translations};
use crate::{game_manager::CompileLanguage, games::compiling::Compiling, games::game_trait::Game};

pub struct MineSweeper {
    // pub board: Vec<Vec<u8>>,
    // pub mines: Vec<Vec<u8>>,
    // pub revealed: Vec<Vec<bool>>,
    // pub flagged: Vec<Vec<bool>>,
    // pub game_over: bool,
}

impl Game for MineSweeper {
    fn new() -> Self {
        Self::new()
    }

    fn handle_input(&mut self, key: KeyCode) {
        // match key {
        //   KeyCode::Char('q') => {
        //     self.quit();
        //   }
    }

    fn update(&mut self) {
        // 更新游戏逻辑
    }

    fn render(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        // 渲染游戏界面
    }

    fn set_language(&mut self, language: Language) {
        // 设置语言
    }

    fn set_compile_language(&mut self, lang: CompileLanguage) {
        // 设置编译语言
    }
}

impl MineSweeper {
    pub fn new() -> MineSweeper {
        Self {}
    }
}
