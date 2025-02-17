
# Gold Miner

A simple terminal-based game inspired by the classic "Gold Miner" game, implemented in Rust. Control a hook to collect gold and avoid stones in this minimalist terminal adventure!

## Features

- **Terminal Rendering**: Uses `crossterm` and `tui` for terminal input/output and rendering.
- **Simple Controls**: Move the hook left and right using arrow keys.
- **Score System**: Collect gold to earn points and avoid stones to prevent losing points.
- **Minimalist Design**: A clean and simple interface for a nostalgic gaming experience.

## How to Play

1. The hook automatically swings left and right.
2. Press **space** to release the hook and collect items.
   - Gold ($) is worth 100 points
   - Stones (O) will deduct 50 points
3. When all items are collected, new ones will be generated.
4. Press `q` to quit the game.
