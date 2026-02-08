# Teleport Sync Manager

A TUI tool for managing Guild Wars 2 teleport configurations.

![Screenshot](image.png)

## What It Does

- Browse local and remote teleport groups
- Install teleport groups from GitHub repository
- Delete local teleport groups
- Sync status indicators (✓ synced, ↑ local only, ↓ remote only, ⟳ modified)

## Usage

1. Run `tp_sync.exe`
2. On first run, select your `teleport.json` file
3. Use arrow keys to navigate, Tab to switch panes
4. Press Space to install remote groups, Delete to remove local groups
5. Press Q to quit

## Config

Settings are stored in `config.toml` in the same folder as the executable.

