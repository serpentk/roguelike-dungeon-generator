use crate::DungeonBuilder;
use dungeon_models::{Dungeon, Field};
use rand::{thread_rng, Rng};

pub struct CellularAutomataGenerator {
    pub size_x: usize,
    pub size_y: usize,
    pub wall_probability: f64,
    pub iterations: i32,
}

impl Default for CellularAutomataGenerator {
    fn default() -> Self {
        Self {
            size_x: 50,
            size_y: 50,
            wall_probability: 0.45,
            iterations: 5,
        }
    }
}

impl DungeonBuilder for CellularAutomataGenerator {
    fn build(self) -> Dungeon {
        let mut d = Dungeon::from((self.size_x, self.size_y));
        let mut rng = thread_rng();
        for x in 0..self.size_x {
            for y in 0..self.size_y - 1 {
                d.map[x][y] = match rng.gen_bool(self.wall_probability) {
                    true => Field::Wall,
                    false => Field::Floor,
                };
            }
        }
        for _i in 0..self.iterations {
            step(&mut d);
        }
        d
    }
}

fn r_n(x: usize, y: usize, n: usize, size_x: usize, size_y: usize) -> Vec<(usize, usize)> {
    let x0 = x as i32;
    let y0 = y as i32;
    let n0 = n as i32;
    let mut res = vec![];
    for i in vec![(x0 - n0), (x0 + n0)].into_iter() {
        for j in (y0 - n0)..(y0 + n0 + 1) {
            res.push((i, j));
        }
    }
    for j in vec![(y0 - n0), (y0 + n0)].into_iter() {
        for i in (x0 - n0 + 1)..(x0 + n0) {
            res.push((i, j));
        }
    }
    res.into_iter()
        .filter(|(a, b)| *a >= 0 && *a < (size_x as i32) && *b >= 0 && *b < (size_y as i32))
        .map(|(a, b)| (a as usize, b as usize))
        .collect::<Vec<_>>()
}

fn step(d: &mut Dungeon) {
    let mut new_map: Vec<Vec<Field>> = vec![vec![Default::default(); d.size_x]; d.size_y];
    for x in 1..(d.size_x - 1) {
        for y in 1..(d.size_y - 1) {
            let r1 = r_n(x, y, 1, d.size_x, d.size_y)
                .into_iter()
                .fold(0, |acc, (i, j)| match d.map[i][j] {
                    Field::Wall => acc + 1,
                    _ => acc,
                });
            let r2 = r_n(x, y, 2, d.size_x, d.size_y)
                .into_iter()
                .fold(0, |acc, (i, j)| match d.map[i][j] {
                    Field::Wall => acc + 1,
                    _ => acc,
                });
            if r1 >= 5 || r2 < 2 {
                new_map[x][y] = Field::Wall;
            } else {
                new_map[x][y] = Field::Floor;
            }
        }
    }
    for x in 0..d.size_x {
        for y in 0..d.size_y {
            d.map[x][y] = new_map[x][y];
        }
    }
}
