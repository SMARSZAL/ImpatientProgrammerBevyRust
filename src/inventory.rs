use bevy::prelude::*;
use std::collections::HashMap;
use std::fmt;

/// Default pickup radius in world units before the player collects the item.
pub const DEFAULT_PICKUP_RADIUS: f32 = 40.0;

/// Items that can be stored in the player's inventory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemKind {
    TreeStump2,
    Plant1,
    Plant2,
    Plant3,
    Plant4,
}

impl ItemKind {
    pub fn display_name(&self) -> &'static str {
        match self {
            ItemKind::TreeStump2 => "tree_stump_2",
            ItemKind::Plant1 => "plant_1",
            ItemKind::Plant2 => "plant_2",
            ItemKind::Plant3 => "plant_3",
            ItemKind::Plant4 => "plant_4",
        }
    }
}

impl fmt::Display for ItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.display_name())
    }
}

/// Component tagging an entity as a pickable item.
#[derive(Component, Debug)]
pub struct Pickable {
    pub kind: ItemKind,
    pub radius: f32,
}

impl Pickable {
    pub fn new(kind: ItemKind) -> Self {
        Self {
            kind,
            radius: DEFAULT_PICKUP_RADIUS,
        }
    }
}

/// Resource storing the collected items.
#[derive(Resource, Default, Debug)]
pub struct Inventory {
    items: HashMap<ItemKind, u32>,
}

impl Inventory {
    pub fn add(&mut self, kind: ItemKind) -> u32 {
        let entry = self.items.entry(kind).or_insert(0);
        *entry += 1;
        *entry
    }

    pub fn summary(&self) -> String {
        if self.items.is_empty() {
            return "empty".to_string();
        }

        let mut parts: Vec<String> = self
            .items
            .iter()
            .map(|(kind, count)| format!("{}: {}", kind, count))
            .collect();
        parts.sort();
        parts.join(", ")
    }
}
