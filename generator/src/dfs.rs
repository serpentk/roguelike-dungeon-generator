use crate::DungeonBuilder;
use dungeon_models::{Dungeon, Field};
use rand::seq::SliceRandom;
use rand::Rng;
use std::cmp::min;

pub struct DFSGenerator {
    pub size_x: usize,
    pub size_y: usize,
}

#[derive(Clone, Copy)]
struct Node {
    x: usize,
    y: usize,
}

impl DungeonBuilder for DFSGenerator {
    fn build(self) -> Dungeon {
        let mut d = Dungeon::from((self.size_x * 3, self.size_y * 3));
        let initial = Node {
            x: rand::thread_rng().gen_range(0..self.size_x),
            y: rand::thread_rng().gen_range(0..self.size_y),
        };
        process(&mut d, initial);
        d
    }
}

fn process(map: &mut Dungeon, node: Node) {
    map.map[3 * node.x + 1][3 * node.y + 1] = Field::Floor;
    for n in neighbors(node, map.size_x / 3, map.size_y / 3) {
        if !is_visited(map, n) {
            if n.x == node.x {
                let min_y = min(n.y, node.y);
                map.map[3 * n.x + 1][min_y * 3 + 2] = Field::Floor;
                map.map[3 * n.x + 1][min_y * 3 + 3] = Field::Floor;
            } else {
                let min_x = min(n.x, node.x);
                map.map[min_x * 3 + 2][3 * n.y + 1] = Field::Floor;
                map.map[min_x * 3 + 3][3 * n.y + 1] = Field::Floor;
            }
            process(map, n);
        }
    }
}

fn neighbors(node: Node, x_size: usize, y_size: usize) -> Vec<Node> {
    let mut res: Vec<Node> = vec![];
    if node.x > 0 {
        res.push(Node {
            x: node.x - 1,
            y: node.y,
        });
    }
    if node.y > 0 {
        res.push(Node {
            x: node.x,
            y: node.y - 1,
        });
    }
    if node.x < x_size - 1 {
        res.push(Node {
            x: node.x + 1,
            y: node.y,
        });
    }
    if node.y < y_size - 1 {
        res.push(Node {
            x: node.x,
            y: node.y + 1,
        });
    }
    res.shuffle(&mut rand::thread_rng());
    res
}

fn is_visited(map: &Dungeon, node: Node) -> bool {
    !matches!(map.map[3 * node.x + 1][3 * node.y + 1], Field::Wall)
}
