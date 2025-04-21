use crate::games::{
    goldminer::GoldMiner, minesweeper::MineSweeper, snake::Snake, tetris::Tetris,
    twenty_forty_eight::TwentyFortyEight,
};
use crate::translation::{Language, Translations};
use crate::Game;
use crossterm::event::KeyCode;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::{
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[derive(PartialEq, Clone, Copy)]
pub enum CompileLanguage {
    Rust,
    Go,
    CMake,
}

// 定义所有游戏的枚举
#[derive(PartialEq, Clone, Copy)]
pub enum GameType {
    MainMenu,
    GoldMiner,
    Tetris,
    Snake,
    TwentyFortyEight,
    MineSweeper,
}

// 移除 GameEnum，直接使用 Box<dyn Game>
pub struct GameInfo {
    game_type: GameType,
    title_key: &'static str,
    game: Box<dyn Game>,
}

pub struct GameManager {
    pub state: GameType,
    games: Vec<GameInfo>, // 改为 Vec 而不是固定大小数组
    selected_game: usize,
    translations: Translations,
    selecting_language: bool,
    compile_language: CompileLanguage,
    selecting_compile_language: bool,
}

impl GameManager {
    pub fn new() -> Self {
        // 定义所有游戏
        let games = vec![
            GameInfo {
                game_type: GameType::GoldMiner,
                title_key: "goldminer_title",
                game: Box::new(GoldMiner::new()),
            },
            GameInfo {
                game_type: GameType::Tetris,
                title_key: "tetris_title",
                game: Box::new(Tetris::new()),
            },
            GameInfo {
                game_type: GameType::Snake,
                title_key: "snake_title",
                game: Box::new(Snake::new()),
            },
            GameInfo {
                game_type: GameType::TwentyFortyEight,
                title_key: "twenty_forty_eight_title",
                game: Box::new(TwentyFortyEight::new()),
            },
            GameInfo {
                game_type: GameType::MineSweeper,
                title_key: "minesweeper_title",
                game: Box::new(MineSweeper::new()),
            },
        ];

        Self {
            state: GameType::MainMenu,
            games,
            selected_game: 0,
            translations: Translations::new(),
            selecting_language: false,
            compile_language: CompileLanguage::Rust,
            selecting_compile_language: false,
        }
    }

    pub fn handle_input(&mut self, key: KeyCode) {
        match self.state {
            GameType::MainMenu => {
                if self.selecting_language {
                    match key {
                        KeyCode::Char('e') => {
                            self.translations.set_language(Language::English);
                            self.set_language(Language::English);
                            self.selecting_language = false;
                        }
                        KeyCode::Char('c') => {
                            self.translations.set_language(Language::Chinese);
                            self.set_language(Language::Chinese);
                            self.selecting_language = false;
                        }
                        KeyCode::Esc => self.selecting_language = false,
                        _ => {}
                    }
                } else if self.selecting_compile_language {
                    match key {
                        KeyCode::Char('r') => {
                            self.compile_language = CompileLanguage::Rust;
                            self.set_compile_language(CompileLanguage::Rust);
                            self.selecting_compile_language = false;
                        }
                        KeyCode::Char('g') => {
                            self.compile_language = CompileLanguage::Go;
                            self.set_compile_language(CompileLanguage::Go);
                            self.selecting_compile_language = false;
                        }
                        KeyCode::Char('m') => {
                            self.compile_language = CompileLanguage::CMake;
                            self.set_compile_language(CompileLanguage::CMake);
                            self.selecting_compile_language = false;
                        }
                        KeyCode::Esc => self.selecting_compile_language = false,
                        _ => {}
                    }
                } else {
                    match key {
                        KeyCode::Char(c) => {
                            // 通过数字选择游戏
                            if let Some(index) = c.to_digit(10) {
                                let index = index as usize - 1;
                                if index < self.games.len() {
                                    self.state = self.games[index].game_type;
                                }
                            }
                        }
                        KeyCode::Up => {
                            if self.selected_game > 0 {
                                self.selected_game -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if self.selected_game < self.games.len() - 1 {
                                self.selected_game += 1;
                            }
                        }
                        KeyCode::Enter => {
                            self.state = self.games[self.selected_game].game_type;
                        }
                        _ => {}
                    }
                }
            }
            _ => {
                if let Some(game_info) = self.games.iter_mut().find(|g| g.game_type == self.state) {
                    game_info.game.handle_input(key);
                }
            }
        }
    }

    pub fn update(&mut self) {
        if let Some(game_info) = self.games.iter_mut().find(|g| g.game_type == self.state) {
            game_info.game.update();
        }
    }

    pub fn render(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        match self.state {
            GameType::MainMenu => self.render_main_menu(f, area),
            _ => {
                if let Some(game_info) = self.games.iter_mut().find(|g| g.game_type == self.state) {
                    game_info.game.render(f, area);
                }
            }
        }
    }

    fn render_main_menu(&self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let mut menu_text = vec![
            Spans::from(vec![Span::styled(
                self.translations.get_text("menu_title"),
                Style::default().fg(Color::Yellow),
            )]),
            Spans::from(""),
            Spans::from(self.translations.get_text("available_games")),
            Spans::from(""),
        ];

        // 动态生成游戏列表
        for (index, game_info) in self.games.iter().enumerate() {
            menu_text.push(Spans::from(vec![Span::styled(
                format!(
                    " {}. {}",
                    index + 1,
                    self.translations.get_text(game_info.title_key)
                ),
                Style::default().fg(if self.selected_game == index {
                    Color::Green
                } else {
                    Color::White
                }),
            )]));
        }

        if self.selecting_compile_language {
            menu_text.push(Spans::from(""));
            menu_text.push(Spans::from(vec![Span::styled(
                "Select Compile Language / 选择编译语言:",
                Style::default().fg(Color::Yellow),
            )]));
            menu_text.push(Spans::from("R: Rust"));
            menu_text.push(Spans::from("G: Go"));
            menu_text.push(Spans::from("M: CMake"));
            menu_text.push(Spans::from("ESC: Cancel / 取消"));
        } else if self.selecting_language {
            menu_text.push(Spans::from(""));
            menu_text.push(Spans::from(vec![Span::styled(
                "Select Language / 选择语言:",
                Style::default().fg(Color::Yellow),
            )]));
            menu_text.push(Spans::from("E: English"));
            menu_text.push(Spans::from("C: 中文"));
            menu_text.push(Spans::from("ESC: Cancel / 取消"));
        } else {
            for line in self.translations.get_text("controls").split('\n') {
                menu_text.push(Spans::from(line.to_string()));
            }
            menu_text.push(Spans::from(format!(
                "{} ({})",
                self.translations.get_text("compiling"),
                match self.compile_language {
                    CompileLanguage::Rust => "Rust",
                    CompileLanguage::Go => "Go",
                    CompileLanguage::CMake => "CMake",
                }
            )));
            menu_text.push(Spans::from(
                self.translations.get_text("compiling_language"),
            ));
        }

        let paragraph = Paragraph::new(menu_text)
            .block(Block::default().borders(Borders::ALL).title(Span::styled(
                self.translations.get_text("menu_title"),
                Style::default().fg(Color::Yellow),
            )))
            .alignment(tui::layout::Alignment::Center);
        f.render_widget(paragraph, area);
    }

    pub fn set_language(&mut self, language: Language) {
        self.translations.set_language(language);
        for game_info in &mut self.games {
            game_info.game.set_language(language);
        }
    }

    pub fn set_compile_language(&mut self, lang: CompileLanguage) {
        self.compile_language = lang;
        for game_info in &mut self.games {
            game_info.game.set_compile_language(lang);
        }
    }
}
