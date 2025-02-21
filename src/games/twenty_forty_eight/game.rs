use crate::games::compiling::Compiling;
use crate::Game;
use crate::{
    game_manager::CompileLanguage,
    translation::{Language, Translations},
};
use crossterm::event::KeyCode;
use rand::seq::SliceRandom;
use std::cell::RefCell;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[derive(PartialEq)]
pub enum GameState {
    Welcome,
    Playing,
    Paused,
}

// 在impl外部定义Direction
#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct TwentyFortyEight {
    board: Vec<Vec<u32>>,
    score: u32,
    game_over: bool,
    game_state: GameState,
    translations: Translations,
    compiling: RefCell<Compiling>,
}

impl TwentyFortyEight {
    pub fn new() -> Self {
        let mut game = Self {
            board: vec![vec![0; 4]; 4],
            score: 0,
            game_over: false,
            game_state: GameState::Welcome,
            translations: Translations::new(),
            compiling: RefCell::new(Compiling::new()),
        };
        game.spawn_new_tile();
        game.spawn_new_tile();
        game
    }

    pub fn set_language(&mut self, language: Language) {
        self.translations.set_language(language);
    }

    pub fn handle_input(&mut self, key: KeyCode) -> bool {
        if self.game_over {
            if key == KeyCode::Char('r') {
                *self = Self::new();
                self.game_state = GameState::Playing;
                return true;
            }
            return false;
        }

        match self.game_state {
            GameState::Welcome => {
                if key == KeyCode::Enter {
                    self.game_state = GameState::Playing;
                    true
                } else {
                    false
                }
            }
            GameState::Playing => {
                let moved = match key {
                    KeyCode::Left | KeyCode::Char('a') => self.move_tiles(Direction::Left),
                    KeyCode::Right | KeyCode::Char('d') => self.move_tiles(Direction::Right),
                    KeyCode::Up | KeyCode::Char('w') => self.move_tiles(Direction::Up),
                    KeyCode::Down | KeyCode::Char('s') => self.move_tiles(Direction::Down),
                    KeyCode::Char('p') | KeyCode::Esc => {
                        self.game_state = GameState::Paused;
                        true
                    }
                    _ => false,
                };

                if moved {
                    self.spawn_new_tile();
                    self.check_game_over();
                }
                moved
            }
            GameState::Paused => {
                if key == KeyCode::Char('p') || key == KeyCode::Esc {
                    self.game_state = GameState::Playing;
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn update(&mut self) {
        if self.game_state == GameState::Paused {
            self.compiling.borrow_mut().update();
        }
    }

    fn move_tiles(&mut self, direction: Direction) -> bool {
        let mut moved = false;
        match direction {
            Direction::Left => {
                for row in 0..4 {
                    moved |= self.merge_line(row, true, false);
                }
            }
            Direction::Right => {
                for row in 0..4 {
                    moved |= self.merge_line(row, true, true);
                }
            }
            Direction::Up => {
                for col in 0..4 {
                    moved |= self.merge_line(col, false, false);
                }
            }
            Direction::Down => {
                for col in 0..4 {
                    moved |= self.merge_line(col, false, true);
                }
            }
        }
        moved
    }

    fn merge_line(&mut self, index: usize, is_row: bool, reverse: bool) -> bool {
        let mut moved = false;
        let mut line: Vec<u32> = if is_row {
            self.board[index].clone()
        } else {
            (0..4).map(|i| self.board[i][index]).collect()
        };

        if reverse {
            line.reverse();
        }

        // 合并相同的数字
        let mut merged = vec![false; 4];
        for i in 0..3 {
            if line[i] != 0 {
                for j in (i + 1)..4 {
                    if line[j] != 0 {
                        if line[i] == line[j] && !merged[i] {
                            line[i] *= 2;
                            line[j] = 0;
                            merged[i] = true;
                            self.score += line[i];
                            moved = true;
                        }
                        break;
                    }
                }
            }
        }

        // 移动数字填补空位
        let mut new_line = vec![0; 4];
        let mut pos = 0;
        for &value in &line {
            if value != 0 {
                new_line[pos] = value;
                pos += 1;
            }
        }

        if reverse {
            new_line.reverse();
        }

        // 更新棋盘
        if is_row {
            if self.board[index] != new_line {
                moved = true;
                self.board[index] = new_line;
            }
        } else {
            for i in 0..4 {
                if self.board[i][index] != new_line[i] {
                    moved = true;
                    self.board[i][index] = new_line[i];
                }
            }
        }

        moved
    }

    fn spawn_new_tile(&mut self) {
        let mut empty_cells = Vec::new();
        for i in 0..4 {
            for j in 0..4 {
                if self.board[i][j] == 0 {
                    empty_cells.push((i, j));
                }
            }
        }

        if let Some(&(i, j)) = empty_cells.choose(&mut rand::thread_rng()) {
            self.board[i][j] = if rand::random::<f32>() < 0.9 { 2 } else { 4 };
        }
    }

    fn check_game_over(&mut self) {
        let mut has_empty = false;
        let mut can_merge = false;

        // 检查空格子和相邻格子是否可以合并
        for i in 0..4 {
            for j in 0..4 {
                if self.board[i][j] == 0 {
                    has_empty = true;
                }
                // 检查右侧相邻格子
                if j < 3 && self.board[i][j] == self.board[i][j + 1] {
                    can_merge = true;
                }
                // 检查下方相邻格子
                if i < 3 && self.board[i][j] == self.board[i + 1][j] {
                    can_merge = true;
                }
            }
        }

        self.game_over = !has_empty && !can_merge;
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        match self.game_state {
            GameState::Welcome => self.render_welcome(f, area),
            GameState::Playing => self.render_game(f, area),
            GameState::Paused => self.render_pause(f, area),
        }
    }

    fn render_welcome<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let welcome_text = vec![
            Spans::from(vec![Span::styled(
                self.translations.get_text("welcome_title"),
                Style::default().fg(Color::Yellow),
            )]),
            Spans::from(""),
            Spans::from(self.translations.get_text("press_enter")),
        ];

        let paragraph = Paragraph::new(welcome_text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(tui::layout::Alignment::Center);
        f.render_widget(paragraph, area);
    }

    fn render_game<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let mut text = vec![];

        // 添加顶部边框
        text.push(Spans::from("┌────┬────┬────┬────┐"));

        for (i, row) in self.board.iter().enumerate() {
            // 添加数字行
            let line = row
                .iter()
                .map(|&n| {
                    let num_str = if n == 0 {
                        "    ".to_string()
                    } else {
                        format!("{:^4}", n)
                    };
                    num_str
                })
                .collect::<Vec<_>>()
                .join("│");
            text.push(Spans::from(format!("│{}│", line)));

            // 除了最后一行，每行数字后面添加分隔线
            if i < self.board.len() - 1 {
                text.push(Spans::from("├────┼────┼────┼────┤"));
            }
        }

        // 添加底部边框
        text.push(Spans::from("└────┴────┴────┴────┘"));
        text.push(Spans::from(""));
        text.push(Spans::from(format!(
            "{}{}",
            self.translations.get_text("score"),
            self.score
        )));

        // 添加游戏结束提示
        if self.game_over {
            text.push(Spans::from(""));
            text.push(Spans::from(vec![Span::styled(
                self.translations.get_text("game_over"),
                Style::default().fg(Color::Red),
            )]));
        }

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(tui::layout::Alignment::Center);
        f.render_widget(paragraph, area);
    }

    fn render_pause<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        self.compiling.borrow_mut().render(f, area);
    }

    pub fn set_compile_language(&mut self, language: CompileLanguage) {
        self.compiling.borrow_mut().set_language(language);
    }
}

impl Game for TwentyFortyEight {
    fn new() -> Self {
        TwentyFortyEight::new()
    }

    fn handle_input(&mut self, key: KeyCode) {
        self.handle_input(key);
    }

    fn update(&mut self) {
        self.update();
    }

    fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        TwentyFortyEight::render(self, f, area);
    }

    fn set_language(&mut self, language: Language) {
        TwentyFortyEight::set_language(self, language);
    }

    fn set_compile_language(&mut self, lang: CompileLanguage) {
        TwentyFortyEight::set_compile_language(self, lang);
    }
}