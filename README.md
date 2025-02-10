
# Terminal Gold Miner

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

## Installation

1. Ensure you have Rust and Cargo installed.
2. Clone this repository:
   ```bash
   git clone https://github.com/kongtaoxing/goldminer.git
   ```
3. Navigate to the project directory:
   ```bash
   cd terminal-gold-miner
   ```
4. Run the game:
   ```bash
   cargo run
   ```

## Dependencies

- `crossterm`: For terminal input/output handling.
- `tui`: For building the terminal user interface.

## Future Improvements

- Add more gold and stone spawns.
- Introduce difficulty levels.
- Add animations and sound effects.
- Implement a high-score system.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.
