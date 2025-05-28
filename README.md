# Tracker

A terminal-based episode tracker for TV shows, built with Rust. This program helps you keep track of which episodes you've watched in a simple and efficient way.

## Features

- Terminal-based user interface using TUI (Text User Interface)
- Track watched and unwatched episodes
- Maintains watch history even after episodes are deleted
- Simple keyboard navigation
- Persistent storage of watch status in a JSON file
- Saves a history of folders you've used the tracker with
- Interactive folder selection menu

## Installation

### Using Cargo (Recommended)

```bash
cargo install --path .
```

This will install the binary in `~/.cargo/bin/`. Make sure this directory is in your PATH.

### Manual Installation

```bash
cargo build --release
cp target/release/tracker ~/.local/bin/
```

## Uninstallation

### If installed with Cargo

```bash
cargo uninstall tracker
```

### If installed manually

```bash
rm ~/.local/bin/tracker
```

Note: If you want to remove the watch history, you'll need to manually delete the `watched.json` files from your episode folders.

## Updating

### If installed with Cargo

1. Navigate to the project directory:
```bash
cd /path/to/tracker
```

2. Pull the latest changes (if using git):
```bash
git pull
```

3. Rebuild and reinstall:
```bash
cargo install --path .
```

### If installed manually

1. Navigate to the project directory:
```bash
cd /path/to/tracker
```

2. Pull the latest changes (if using git):
```bash
git pull
```

3. Rebuild and copy the new binary:
```bash
cargo build --release
cp target/release/tracker ~/.local/bin/
```

## Usage

### Interactive Mode

Simply run:
```bash
tracker
```

This will show a menu where you can:
- Select from previously used folders
- Add a new folder to track
- Navigate using Up/Down arrow keys
- Press Enter to select a folder
- Press 'n' to add a new folder
- Press 'q' to quit

### Direct Mode

You can also directly specify a folder:
```bash
tracker <folder_path>
```

### Controls in Episode Tracker

- **Up/Down Arrow Keys**: Navigate through episodes
- **Enter/Space**: Toggle watched status
- **q**: Quit the program

### Example

```bash
# Interactive mode
tracker

# Direct mode
tracker ~/Videos/Tom\ and\ Jerry/
```

## How It Works

The program creates a `watched.json` file in the specified folder to store the watch status of episodes. This file is automatically managed by the program and should not be edited manually.

### File Structure

- `watched.json`: Stores the watch status of episodes in each folder
- `~/.config/tracker/config.json`: Stores the history of folders you've used
- Episode files: Your video files in the specified folder

## Building from Source

1. Clone the repository:
```bash
git clone https://github.com/yourusername/tracker.git
cd tracker
```

2. Build the project:
```bash
cargo build --release
```

## Dependencies

- [ratatui](https://github.com/ratatui-org/ratatui): Terminal UI library
- [crossterm](https://github.com/crossterm-rs/crossterm): Terminal manipulation library
- [serde](https://github.com/serde-rs/serde): Serialization framework
- [serde_json](https://github.com/serde-rs/json): JSON support for Serde

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Feel free to submit a Pull Request. 