use std::collections::VecDeque;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::game_manager::CompileLanguage;

pub struct Compiling {
    messages: VecDeque<String>,
    tick_count: u32,
    display_messages: Vec<String>,
    current_language: CompileLanguage,
}

impl Compiling {
    pub fn new() -> Self {
        let mut compiling = Self {
            messages: VecDeque::new(),
            tick_count: 0,
            display_messages: Vec::new(),
            current_language: CompileLanguage::Rust,
        };
        compiling.set_language(CompileLanguage::Rust);
        compiling
    }

    pub fn set_language(&mut self, lang: CompileLanguage) {
        self.current_language = lang.clone();
        self.messages.clear();

        let compile_messages = match lang {
            CompileLanguage::Rust => vec![
                "Compiling libc v0.2.169",
                "Compiling proc-macro2 v1.0.93",
                "Compiling unicode-ident v1.0.16",
                "Compiling autocfg v1.4.0",
                "Compiling parking_lot_core v0.8.6",
                "Compiling signal-hook v0.3.17",
                "Compiling lock_api v0.4.12",
                "Compiling signal-hook-registry v1.4.2",
                "Compiling getrandom v0.2.15",
                "Compiling mio v0.7.14",
                "Compiling rand_core v0.6.4",
                "Compiling signal-hook-mio v0.2.4",
                "Compiling parking_lot v0.11.2",
                "Compiling quote v1.0.38",
                "Compiling syn v2.0.98",
                "Compiling crossterm v0.22.1",
                "Compiling tui v0.17.0",
            ],
            CompileLanguage::Go => vec![
                "go: downloading github.com/gin-gonic/gin v1.9.1",
                "go: downloading github.com/json-iterator/go v1.1.12",
                "go: downloading golang.org/x/crypto v0.14.0",
                "go: downloading github.com/golang/protobuf v1.5.0",
                "go: downloading google.golang.org/protobuf v1.31.0",
                "go: downloading github.com/modern-go/concurrent v0.0.0-20180306012644-bacd9c7ef1dd",
                "go: downloading github.com/modern-go/reflect2 v1.0.2",
                "go: downloading gopkg.in/yaml.v3 v3.0.1",
                "go: downloading github.com/go-playground/validator/v10 v10.14.0",
                "go: downloading github.com/bytedance/sonic v1.9.1",
                "go: downloading github.com/gabriel-vasile/mimetype v1.4.2",
                "go: downloading golang.org/x/arch v0.3.0",
                "go: downloading golang.org/x/net v0.17.0",
                "go: downloading golang.org/x/sys v0.13.0",
                "go: downloading golang.org/x/text v0.13.0",
            ],
            CompileLanguage::CMake => vec![
                "-- The C compiler identification is GNU 11.4.0",
                "-- The CXX compiler identification is GNU 11.4.0",
                "-- Detecting C compiler ABI info",
                "-- Detecting C compiler ABI info - done",
                "-- Check for working C compiler: /usr/bin/cc - skipped",
                "-- Detecting C compile features",
                "-- Detecting C compile features - done",
                "-- Detecting CXX compiler ABI info",
                "-- Detecting CXX compiler ABI info - done",
                "-- Check for working CXX compiler: /usr/bin/c++ - skipped",
                "-- Detecting CXX compile features",
                "-- Detecting CXX compile features - done",
                "-- Found OpenSSL: /usr/lib/x86_64-linux-gnu/libcrypto.so",
                "-- Looking for pthread.h",
                "-- Looking for pthread.h - found",
                "-- Found Threads: TRUE",
                "-- Configuring done",
                "-- Generating done",
                "-- Build files have been written to: /build",
            ],
        };

        self.messages.extend(compile_messages.iter().map(|&s| s.to_string()));
    }

    pub fn update(&mut self) {
        self.tick_count += 1;
        
        // 每50ms (假设update每10ms调用一次) 更新一次消息
        if self.tick_count % 5 == 0 && !self.messages.is_empty() {
            let first_msg = self.messages.pop_front().unwrap();
            self.messages.push_back(first_msg);
        }
    }

    pub fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        // 根据区域高度确定显示的消息数量
        let visible_lines = (area.height as usize).saturating_sub(2);
        self.display_messages = self.messages
            .iter()
            .take(visible_lines)
            .map(|msg| msg.to_string())
            .collect();

        let spans: Vec<Spans> = self.display_messages
            .iter()
            .map(|msg| {
                let mut spans = vec![];
                
                match self.current_language {
                    CompileLanguage::Rust => {
                        if msg.starts_with("Compiling") {
                            spans.push(Span::styled(
                                "Compiling",
                                Style::default().fg(Color::Green),
                            ));
                            spans.push(Span::raw(" "));
                            spans.push(Span::raw(msg.strip_prefix("Compiling ").unwrap_or(msg)));
                        } else {
                            spans.push(Span::raw(msg));
                        }
                    }
                    CompileLanguage::Go => {
                        if msg.starts_with("go:") {
                            spans.push(Span::styled(
                                "go:",
                                Style::default().fg(Color::Cyan),
                            ));
                            spans.push(Span::raw(" "));
                            spans.push(Span::raw(msg.strip_prefix("go: ").unwrap_or(msg)));
                        } else {
                            spans.push(Span::raw(msg));
                        }
                    }
                    CompileLanguage::CMake => {
                        if msg.starts_with("--") {
                            spans.push(Span::styled(
                                "--",
                                Style::default().fg(Color::Yellow),
                            ));
                            spans.push(Span::raw(" "));
                            spans.push(Span::raw(msg.strip_prefix("-- ").unwrap_or(msg)));
                        } else {
                            spans.push(Span::raw(msg));
                        }
                    }
                }
                
                Spans::from(spans)
            })
            .collect();

        let title = match self.current_language {
            CompileLanguage::Rust => "Compiling Rust Project",
            CompileLanguage::Go => "Building Go Project",
            CompileLanguage::CMake => "Configuring CMake Project",
        };

        let paragraph = Paragraph::new(spans)
            .block(Block::default()
                .borders(Borders::ALL)
                .title(title));

        f.render_widget(paragraph, area);
    }
}
