use crate::games::compiling::Compiling;
use crate::Game;
use crate::{
    game_manager::CompileLanguage,
    translation::{Language, Translations},
};
use crossterm::event::KeyCode;
use rand::Rng;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::{
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

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, PartialEq)]
enum FoodType {
    Apple, // 大食物
    Candy, // 小食物
}

#[derive(Clone, PartialEq)]
struct Food {
    position: Position,
    food_type: FoodType,
    positions: Vec<Position>, // 用于存储大食物的所有位置
}

pub struct Snake {
    // board: Vec<Vec<bool>>,
    snake_body: VecDeque<Position>,
    food: Food,
    direction: Direction,
    game_over: bool,
    score: u32,
    tick_count: u32,
    game_state: GameState,
    translations: Translations,
    compiling: RefCell<Compiling>,
    next_direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        let mut snake = Self {
            // board: vec![vec![false; 20]; 20],
            snake_body: VecDeque::new(),
            food: Food {
                position: Position { x: 0, y: 0 },
                food_type: FoodType::Apple,
                positions: Vec::new(),
            },
            direction: Direction::Right,
            game_over: false,
            score: 0,
            tick_count: 0,
            game_state: GameState::Welcome,
            translations: Translations::new(),
            compiling: RefCell::new(Compiling::new()),
            next_direction: Direction::Right,
        };

        snake.snake_body.push_back(Position { x: 10, y: 10 });
        snake.spawn_food();
        snake
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
                KeyCode::Left | KeyCode::Char('a') => {
                    if self.direction != Direction::Right {
                        self.next_direction = Direction::Left;
                    }
                    true
                }
                KeyCode::Right | KeyCode::Char('d') => {
                    if self.direction != Direction::Left {
                        self.next_direction = Direction::Right;
                    }
                    true
                }
                KeyCode::Up | KeyCode::Char('w') => {
                    if self.direction != Direction::Down {
                        self.next_direction = Direction::Up;
                    }
                    true
                }
                KeyCode::Down | KeyCode::Char('s') => {
                    if self.direction != Direction::Up {
                        self.next_direction = Direction::Down;
                    }
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
        if self.tick_count % 10 == 0 {
            self.direction = self.next_direction.clone();
            self.move_snake();
        }
    }

    fn spawn_food(&mut self) {
        let mut rng = rand::thread_rng();

        // 随机选择食物类型
        self.food.food_type = if rng.gen_bool(0.7) {
            FoodType::Apple // 70%概率生成苹果
        } else {
            FoodType::Candy // 30%概率生成糖果
        };

        loop {
            let x = rng.gen_range(0..20);
            let y = rng.gen_range(0..20);
            let base_pos = Position { x, y };

            // 检查是否有足够的空间放置食物
            match self.food.food_type {
                FoodType::Apple => {
                    // 检查2x2的空间
                    let positions = vec![
                        base_pos.clone(),
                        Position {
                            x: base_pos.x + 1,
                            y: base_pos.y,
                        },
                        Position {
                            x: base_pos.x,
                            y: base_pos.y + 1,
                        },
                        Position {
                            x: base_pos.x + 1,
                            y: base_pos.y + 1,
                        },
                    ];

                    if x < 19
                        && y < 19
                        && positions.iter().all(|pos| {
                            !self.snake_body.iter().any(|p| p.x == pos.x && p.y == pos.y)
                        })
                    {
                        self.food.position = base_pos;
                        self.food.positions = positions;
                        break;
                    }
                }
                FoodType::Candy => {
                    if !self.snake_body.iter().any(|p| p.x == x && p.y == y) {
                        self.food.position = base_pos.clone();
                        self.food.positions = vec![base_pos];
                        break;
                    }
                }
            }
        }
    }

    fn check_food_collision(&mut self, head: &Position) -> bool {
        self.food
            .positions
            .iter()
            .any(|food_pos| food_pos.x == head.x && food_pos.y == head.y)
    }

