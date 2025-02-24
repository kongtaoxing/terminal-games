use crate::games::{
    goldminer::GoldMiner, snake::Snake, tetris::Tetris, twenty_forty_eight::TwentyFortyEight,
};
use crate::translation::{Language, Translations};
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
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
}

// 使用枚举替代 trait object
enum GameEnum {
    GoldMiner(GoldMiner),
    Tetris(Tetris),
    Snake(Snake),
    TwentyFortyEight(TwentyFortyEight),
}

impl GameEnum {
    fn handle_input(&mut self, key: KeyCode) -> () {
        match self {
            Self::GoldMiner(game) => { game.handle_input(key); },
            Self::Tetris(game) => { game.handle_input(key); },
            Self::Snake(game) => { game.handle_input(key); },
            Self::TwentyFortyEight(game) => { game.handle_input(key); },
        }
    }

    fn update(&mut self) {
        match self {
            Self::GoldMiner(game) => game.update(),
            Self::Tetris(game) => game.update(),
            Self::Snake(game) => game.update(),
            Self::TwentyFortyEight(game) => game.update(),
        }
    }

    fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        match self {
            Self::GoldMiner(game) => game.render(f, area),
            Self::Tetris(game) => game.render(f, area),
            Self::Snake(game) => game.render(f, area),
            Self::TwentyFortyEight(game) => game.render(f, area),
        }
    }

    fn set_language(&mut self, language: Language) {
        match self {
            Self::GoldMiner(game) => game.set_language(language),
            Self::Tetris(game) => game.set_language(language),
            Self::Snake(game) => game.set_language(language),
            Self::TwentyFortyEight(game) => game.set_language(language),
        }
    }

    fn set_compile_language(&mut self, lang: CompileLanguage) {
        match self {
            Self::GoldMiner(game) => game.set_compile_language(lang),
            Self::Tetris(game) => game.set_compile_language(lang),
            Self::Snake(game) => game.set_compile_language(lang),
            Self::TwentyFortyEight(game) => game.set_compile_language(lang),
        }
    }
}

// 定义游戏信息结构体
struct GameInfo {
    game_type: GameType,
    title_key: &'static str,
    game: GameEnum,
}

pub struct GameManager {
    pub state: GameType,
    games: [GameInfo; 4], // 固定大小的数组
    selected_game: usize,
    translations: Translations,
    selecting_language: bool,
    compile_language: CompileLanguage,
    selecting_compile_language: bool,
}

impl GameManager {
    pub fn new() -> Self {
        // 定义所有游戏
        let games = [
            GameInfo {
                game_type: GameType::GoldMiner,
                title_key: "goldminer_title",
                game: GameEnum::GoldMiner(GoldMiner::new()),
            },
            GameInfo {
                game_type: GameType::Tetris,
                title_key: "tetris_title",
                game: GameEnum::Tetris(Tetris::new()),
            },
            GameInfo {
                game_type: GameType::Snake,
                title_key: "snake_title",
                game: GameEnum::Snake(Snake::new()),
            },
            GameInfo {
                game_type: GameType::TwentyFortyEight,
                title_key: "twenty_forty_eight_title",
                game: GameEnum::TwentyFortyEight(TwentyFortyEight::new()),
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
                if let Some(game_info) = self.games.iter_mut()
                    .find(|g| g.game_type == self.state) {
                    game_info.game.handle_input(key);
                }
            }
        }
    }

    pub fn update(&mut self) {
        if let Some(game_info) = self.games.iter_mut()
            .find(|g| g.game_type == self.state) {
            game_info.game.update();
        }
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        match self.state {
            GameType::MainMenu => self.render_main_menu(f, area),
            _ => {
                if let Some(game_info) = self.games.iter_mut()
                    .find(|g| g.game_type == self.state) {
                    game_info.game.render(f, area);
                }
            }
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
        ];

        // 动态生成游戏列表
        for (index, game_info) in self.games.iter().enumerate() {
            menu_text.push(Spans::from(vec![Span::styled(
                format!(" {}. {}", 
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
