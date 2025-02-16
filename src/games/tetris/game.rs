use crossterm::event::KeyCode;
use rand::Rng;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

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
        }
    }

    pub fn handle_input(&mut self, key: KeyCode) -> bool {
        if self.game_over {
            return false;
        }

        match key {
            KeyCode::Left => self.move_piece(-1, 0),
            KeyCode::Right => self.move_piece(1, 0),
            KeyCode::Down => self.move_piece(0, 1),
            KeyCode::Up => {
                self.rotate_piece();
                true
            }
            KeyCode::Char(' ') => {
                self.hard_drop();
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {
        if self.game_over {
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

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let mut display_board = self.board.clone();

        // 将当前方块添加到显示板上
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
        for row in &display_board {
            let mut line = String::new();
            for &cell in row {
                line.push(if cell { '█' } else { '·' });
            }
            text.push(Spans::from(line));
        }

        text.push(Spans::from(""));
        text.push(Spans::from(format!("Score: {}", self.score)));
        if self.game_over {
            text.push(Spans::from("Game Over!"));
        }

        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled("Tetris", Style::default().fg(Color::Cyan))),
            )
            .alignment(tui::layout::Alignment::Center);
        f.render_widget(paragraph, area);
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
        let mut lines_cleared = 0;

        for y in (0..20).rev() {
            if self.board[y].iter().all(|&cell| cell) {
                self.board.remove(y);
                self.board.insert(0, vec![false; 10]);
                lines_cleared += 1;
            }
        }

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
}
