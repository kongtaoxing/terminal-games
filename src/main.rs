use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use goldminer::Game;

fn main() -> Result<(), Box<dyn Error>> {
    // 设置终端
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 运行游戏
    let mut game = Game::new();
    let res = run_game(&mut terminal, &mut game);

    // 清理终端设置
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_game<B: Backend>(terminal: &mut Terminal<B>, game: &mut Game) -> io::Result<()> {
    loop {
        terminal.draw(|f| game.render(f, f.size()))?;

        game.update();

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    key_code => game.handle_input(key_code),
                }
            }
        }
    }
}
