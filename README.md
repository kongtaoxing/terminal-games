# Terminal Game Collection

A collection of classic terminal-based games implemented in Rust. Currently featuring Gold Miner and Tetris, this collection offers a nostalgic gaming experience right in your terminal!

## Available Games

### 1. Gold Miner
- Control a hook to collect gold and avoid stones
- Earn points by collecting gold pieces
- Avoid stones that deduct points
- Simple yet addictive gameplay

### 2. Tetris
- Classic block-stacking puzzle game
- Seven different block shapes
- Score points by clearing lines
- Progressive difficulty
- Scoring system:
  - 1 line: 100 points
  - 2 lines: 300 points
  - 3 lines: 500 points
  - 4 lines: 800 points

## Features

- **Terminal-based Interface**: Uses `crossterm` and `tui` for smooth terminal rendering
- **Easy Navigation**: Use arrow keys or number keys to select games
- **Simple Controls**: Intuitive controls for each game
- **Minimalist Design**: Clean and simple interface for a distraction-free gaming experience

## How to Play

### Main Menu
1. Use **UP/DOWN** arrow keys to select a game
2. Press **ENTER** to start the selected game
3. Alternatively, press number keys (**1**, **2**) to directly select games
4. Press **Q** to quit

### Gold Miner
1. The hook automatically swings left and right
2. Press **SPACE** to release the hook and collect items
   - Gold ($) is worth 100 points
   - Stones (O) will deduct 50 points
3. When all items are collected, new ones will be generated
4. Press **Q** to return to main menu

### Tetris
1. Use **LEFT/RIGHT** arrow keys to move blocks horizontally
2. Use **UP** arrow key to rotate blocks
3. Use **DOWN** arrow key to speed up descent
4. Press **SPACE** for hard drop
5. Clear lines to score points
6. Press **Q** to return to main menu

## Installation

1. Ensure you have Rust and Cargo installed
2. Clone this repository:
   ```bash
   git clone https://github.com/kongtaoxing/terminal-games.git
   ```
3. Navigate to the project directory:
   ```bash
   cd terminal-games
   ```
4. Run the game collection:
   ```bash
   cargo run
   ```

## Dependencies

- `crossterm`: Terminal input/output handling
- `tui`: Terminal user interface framework
- `rand`: Random number generation

## Future Improvements

- Add more classic games to the collection
- Add high score system
- Include sound effects
- Add game settings and difficulty levels
- Implement save/load game states

## Contributing

Contributions are welcome! Feel free to:
- Add new games
- Improve existing games
- Fix bugs
- Enhance the user interface
- Suggest new features

Please feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.
