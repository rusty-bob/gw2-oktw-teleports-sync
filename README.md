# tp_sync - Teleport JSON Manager

A Rust library for managing teleport.json file structures with support for adding, removing, and querying teleport groups.

## Features

- ✅ Load and save teleport.json files
- ✅ Add new teleport groups
- ✅ Remove teleport groups
- ✅ Get hash of teleport group items
- ✅ Return all teleport groups
- ✅ Type-safe data structures with serde
- ✅ Comprehensive error handling

## Usage

### Basic Example

```rust
use tp_sync::{TeleportManager, TeleportGroup, Teleport};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load existing teleport.json
    let mut manager = TeleportManager::load("teleport.json")?;
    
    // Get all groups
    for group in manager.get_all_groups() {
        println!("{}: {} teleports", group.name, group.teleports.len());
    }
    
    // Add a new group
    let new_group = TeleportGroup {
        name: "My Custom Group".to_string(),
        teleports: vec![
            Teleport {
                name: "Custom Location".to_string(),
                coordinates: [100.0, 200.0, 300.0],
                map: 1234,
                tooltip: Some("A custom teleport".to_string()),
            }
        ],
    };
    manager.add_group(new_group)?;
    
    // Get hash of a group
    let hash = manager.get_group_hash("My Custom Group")?;
    println!("Group hash: {}", hash);
    
    // Remove a group
    let removed = manager.remove_group("My Custom Group")?;
    println!("Removed: {}", removed.name);
    
    // Save changes
    manager.save("teleport.json")?;
    
    Ok(())
}
```

## API Reference

### TeleportManager

The main interface for managing teleport configurations.

#### Creating a Manager

```rust
// Create with default configuration
let manager = TeleportManager::new();

// Load from file
let manager = TeleportManager::load("teleport.json")?;

// Create from existing config
let manager = TeleportManager::from_config(config);
```

#### Core Methods

- `add_group(&mut self, group: TeleportGroup) -> Result<()>` - Add a new teleport group
- `remove_group(&mut self, name: &str) -> Result<TeleportGroup>` - Remove a group by name
- `get_group_hash(&self, name: &str) -> Result<u64>` - Get hash of a group
- `get_all_groups(&self) -> &[TeleportGroup]` - Get all teleport groups
- `get_group(&self, name: &str) -> Option<&TeleportGroup>` - Get a specific group
- `group_count(&self) -> usize` - Get the number of groups

#### File Operations

- `load<P: AsRef<Path>>(path: P) -> Result<Self>` - Load from file
- `save<P: AsRef<Path>>(&self, path: P) -> Result<()>` - Save to file
- `save_to_original(&self) -> Result<()>` - Save to the originally loaded file

### Data Structures

#### TeleportGroup

```rust
pub struct TeleportGroup {
    pub name: String,
    pub teleports: Vec<Teleport>,
}
```

#### Teleport

```rust
pub struct Teleport {
    pub name: String,
    pub coordinates: [f32; 3],
    pub map: u32,
    pub tooltip: Option<String>,
}
```

## Error Handling

The library uses a custom `TeleportError` type:

- `IoError` - File I/O errors
- `JsonError` - JSON parsing/serialization errors
- `GroupNotFound` - Requested group doesn't exist
- `GroupAlreadyExists` - Attempting to add a duplicate group

## License

This project is open source.

