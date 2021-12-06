use crate::DungeonBuilder;
use dungeon_models::{Dungeon, Field};
use rand::Rng;

pub struct BSPGenerator {
    pub size_x: usize,
    pub size_y: usize,
    pub split_size: usize,
    pub wall_size: usize,
}

#[derive(Clone, Copy)]
struct Boundaries {
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
}

impl DungeonBuilder for BSPGenerator {
    fn build(self) -> Dungeon {
        let mut d = Dungeon::from((self.size_x, self.size_y));
        process(
            &mut d,
            Boundaries {
                x0: 0,
                y0: 0,
                x1: self.size_x,
                y1: self.size_y,
            },
            self.split_size,
            self.wall_size,
        );
        d
    }
}

fn can_split_x(b: Boundaries, split_size: usize) -> bool {
    b.x1 - b.x0 > split_size
}

fn can_split_y(b: Boundaries, split_size: usize) -> bool {
    b.y1 - b.y0 > split_size
}

fn make_hall(map: &mut Dungeon, b: Boundaries, wall_size: usize) {
    for x in (b.x0 + wall_size)..(b.x1 - wall_size) {
        for y in (b.y0 + wall_size)..(b.y1 - wall_size) {
            map.map[x][y] = Field::Floor
        }
    }
}

fn split_by_x(map: &mut Dungeon, b: Boundaries, split_size: usize, wall_size: usize) {
    let split = rand::thread_rng().gen_range((b.x0 + split_size / 2)..(b.x1 - split_size / 2));
    process(
        map,
        Boundaries {
            x0: b.x0,
            x1: split,
            y0: b.y0,
            y1: b.y1,
        },
        split_size,
        wall_size,
    );
    process(
        map,
        Boundaries {
            x0: split,
            x1: b.x1,
            y0: b.y0,
            y1: b.y1,
        },
        split_size,
        wall_size,
    );
    let pass_y = rand::thread_rng().gen_range((b.y0 + wall_size)..(b.y1 - wall_size));
    let mut x = split;
    while let Field::Wall = map.map[x][pass_y] {
        map.map[x][pass_y] = Field::Floor;
        x -= 1;
    }
    let mut x = split + 1;
    while let Field::Wall = map.map[x][pass_y] {
        map.map[x][pass_y] = Field::Floor;
        x += 1;
    }
}

fn split_by_y(map: &mut Dungeon, b: Boundaries, split_size: usize, wall_size: usize) {
    let split = rand::thread_rng().gen_range((b.y0 + split_size / 2)..(b.y1 - split_size / 2));
    process(
        map,
        Boundaries {
            x0: b.x0,
            x1: b.x1,
            y0: b.y0,
            y1: split,
        },
        split_size,
        wall_size,
    );
    process(
        map,
        Boundaries {
            x0: b.x0,
            x1: b.x1,
            y0: split,
            y1: b.y1,
        },
        split_size,
        wall_size,
    );
    let pass_x = rand::thread_rng().gen_range((b.x0 + wall_size)..(b.x1 - wall_size));
    let mut y = split;
    while let Field::Wall = map.map[pass_x][y] {
        map.map[pass_x][y] = Field::Floor;
        y -= 1;
    }
    let mut y = split + 1;
    while let Field::Wall = map.map[pass_x][y] {
        map.map[pass_x][y] = Field::Floor;
        y += 1;
    }
}

fn process(map: &mut Dungeon, b: Boundaries, split_size: usize, wall_size: usize) {
    let split_x = can_split_x(b, split_size);
    let split_y = can_split_y(b, split_size);
    match (split_x, split_y) {
        (false, false) => make_hall(map, b, wall_size),
        (true, false) => split_by_x(map, b, split_size, wall_size),
        (false, true) => split_by_y(map, b, split_size, wall_size),
        (true, true) => {
            if rand::thread_rng().gen::<bool>() {
                split_by_x(map, b, split_size, wall_size)
            } else {
                split_by_y(map, b, split_size, wall_size)
            }
        }
    }
}
