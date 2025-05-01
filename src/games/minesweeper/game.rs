use crossterm::event::KeyCode;
use std::{cell::RefCell, io::Stdout};
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

const BOARD_SIZE: usize = 10;
const MINE_COUNT: usize = 15;

#[derive(PartialEq)]
pub enum GameState {
    Welcome,
    Playing,
    Paused,
}

pub struct MineSweeper {
    board: Vec<Vec<u8>>,
    mines: Vec<Vec<bool>>,
    revealed: Vec<Vec<bool>>,
    flagged: Vec<Vec<bool>>,
    game_over: bool,
    is_win: bool,
    cursor_x: usize,
    cursor_y: usize,
    language: Language,
    compile_language: CompileLanguage,
    game_state: GameState,
    translations: Translations,
    compiling: RefCell<Compiling>,
    last_click_x: Option<usize>,
    last_click_y: Option<usize>,
}

impl Game for MineSweeper {
    fn new() -> Self {
        Self::new()
    }

    fn handle_input(&mut self, key: KeyCode) {
        match self.game_state {
            GameState::Welcome => {
                if key == KeyCode::Enter {
                    self.game_state = GameState::Playing;
                }
            }
            GameState::Playing => {
                if self.game_over {
                    if key == KeyCode::Char('r') {
                        *self = Self::new();
                        self.game_state = GameState::Playing;
                    }
                    return;
                }

                match key {
                    KeyCode::Up | KeyCode::Char('w') => {
                        if self.cursor_y > 0 {
                            self.cursor_y -= 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('s') => {
                        if self.cursor_y < BOARD_SIZE - 1 {
                            self.cursor_y += 1;
                        }
                    }
                    KeyCode::Left | KeyCode::Char('a') => {
                        if self.cursor_x > 0 {
                            self.cursor_x -= 1;
                        }
                    }
                    KeyCode::Right | KeyCode::Char('d') => {
                        if self.cursor_x < BOARD_SIZE - 1 {
                            self.cursor_x += 1;
                        }
                    }
                    KeyCode::Char(' ') => {
                        if !self.flagged[self.cursor_y][self.cursor_x] {
                            self.last_click_x = Some(self.cursor_x);
                            self.last_click_y = Some(self.cursor_y);
                            self.reveal(self.cursor_x, self.cursor_y);
                        }
                    }
                    KeyCode::Char('f') => {
                        if !self.revealed[self.cursor_y][self.cursor_x] {
                            self.flagged[self.cursor_y][self.cursor_x] = !self.flagged[self.cursor_y][self.cursor_x];
                            self.check_win();
                        }
                    }
                    KeyCode::Char('p') | KeyCode::Esc => {
                        self.game_state = GameState::Paused;
                    }
                    _ => {}
                }
            }
            GameState::Paused => {
                if key == KeyCode::Char('p') || key == KeyCode::Esc {
                    self.game_state = GameState::Playing;
                }
            }
        }
    }

    fn update(&mut self) {
        if self.game_state == GameState::Paused {
            self.compiling.borrow_mut().update();
        }
    }

    fn render(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        match self.game_state {
            GameState::Welcome => self.render_welcome(f, area),
            GameState::Playing => self.render_game(f, area),
            GameState::Paused => self.render_pause(f, area),
        }
    }

    fn set_language(&mut self, language: Language) {
        self.language = language;
        self.translations.set_language(language);
    }

    fn set_compile_language(&mut self, lang: CompileLanguage) {
        self.compile_language = lang;
        self.compiling.borrow_mut().set_language(lang);
    }
}

impl MineSweeper {
    pub fn new() -> MineSweeper {
        let mut game = MineSweeper {
            board: vec![vec![0; BOARD_SIZE]; BOARD_SIZE],
            mines: vec![vec![false; BOARD_SIZE]; BOARD_SIZE],
            revealed: vec![vec![false; BOARD_SIZE]; BOARD_SIZE],
            flagged: vec![vec![false; BOARD_SIZE]; BOARD_SIZE],
            game_over: false,
            is_win: false,
            cursor_x: 0,
            cursor_y: 0,
            language: Language::English,
            compile_language: CompileLanguage::Rust,
            game_state: GameState::Welcome,
            translations: Translations::new(),
            compiling: RefCell::new(Compiling::new()),
            last_click_x: None,
            last_click_y: None,
        };
        game.place_mines();
        game.calculate_numbers();
        game
    }

    fn render_welcome(&self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let welcome_text = vec![
            Spans::from(vec![Span::styled(
                self.translations.get_text("minesweeper_title"),
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

    fn render_game(&self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let mut text = vec![];

        // 添加顶部边框
        text.push(Spans::from("┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┐"));

        for y in 0..BOARD_SIZE {
            let mut line = Vec::new();
            for x in 0..BOARD_SIZE {
                let mut style = Style::default();
                let cell_str = if self.revealed[y][x] {
                    if self.mines[y][x] {
                        " 💣 ".to_string()
                    } else {
                        let count = self.board[y][x];
                        if count > 0 {
                            format!("{:^4}", count)
                        } else {
                            "    ".to_string()
                        }
                    }
                } else if self.flagged[y][x] {
                    " 🚩 ".to_string()
                } else {
                    "  ■ ".to_string()
                };

                // 设置当前光标位置的背景色为黄色
                if x == self.cursor_x && y == self.cursor_y {
                    style = style.bg(Color::Yellow);
                }
                line.push(Span::styled(cell_str, style));
                line.push(Span::raw("│"));
            }
            line.insert(0, Span::raw("│"));
            text.push(Spans::from(line));

            if y < BOARD_SIZE - 1 {
                text.push(Spans::from("├────┼────┼────┼────┼────┼────┼────┼────┼────┼────┤"));
            }
        }

        // 添加底部边框
        text.push(Spans::from("└────┴────┴────┴────┴────┴────┴────┴────┴────┴────┘"));

        // 添加游戏结束提示
        if self.game_over {
            text.push(Spans::from(""));
            if self.is_win {
                text.push(Spans::from(vec![Span::styled(
                    self.translations.get_text("game_win"),
                    Style::default().fg(Color::Green),
                )]));
            } else {
                text.push(Spans::from(vec![Span::styled(
                    self.translations.get_text("game_over"),
                    Style::default().fg(Color::Red),
                )]));
            }
            text.push(Spans::from(self.translations.get_text("press_r_restart")));
        }

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(tui::layout::Alignment::Center);
        f.render_widget(paragraph, area);
    }

    fn render_pause(&self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        self.compiling.borrow_mut().render(f, area);
    }

    fn place_mines(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut mines_placed = 0;

        while mines_placed < MINE_COUNT {
            let x = rng.gen_range(0..BOARD_SIZE);
            let y = rng.gen_range(0..BOARD_SIZE);
            if !self.mines[y][x] {
                self.mines[y][x] = true;
                mines_placed += 1;
            }
        }
    }

    fn calculate_numbers(&mut self) {
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if !self.mines[y][x] {
                    let mut count = 0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;
                            if nx >= 0 && nx < BOARD_SIZE as i32 && ny >= 0 && ny < BOARD_SIZE as i32 {
                                if self.mines[ny as usize][nx as usize] {
                                    count += 1;
                                }
                            }
                        }
                    }
                    self.board[y][x] = count;
                }
            }
        }
    }

    fn reveal(&mut self, x: usize, y: usize) {
        if self.revealed[y][x] || self.flagged[y][x] {
            return;
        }

        self.revealed[y][x] = true;

        if self.mines[y][x] {
            self.game_over = true;
            // 显示所有地雷
            for y in 0..BOARD_SIZE {
                for x in 0..BOARD_SIZE {
                    if self.mines[y][x] {
                        self.revealed[y][x] = true;
                    }
                }
            }
            return;
        }

        if self.board[y][x] == 0 {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < BOARD_SIZE as i32 && ny >= 0 && ny < BOARD_SIZE as i32 {
                        self.reveal(nx as usize, ny as usize);
                    }
                }
            }
        }

        self.check_win();
    }

    fn check_win(&mut self) {
        // 检查是否所有非地雷格子都已揭开
        let mut all_safe_revealed = true;
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if !self.mines[y][x] && !self.revealed[y][x] {
                    all_safe_revealed = false;
                    break;
                }
            }
            if !all_safe_revealed {
                break;
            }
        }

        // 检查是否所有地雷都被正确标记
        let mut all_mines_flagged = true;
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if self.mines[y][x] != self.flagged[y][x] {
                    all_mines_flagged = false;
                    break;
                }
            }
            if !all_mines_flagged {
                break;
            }
        }

        // 任一条件满足即胜利
        if all_safe_revealed || all_mines_flagged {
            self.is_win = true;
            self.game_over = true;
            // 胜利时显示所有地雷位置
            for y in 0..BOARD_SIZE {
                for x in 0..BOARD_SIZE {
                    if self.mines[y][x] {
                        self.revealed[y][x] = true;
                    }
                }
            }
        }
    }
}
