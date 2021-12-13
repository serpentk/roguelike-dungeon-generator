use dungeon_models::Dungeon;

pub trait DungeonBuilder {
    fn build(self) -> Dungeon;
}

pub mod bsp;
pub mod cellular;
pub mod dfs;
