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
- Delete a folder from history with 'd'
- Navigate using Up/Down arrow keys
- Press Enter to select a folder
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

```