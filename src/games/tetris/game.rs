use crate::games::compiling::Compiling;
use crate::Game;
use crate::{
    game_manager::CompileLanguage,
    translation::{Language, Translations},
};
use crossterm::event::KeyCode;
use rand::Rng;
use std::cell::RefCell;
use tui::{
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::io::Stdout;
use tui::backend::CrosstermBackend;

#[derive(PartialEq)]
pub enum GameState {
    Welcome,
    Playing,
    Paused,
}

// 定义方块形状
const SHAPES: [[[bool; 4]; 4]; 7] = [
    // I形
    [
        [false, false, false, false],
        [true, true, true, true],
        [false, false, false, false],
        [false, false, false, false],
    ],
    // O形
    [
        [false, false, false, false],
        [false, true, true, false],
        [false, true, true, false],
        [false, false, false, false],
    ],
    // T形
    [
        [false, false, false, false],
        [false, true, false, false],
        [true, true, true, false],
        [false, false, false, false],
    ],
    // J形
    [
        [false, false, false, false],
        [true, false, false, false],
        [true, true, true, false],
        [false, false, false, false],
    ],
    // L形
    [
        [false, false, false, false],
        [false, false, true, false],
        [true, true, true, false],
        [false, false, false, false],
    ],
    // S形
    [
        [false, false, false, false],
        [false, true, true, false],
        [true, true, false, false],
        [false, false, false, false],
    ],
    // Z形
    [
        [false, false, false, false],
        [true, true, false, false],
        [false, true, true, false],
        [false, false, false, false],
    ],
];

pub struct Tetris {
    board: Vec<Vec<bool>>,
    current_piece: usize,
    piece_x: i32,
    piece_y: i32,
    game_over: bool,
    score: u32,
    tick_count: u32,
    current_shape: [[bool; 4]; 4],
    block_width: u16,
    game_state: GameState,
    translations: Translations,
    compiling: RefCell<Compiling>,
}

impl Tetris {
    pub fn new() -> Self {
        let piece = rand::thread_rng().gen_range(0..7);
        Self {
            board: vec![vec![false; 10]; 20],
            current_piece: piece,
            piece_x: 3,
            piece_y: 0,
            game_over: false,
            score: 0,
            tick_count: 0,
            current_shape: SHAPES[piece],
            block_width: 2,
            game_state: GameState::Welcome,
            translations: Translations::new(),
            compiling: RefCell::new(Compiling::new()),
        }
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
            GameState::Playing => match key {
                KeyCode::Left | KeyCode::Char('a') => self.move_piece(-1, 0),
                KeyCode::Right | KeyCode::Char('d') => self.move_piece(1, 0),
                KeyCode::Down | KeyCode::Char('s') => self.move_piece(0, 1),
                KeyCode::Up | KeyCode::Char('w') => {
                    self.rotate_piece();
                    true
                }
                KeyCode::Char(' ') => {
                    self.hard_drop();
                    true
                }
                KeyCode::Char('p') | KeyCode::Esc => {
                    self.game_state = GameState::Paused;
                    true
                }
                _ => false,
            },
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
        if self.game_over || self.game_state != GameState::Playing {
            if self.game_state == GameState::Paused {
                self.compiling.borrow_mut().update();
            }
            return;
        }

        self.tick_count += 1;
        if self.tick_count % 20 == 0 {
            if !self.move_piece(0, 1) {
                self.freeze_piece();
                self.clear_lines();
                self.spawn_new_piece();
            }
        }
    }

    pub fn render(&self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        match self.game_state {
            GameState::Welcome => self.render_welcome(f, area),
            GameState::Playing => self.render_game(f, area),
            GameState::Paused => self.render_pause(f, area),
        }
    }

    pub fn render_welcome(&self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let welcome_text = vec![
            Spans::from(vec![Span::styled(
                format!(
                    "{} {}!",
                    self.translations.get_text("welcome_to"),
                    self.translations.get_text("tetris_title")
                ),
                Style::default().fg(Color::Yellow),
            )]),
            Spans::from(""),
            Spans::from(self.translations.get_text("how_to_play")),
            Spans::from(""),
            Spans::from(self.translations.get_text("move_horizontal")),
            Spans::from(self.translations.get_text("speed_up")),
            Spans::from(self.translations.get_text("rotate")),
            Spans::from(self.translations.get_text("hard_drop")),
            Spans::from(self.translations.get_text("clear_lines")),
            Spans::from(self.translations.get_text("one_line")),
            Spans::from(self.translations.get_text("two_lines")),
            Spans::from(self.translations.get_text("three_lines")),
            Spans::from(self.translations.get_text("four_lines")),
            Spans::from(self.translations.get_text("game_ends")),
            Spans::from(""),
            Spans::from(self.translations.get_text("quit_control")),
            Spans::from(self.translations.get_text("press_enter")),
            Spans::from(self.translations.get_text("pause_game")),
            Spans::from(self.translations.get_text("restart")),
        ];

        let paragraph = Paragraph::new(welcome_text)
            .block(Block::default().borders(Borders::ALL).title(Span::styled(
                self.translations.get_text("tetris_title"),
                Style::default().fg(Color::Yellow),
            )))
            .alignment(tui::layout::Alignment::Center);
        f.render_widget(paragraph, area);
    }

    pub fn render_game(&self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let available_width = area.width as usize;
        let game_width = self.board[0].len() * self.block_width as usize;
        let padding = if available_width > game_width {
            (available_width - game_width) / 2
        } else {
            0
        };

        let mut display_board = self.board.clone();

        if !self.game_over {
            for y in 0..4 {
                for x in 0..4 {
                    if self.current_shape[y][x] {
                        let board_x = self.piece_x + x as i32;
                        let board_y = self.piece_y + y as i32;
                        if board_x >= 0 && board_x < 10 && board_y >= 0 && board_y < 20 {
                            display_board[board_y as usize][board_x as usize] = true;
                        }
                    }
                }
            }
        }

        let mut text = vec![];

        text.push(Spans::from(""));

        for row in &display_board {
            let mut line = " ".repeat(padding);
            for &cell in row {
                line.push_str(if cell { "██" } else { "··" });
            }
            text.push(Spans::from(line));
        }

        text.push(Spans::from(""));
        text.push(Spans::from(format!(
            "{} {}",
            self.translations.get_text("score"),
            self.score
        )));

        if self.game_over {
            text.push(Spans::from(self.translations.get_text("game_over")));
            text.push(Spans::from(self.translations.get_text("press_r_restart")));
        }

        let available_height = area.height as usize;
        let required_height = text.len();
        let start_index = if required_height > available_height {
            required_height - available_height
        } else {
            0
        };
        let visible_text = text[start_index..].to_vec();

        let paragraph = Paragraph::new(visible_text)
            .block(Block::default().borders(Borders::ALL).title(Span::styled(
                self.translations.get_text("tetris_title"),
                Style::default().fg(Color::Cyan),
            )))
            .alignment(tui::layout::Alignment::Left);

        f.render_widget(paragraph, area);
    }

    pub fn render_pause(&self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        if self.game_state == GameState::Paused {
            self.compiling.borrow_mut().render(f, area);
        }
    }

    fn move_piece(&mut self, dx: i32, dy: i32) -> bool {
        let new_x = self.piece_x + dx;
        let new_y = self.piece_y + dy;

        if self.is_valid_position(new_x, new_y) {
            self.piece_x = new_x;
            self.piece_y = new_y;
            true
        } else {
            false
        }
    }

    fn rotate_piece(&mut self) {
        let mut rotated = [[false; 4]; 4];
        for y in 0..4 {
            for x in 0..4 {
                rotated[x][3 - y] = self.current_shape[y][x];
            }
        }

        if self.is_valid_position_with_shape(self.piece_x, self.piece_y, &rotated) {
            self.current_shape = rotated;
        }
    }

    fn is_valid_position(&self, x: i32, y: i32) -> bool {
        for piece_y in 0..4 {
            for piece_x in 0..4 {
                if self.current_shape[piece_y][piece_x] {
                    let board_x = x + piece_x as i32;
                    let board_y = y + piece_y as i32;

                    if board_x < 0 || board_x >= 10 || board_y >= 20 {
                        return false;
                    }

                    if board_y >= 0 && self.board[board_y as usize][board_x as usize] {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn is_valid_position_with_shape(&self, x: i32, y: i32, shape: &[[bool; 4]; 4]) -> bool {
        for piece_y in 0..4 {
            for piece_x in 0..4 {
                if shape[piece_y][piece_x] {
                    let board_x = x + piece_x as i32;
                    let board_y = y + piece_y as i32;

                    if board_x < 0 || board_x >= 10 || board_y >= 20 {
                        return false;
                    }

                    if board_y >= 0 && self.board[board_y as usize][board_x as usize] {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn freeze_piece(&mut self) {
        for y in 0..4 {
            for x in 0..4 {
                if self.current_shape[y][x] {
                    let board_x = self.piece_x + x as i32;
                    let board_y = self.piece_y + y as i32;
                    if board_x >= 0 && board_x < 10 && board_y >= 0 && board_y < 20 {
                        self.board[board_y as usize][board_x as usize] = true;
                    }
                }
            }
        }
    }

    fn clear_lines(&mut self) {
        let mut lines_to_clear = vec![];

        // 首先找出所有需要清除的行
        for y in 0..20 {
            if self.board[y].iter().all(|&cell| cell) {
                lines_to_clear.push(y);
            }
        }

        // 从下往上清除行
        for &y in lines_to_clear.iter().rev() {
            self.board.remove(y);
            self.board.insert(0, vec![false; 10]);
        }

        // 根据消除的行数计算分数
        let lines_cleared = lines_to_clear.len();
        self.score += match lines_cleared {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0,
        };
    }

    fn spawn_new_piece(&mut self) {
        self.current_piece = rand::thread_rng().gen_range(0..7);
        self.piece_x = 3;
        self.piece_y = 0;
        self.current_shape = SHAPES[self.current_piece];

        if !self.is_valid_position(self.piece_x, self.piece_y) {
            self.game_over = true;
        }
    }

    fn hard_drop(&mut self) {
        while self.move_piece(0, 1) {}
        self.freeze_piece();
        self.clear_lines();
        self.spawn_new_piece();
    }

    pub fn set_compile_language(&mut self, lang: CompileLanguage) {
        self.compiling.borrow_mut().set_language(lang);
    }
}


impl Game for Tetris {
    fn new() -> Self {
        Tetris::new()
    }

    fn handle_input(&mut self, key: KeyCode) {
        let _ = self.handle_input(key);
    }

    fn update(&mut self) {
        Tetris::update(self);
    }

    fn render(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        Tetris::render(self, f, area);
    }

    fn set_language(&mut self, language: Language) {
        Tetris::set_language(self, language);
    }

    fn set_compile_language(&mut self, lang: CompileLanguage) {
        Tetris::set_compile_language(self, lang);
    }
}
