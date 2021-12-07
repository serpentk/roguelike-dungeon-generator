/// Just adds randomly Enter and Exit fields
use crate::NarrativeBuilder;
use dungeon_models::{Dungeon, Field};
use itertools::Itertools;
use rand::prelude::SliceRandom;

pub struct DummyNarrativeBuilder {}

impl NarrativeBuilder for DummyNarrativeBuilder {
    fn fill(self, d: &mut Dungeon) {
        let free_space = (0..d.size_x)
            .cartesian_product(0..d.size_y)
            .filter(|(x, y)| matches!(d.map[*x][*y], Field::Floor))
            .collect::<Vec<(usize, usize)>>();
        let mut rng = &mut rand::thread_rng();
        let choice = free_space
            .choose_multiple(&mut rng, 2)
            .cloned()
            .collect::<Vec<(usize, usize)>>();
        d.map[choice[0].0][choice[0].1] = Field::Enter;
        d.map[choice[1].0][choice[1].1] = Field::Exit;
    }
}
