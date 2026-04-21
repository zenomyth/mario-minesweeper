# Mario Minesweeper

A classic Minesweeper game with a Mario theme, built in Rust using the egui GUI framework. This modern take on the iconic puzzle game features multiple difficulty levels, a dark theme, and an interactive desktop experience.

## Features

- **Three Difficulty Levels**
  - Easy: 9×9 grid with 10 mines
  - Medium: 16×16 grid with 40 mines
  - Hard: 30×16 grid with 99 mines

- **Classic Minesweeper Gameplay**
  - Left-click to reveal cells
  - Right-click to flag/unflag suspected mines
  - Auto-reveal adjacent cells when all neighboring mines are flagged
  - First-click safety: the first cell clicked is always safe

- **Interactive UI**
  - Dark theme with carefully chosen colors
  - Real-time timer to track game duration
  - Mine counter showing remaining flags
  - Game status display (Playing/Won/Lost)
  - Easy difficulty selection and restart buttons

- **Visual Feedback**
  - Numbered cells indicating adjacent mines (1-8)
  - Exploded mine highlight when a mine is hit
  - Flagged mines display
  - Hover effects on interactive elements

- **Mario Theme**
  - Custom Mario-themed application icon
  - Mario character expressions based on game state
  - Themed visuals and color palette

## Requirements

- Rust 1.70 or later
- Windows, macOS, or Linux

## Building the Project

### Prerequisites

Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

### Build for Development

```bash
cargo build
./target/debug/mario-minesweeper
```

### Build for Release

```bash
cargo build --release
./target/release/mario-minesweeper
```

The release build is optimized for performance and produces a smaller executable located in `target/release/`.

## How to Play

1. **Select Difficulty**: Choose between Easy, Medium, or Hard from the difficulty selector
2. **Click to Start**: Left-click any cell to start the game (this cell is guaranteed to be safe)
3. **Reveal Cells**: 
   - Left-click to reveal a cell
   - If it's empty or numbered, it shows how many mines are adjacent
   - If it's a mine, you lose!
4. **Flag Mines**: 
   - Right-click to place or remove a flag on a suspicious cell
   - The mine counter shows how many flags you've placed
5. **Win Condition**: Reveal all non-mine cells to win
6. **Advanced**: Once a numbered cell is revealed with all adjacent mines flagged, you can click it to auto-reveal safe neighbors

## Project Structure

```
mario-minesweeper/
├── src/
│   ├── main.rs          # Application entry point and UI logic
│   └── logic.rs         # Game logic and grid management
├── assets/
│   ├── application.ico  # Application icon
│   └── *.webp          # Mario character expressions
├── build.rs            # Build script for resource embedding
├── mario-minesweeper.rc # Windows resource configuration
├── Cargo.toml          # Project dependencies
└── README.md           # This file
```

## Dependencies

- **eframe** (0.31.0): Desktop GUI framework
- **egui** (via eframe): Immediate mode GUI library
- **egui_extras** (0.31.0): Additional UI components and image loaders
- **image** (0.25.0): Image format support (ICO, WebP)
- **chrono** (0.4.44): Date/time utilities
- **rand** (0.10.0): Random number generation for mine placement

Build dependencies:
- **embed-resource** (2.5.1): Embeds resources into Windows executables

## Platform Notes

### Windows
- The application runs as a native Windows window without a console
- The application icon is embedded in the executable using Windows resources

### macOS & Linux
- Runs with the egui native GUI
- Icon support depends on the platform's window manager

## Troubleshooting

### Icon not showing in Explorer
If the application icon doesn't appear in Windows Explorer:
1. Clear the Windows icon cache: `ie4uinit.exe -ClearIconCache`
2. Rebuild the project: `cargo build --release`

### Dependencies issues
If you encounter build issues:
1. Update Rust: `rustup update`
2. Clean the build cache: `cargo clean`
3. Rebuild: `cargo build --release`

## License

This project is open source and provided as-is for educational and entertainment purposes. 

**IMPORTANT DISCLAIMER:** This is a fan project and is not affiliated with, endorsed by, or associated with Nintendo. Mario, Luigi, and related characters are trademarks and intellectual property of Nintendo Co., Ltd. The Mario-themed assets in this project (icons and character expressions) were generated using AI icon generation tools and are subject to the terms of service of those respective platforms.

This project is provided for personal, non-commercial use only. Users are responsible for ensuring compliance with applicable laws and the terms of service of the AI generation platform used for the assets.

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests to improve the game.

## Credits

Built with Rust and the egui framework. 

- Mario and related assets are intellectual property of Nintendo Co., Ltd.
- Application icons and character expressions were generated using AI icon generation tools.