    fn move_snake(&mut self) {
        if let Some(head) = self.snake_body.front() {
            let new_head = match self.direction {
                Direction::Up => Position {
                    x: head.x,
                    y: head.y - 1,
                },
                Direction::Down => Position {
                    x: head.x,
                    y: head.y + 1,
                },
                Direction::Left => Position {
                    x: head.x - 1,
                    y: head.y,
                },
                Direction::Right => Position {
                    x: head.x + 1,
                    y: head.y,
                },
            };

            // 检查是否撞墙或撞到自己
            if new_head.x < 0
                || new_head.x >= 20
                || new_head.y < 0
                || new_head.y >= 20
                || self
                    .snake_body
                    .iter()
                    .any(|p| p.x == new_head.x && p.y == new_head.y)
            {
                self.game_over = true;
                return;
            }

            // 检查是否吃到食物
            if self.check_food_collision(&new_head) {
                match self.food.food_type {
                    FoodType::Apple => self.score += 50,  // 苹果分值降低
                    FoodType::Candy => self.score += 150, // 糖果分值提高
                }
                self.spawn_food();
            } else {
                self.snake_body.pop_back();
            }

            self.snake_body.push_front(new_head);
        }
    }

    pub fn render(&self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        match self.game_state {
            GameState::Welcome => self.render_welcome(f, area),
            GameState::Playing => self.render_game(f, area),
            GameState::Paused => self.render_pause(f, area),
        }
    }

    fn render_welcome(&self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let welcome_text = vec![
            Spans::from(vec![Span::styled(
                format!(
                    "{} {}!",
                    self.translations.get_text("welcome_to"),
                    self.translations.get_text("snake_title")
                ),
                Style::default().fg(Color::Yellow),
            )]),
            Spans::from(""),
            Spans::from(self.translations.get_text("how_to_play")),
            Spans::from(""),
            Spans::from(self.translations.get_text("move_snake")),
            Spans::from(self.translations.get_text("eat_food_title")),
            Spans::from(self.translations.get_text("apple_desc")),
            Spans::from(self.translations.get_text("candy_desc")),
            Spans::from(self.translations.get_text("avoid_walls")),
            Spans::from(""),
            Spans::from(self.translations.get_text("press_enter")),
            Spans::from(self.translations.get_text("pause_game")),
        ];

        let paragraph = Paragraph::new(welcome_text)
            .block(Block::default().borders(Borders::ALL).title(Span::styled(
                self.translations.get_text("snake_title"),
                Style::default().fg(Color::Yellow),
            )))
            .alignment(tui::layout::Alignment::Center);
        f.render_widget(paragraph, area);
    }

    fn render_game(&self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let mut display_board = vec![vec![false; 20]; 20];

        // 绘制蛇身
        for pos in &self.snake_body {
            if pos.x >= 0 && pos.x < 20 && pos.y >= 0 && pos.y < 20 {
                display_board[pos.y as usize][pos.x as usize] = true;
            }
        }

        let mut text = vec![];
        text.push(Spans::from(""));

        // 绘制游戏区域
        for y in 0..20 {
            let mut line = String::new();
            for x in 0..20 {
                let current_pos = Position {
                    x: x as i32,
                    y: y as i32,
                };
                if self.food.positions.contains(&current_pos) {
                    match self.food.food_type {
                        FoodType::Apple => line.push_str("🍎"), // 苹果
                        FoodType::Candy => line.push_str("🍬"), // 糖果
                    }
                } else if display_board[y][x] {
                    line.push_str("██"); // 蛇身
                } else {
                    line.push_str("··"); // 空白
                }
            }
            text.push(Spans::from(line));
        }

        text.push(Spans::from(""));
        text.push(Spans::from(format!(
            "{}: {}",
            self.translations.get_text("score"),
            self.score
        )));

        if self.game_over {
            text.push(Spans::from(self.translations.get_text("game_over")));
            text.push(Spans::from(self.translations.get_text("press_r_restart")));
        }

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title(Span::styled(
                self.translations.get_text("snake_title"),
                Style::default().fg(Color::Green),
            )))
            .alignment(tui::layout::Alignment::Center);

        f.render_widget(paragraph, area);
    }

    fn render_pause(&self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        if self.game_state == GameState::Paused {
            self.compiling.borrow_mut().render(f, area);
        }
    }

    pub fn set_compile_language(&mut self, lang: CompileLanguage) {
        self.compiling.borrow_mut().set_language(lang);
    }
}

impl Game for Snake {
    fn new() -> Self {
        Snake::new()
    }

    fn handle_input(&mut self, key: KeyCode) {
        self.handle_input(key);
    }

    fn update(&mut self) {
        self.update();
    }

    fn render(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        Snake::render(self, f, area);
    }

    fn set_language(&mut self, language: Language) {
        Snake::set_language(self, language);
    }

    fn set_compile_language(&mut self, lang: CompileLanguage) {
        Snake::set_compile_language(self, lang);
    }
}
