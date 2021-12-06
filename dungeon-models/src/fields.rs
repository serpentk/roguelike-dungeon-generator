#[derive(Copy, Clone)]
pub enum DoorState {
    Open,
    Closed,
}

impl Default for DoorState {
    fn default() -> Self {
        DoorState::Closed
    }
}

#[derive(Copy, Clone)]
pub enum Field {
    Wall,
    Door { key: i32, state: DoorState },
    Enter,
    Exit,
    Floor,
}

impl Default for Field {
    fn default() -> Self {
        Field::Wall
    }
}

pub struct Dungeon {
    pub size_x: usize,
    pub size_y: usize,
    pub map: Vec<Vec<Field>>,
}

impl From<(usize, usize)> for Dungeon {
    fn from(size: (usize, usize)) -> Self {
        Self {
            size_x: size.0,
            size_y: size.1,
            map: vec![vec![Default::default(); size.0]; size.1],
        }
    }
}
