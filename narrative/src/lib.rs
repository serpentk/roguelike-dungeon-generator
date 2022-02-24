use dungeon_models::Dungeon;

pub trait NarrativeBuilder {
    fn fill(self, _: &mut Dungeon);
}

pub mod dummy;
pub mod longpath;
