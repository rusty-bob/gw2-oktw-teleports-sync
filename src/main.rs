use tp_sync::{Teleport, TeleportGroup, TeleportManager};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example: Load the teleport.json file
    let mut manager = TeleportManager::load("teleport.json")?;

    println!("Loaded {} teleport groups", manager.group_count());

    // Example: Get all groups
    println!("\nAll teleport groups:");
    for group in manager.get_all_groups() {
        println!("  - {} ({} teleports)", group.name, group.teleports.len());
    }

    // Example: Get hash of a specific group
    if let Ok(hash) = manager.get_group_hash("Homestead") {
        println!("\nHash of 'Homestead' group: {}", hash);
    }

    // Example: Add a new group
    let new_group = TeleportGroup {
        name: "Test Group".to_string(),
        teleports: vec![Teleport {
            name: "Test Location".to_string(),
            coordinates: [100.0, 200.0, 300.0],
            map: 1234,
            tooltip: Some("This is a test".to_string()),
        }],
    };

    match manager.add_group(new_group) {
        Ok(_) => println!("\nSuccessfully added 'Test Group'"),
        Err(e) => println!("\nFailed to add group: {}", e),
    }

    println!("Now have {} teleport groups", manager.group_count());

    // Example: Remove the test group
    match manager.remove_group("Test Group") {
        Ok(removed) => println!("\nRemoved group: {}", removed.name),
        Err(e) => println!("\nFailed to remove group: {}", e),
    }

    println!("Back to {} teleport groups", manager.group_count());

    // Note: Uncomment to save changes
    // manager.save_to_original()?;

    Ok(())
}
