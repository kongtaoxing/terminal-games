# Terminal Game Collection

A collection of classic terminal-based games implemented in Rust. Currently featuring Gold Miner and Tetris, this collection offers a nostalgic gaming experience right in your terminal!

## Available Games

1. [Gold Miner](./src/games/goldminer/README.md)
2. [Tetris](./src/games/tetris/README.md)
3. [Snake](./src/games/snake/README.md)
4. [2048](./src/games/twenty_forty_eight/README.md)

## Features

- **Terminal-based Interface**: Uses `crossterm` and `tui` for smooth terminal rendering
- **Easy Navigation**: Use arrow keys or number keys to select games
- **Simple Controls**: Intuitive controls for each game
- **Minimalist Design**: Clean and simple interface for a distraction-free gaming experience

## How to Play

1. Use **UP/DOWN** arrow keys to select a game
2. Press **ENTER** to start the selected game
3. Alternatively, press number keys (**1**, **2**, etc.) to directly select games
4. Press **Q** to quit
5. In any game, press **p** or **Esc** to pause the game and pretend to compile some projects, and then press **Enter** to continue the game
6. Press **C** to choose the language of the code for pretending to compile

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
