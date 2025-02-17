use crate::games::{goldminer::GoldMiner, tetris::Tetris};
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::translation::{Language, Translations};

#[derive(PartialEq)]
pub enum GameState {
    MainMenu,
    GoldMiner,
    Tetris,
}

pub struct GameManager {
    pub state: GameState,
    goldminer: GoldMiner,
    tetris: Tetris,
    selected_game: usize,
    translations: Translations,
    selecting_language: bool,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            state: GameState::MainMenu,
            goldminer: GoldMiner::new(),
            tetris: Tetris::new(),
            selected_game: 0,
            translations: Translations::new(),
            selecting_language: false,
        }
    }

    pub fn handle_input(&mut self, key: KeyCode) {
        match self.state {
            GameState::MainMenu => {
                if self.selecting_language {
                    match key {
                        KeyCode::Char('e') => {
                            self.translations.set_language(Language::English);
                            self.goldminer.set_language(Language::English);
                            self.tetris.set_language(Language::English);
                            self.selecting_language = false;
                        }
                        KeyCode::Char('c') => {
                            self.translations.set_language(Language::Chinese);
                            self.goldminer.set_language(Language::Chinese);
                            self.tetris.set_language(Language::Chinese);
                            self.selecting_language = false;
                        }
                        KeyCode::Esc => self.selecting_language = false,
                        _ => {}
                    }
                } else {
                    match key {
                        KeyCode::Char('t') => self.selecting_language = true,
                        KeyCode::Char('1') => self.state = GameState::GoldMiner,
                        KeyCode::Char('2') => self.state = GameState::Tetris,
                        KeyCode::Up => {
                            if self.selected_game > 0 {
                                self.selected_game -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if self.selected_game < 1 {
                                self.selected_game += 1;
                            }
                        }
                        KeyCode::Enter => {
                            self.state = match self.selected_game {
                                0 => GameState::GoldMiner,
                                1 => GameState::Tetris,
                                _ => GameState::MainMenu,
                            };
                        }
                        _ => {}
                    }
                }
            }
            GameState::GoldMiner => {
                let _ = self.goldminer.handle_input(key);
            }
            GameState::Tetris => {
                let _ = self.tetris.handle_input(key);
            }
        }
    }

    pub fn update(&mut self) {
        match self.state {
            GameState::MainMenu => {}
            GameState::GoldMiner => self.goldminer.update(),
            GameState::Tetris => self.tetris.update(),
        }
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        match self.state {
            GameState::MainMenu => self.render_main_menu(f, area),
            GameState::GoldMiner => self.goldminer.render(f, area),
            GameState::Tetris => self.tetris.render(f, area),
        }
    }

    fn render_main_menu<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let mut menu_text = vec![
            Spans::from(vec![Span::styled(
                self.translations.get_text("menu_title"),
                Style::default().fg(Color::Yellow),
            )]),
            Spans::from(""),
            Spans::from(self.translations.get_text("available_games")),
            Spans::from(""),
            Spans::from(vec![Span::styled(
                format!(" 1. {}", self.translations.get_text("goldminer_title")),
                Style::default().fg(if self.selected_game == 0 {
                    Color::Green
                } else {
                    Color::White
                }),
            )]),
            Spans::from(vec![Span::styled(
                format!(" 2. {}", self.translations.get_text("tetris_title")),
                Style::default().fg(if self.selected_game == 1 {
                    Color::Green
                } else {
                    Color::White
                }),
            )]),
            Spans::from(""),
            Spans::from(self.translations.get_text("controls")),
        ];

        // Add language selection prompt if active
        if self.selecting_language {
            menu_text.push(Spans::from(""));
            menu_text.push(Spans::from(vec![Span::styled(
                "Select Language / 选择语言:",
                Style::default().fg(Color::Yellow),
            )]));
            menu_text.push(Spans::from("E: English"));
            menu_text.push(Spans::from("C: 中文"));
            menu_text.push(Spans::from("ESC: Cancel / 取消"));
        } else {
            for line in self.translations.get_text("controls_desc").split('\n') {
                menu_text.push(Spans::from(line.to_string()));
            }
        }

        let paragraph = Paragraph::new(menu_text)
            .block(Block::default().borders(Borders::ALL).title(Span::styled(
                self.translations.get_text("menu_title"),
                Style::default().fg(Color::Yellow),
            )))
            .alignment(tui::layout::Alignment::Center);
        f.render_widget(paragraph, area);
    }
}
