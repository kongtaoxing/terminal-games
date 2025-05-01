# Minesweeper

A terminal-based Minesweeper game implementation with TUI (Terminal User Interface).

## Features

- 10x10 game board with 15 mines
- Full keyboard control support
- Dual language support (English/Chinese)
- Color-coded interface
- Two ways to win:
  - Reveal all safe cells
  - Correctly flag all mines
- Auto-reveal for empty cells

## Controls

- Arrow keys / WASD: Move cursor
- Space: Reveal cell
- F: Flag/unflag potential mine
- P/ESC: Pause game
- R: Restart (when game is over)
- Enter: Start game (from welcome screen)

## Game Elements

- ■: Unrevealed cell
- 🚩: Flagged cell
- 💣: Mine
- Numbers (1-8): Number of adjacent mines
- Yellow highlight: Current cursor position
- Red highlight: Exploded mine

## Implementation Details

- Built with Rust using tui-rs and crossterm
- Uses RefCell for pause screen compilation animation
- Implements flood-fill algorithm for revealing empty cells
- Supports both keyboard and mouse input
