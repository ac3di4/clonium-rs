enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Copy, Clone, PartialEq)]
struct PlayerId(usize);

const CELL_SPEED: f32 = 0.02;
struct Cell {
    player_id: PlayerId,
    x: f32,
    y: f32,
    completion: f32,
    direction: Direction,
}

impl Cell {
    fn new(player_id: PlayerId, x: usize, y: usize, direction: Direction) -> Self {
        Cell {
            player_id,
            x: x as f32,
            y: y as f32,
            completion: 0.0,
            direction,
        }
    }

    fn is_completed(&self) -> bool {
        self.completion >= 1.0
    }

    fn step(&mut self) -> bool {
        if self.is_completed() {
            self.x = self.x.round();
            self.y = self.y.round();
            true
        } else {
            match self.direction {
                Direction::UP => self.y -= CELL_SPEED,
                Direction::RIGHT => self.x += CELL_SPEED,
                Direction::DOWN => self.y += CELL_SPEED,
                Direction::LEFT => self.x -= CELL_SPEED,
            }
            self.completion += CELL_SPEED;
            false
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Dots {
    ONE,
    TWO,
    THREE,
}

struct StaticCell {
    player_id: PlayerId,
    value: Dots,
}

impl StaticCell {
    fn new(player_id: PlayerId, value: Dots) -> Self {
        Self { player_id, value }
    }
}

const MAP_SIZE: usize = 5;
struct Grid([Option<StaticCell>; MAP_SIZE * MAP_SIZE]);

impl Grid {
    fn new() -> Self {
        Self(Default::default())
    }

    fn put(&mut self, player_id: PlayerId, x: usize, y: usize) -> Option<Vec<Cell>> {
        if let Some(cell) = self.get(x, y) {
            match cell.value {
                Dots::ONE => {
                    self.set(x, y, Some(StaticCell::new(player_id, Dots::TWO)));
                    None
                }
                Dots::TWO => {
                    self.set(x, y, Some(StaticCell::new(player_id, Dots::THREE)));
                    None
                }
                Dots::THREE => {
                    self.set(x, y, None);
                    let mut animation_list = Vec::with_capacity(4);
                    if y > 0 {
                        animation_list.push(Cell::new(player_id, x, y, Direction::UP));
                    }
                    if x < MAP_SIZE - 1 {
                        animation_list.push(Cell::new(player_id, x, y, Direction::RIGHT));
                    }
                    if y < MAP_SIZE - 1 {
                        animation_list.push(Cell::new(player_id, x, y, Direction::DOWN));
                    }
                    if x > 0 {
                        animation_list.push(Cell::new(player_id, x, y, Direction::LEFT));
                    }
                    Some(animation_list)
                }
            }
        } else {
            None
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&StaticCell> {
        self.0[x + y * MAP_SIZE].as_ref()
    }

    fn get_winner(&self) -> Option<PlayerId> {
        let mut i = 0;
        while i < self.0.len() && self.0[i].is_none() {
            i += 1;
        }
        if i >= self.0.len() {
            return None;
        }
        let winner_id = self.0[i].as_ref().unwrap().player_id;
        while i < self.0.len()
            && (self.0[i].is_none() || self.0[i].as_ref().unwrap().player_id == winner_id)
        {
            i += 1;
        }
        if i < self.0.len() {
            None
        } else {
            Some(winner_id)
        }
    }

    fn set(&mut self, x: usize, y: usize, value: Option<StaticCell>) {
        self.0[x + y * MAP_SIZE] = value;
    }
}

fn main() {
    println!("Hello, world!");
}
