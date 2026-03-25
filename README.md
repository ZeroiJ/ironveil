# Ironveil

A terminal-based roguelike dungeon crawler written in Rust.

## Features

- **3 Character Classes**: Warrior, Rogue, Mage - each with unique abilities
- **Procedural Generation**: Every floor is randomly generated with rooms and tunnels
- **18+ Monster Types**: Undead, demons, beasts, elementals, and more
- **Inventory System**: Weapons, armor, rings, and potions
- **Progressive Floor Reveal**: Watch the dungeon expand from your position
- **Save/Load**: Ctrl+S to save, Ctrl+L to load your progress

## Controls

| Key | Action |
|-----|--------|
| Arrow Keys / WASD | Move / Attack |
| 1 | Use ability 1 |
| 2 | Use ability 2 |
| 3 | Use ability 3 |
| Space | Wait a turn |
| Tab | Open inventory |
| > | Descend stairs |
| Ctrl+S | Save game |
| Ctrl+L | Load game |
| q / Esc | Quit |

## How to Play

```bash
# Run the game
cd ironveil
cargo run
```

## Gameplay Tips

- Watch your HP - enemies can deal significant damage
- Use abilities strategically - they have cooldowns
- Explore every room before descending - there may be valuable items
- Some floors have bosses - defeat them to proceed
- Check your coordinates in the UI to navigate

## Building

```bash
# Development build
cargo build

# Release build (faster)
cargo build --release
```

## Documentation

- [Classes](docs/CLASSES.md) - Character class details
- [Monsters](docs/MONSTERS.md) - Enemy encyclopedia
- [Items](docs/ITEMS.md) - Item database
- [Controls](docs/CONTROLS.md) - Full control reference
- [Biomes](docs/BIOMES.md) - Floor theming
- [Changelog](CHANGELOG.md) - Version history

## Tech Stack

- **Language**: Rust
- **Terminal**: crossterm
- **Random**: rand
- **Serialization**: serde + serde_json

## License

MIT
