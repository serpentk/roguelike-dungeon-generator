/// Adds randomly Enter field; exit will be far away
use crate::NarrativeBuilder;
use dungeon_models::{is_neighbor, Dungeon, Field};
use itertools::Itertools;
use rand::seq::SliceRandom;
use std::collections::HashSet;

pub struct LongPathNarrativeBuilder {}

impl NarrativeBuilder for LongPathNarrativeBuilder {
    fn fill(self, d: &mut Dungeon) {
        let free_space = (0..d.size_x)
            .cartesian_product(0..d.size_y)
            .filter(|(x, y)| matches!(d.map[*x][*y], Field::Floor))
            .collect::<Vec<(usize, usize)>>();
        let mut rng = &mut rand::thread_rng();
        let choice = free_space.choose(&mut rng).unwrap();
        d.map[choice.0][choice.1] = Field::Enter;

        let mut passed: HashSet<(usize, usize)> = HashSet::new();
        let mut current: Vec<(usize, usize)> = vec![*choice];
        let mut last: Vec<(usize, usize)> = vec![];
        while !current.is_empty() {
            last = current.clone();
            passed.extend(current);
            current = free_space
                .clone()
                .into_iter()
                .filter(|&(x, y)| {
                    passed.get(&(x, y)).is_none()
                        && last.iter().any(|(a, b)| is_neighbor((x, y), (*a, *b)))
                })
                .collect();
        }
        let choice = last.choose(&mut rng).unwrap();
        d.map[choice.0][choice.1] = Field::Exit;
    }
}
