use crate::{
    render::CellRenderer,
    rule::Rule,
    utilities::{default_noise, get_centre, wrap},
};
use bevy::{math::IVec3, tasks::TaskPool};

#[derive(Clone, Copy, Debug)]
pub struct Position {
    x: usize,
    y: usize,
    z: usize,
}

impl Position {
    fn new(x: i32, y: i32, z: i32) -> Position {
        Position {
            x: x as usize,
            y: y as usize,
            z: z as usize,
        }
    }

    fn from_vec(pos: IVec3) -> Position {
        Position {
            x: pos.x as usize,
            y: pos.y as usize,
            z: pos.z as usize,
        }
    }
}

#[derive(Clone, Copy)]
pub struct SimpleCell {
    state: u8,
    neighbours: u8,
}

impl SimpleCell {
    fn new() -> SimpleCell {
        SimpleCell {
            state: 0,
            neighbours: 0,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.state == 0
    }
}

#[derive(Clone)]
pub struct MultiDimensional {
    cells: Vec<Vec<Vec<SimpleCell>>>,
    bounds: i32,
}

impl MultiDimensional {
    pub fn new() -> Self {
        MultiDimensional {
            cells: vec![vec![vec![]]],
            bounds: 0,
        }
    }

    // Return the current bounds
    pub fn get_bounds(&self) -> i32 {
        self.bounds as i32
    }

    // Get the total number of live cells
    pub fn get_count(&self) -> usize {
        let mut total = 0;
        // Loop through each cell
        for x in 0..self.bounds {
            for y in 0..self.bounds {
                for z in 0..self.bounds {
                    // Add to the total if the cell is alive
                    if !self.cells[x as usize][y as usize][z as usize].is_dead() {
                        total += 1;
                    }
                }
            }
        }
        total
    }

    // Set the size of the bounds
    pub fn set_bounds(&mut self, new_bounds: i32) -> i32 {
        if new_bounds != self.bounds {
            // Source: https://programming-idioms.org/idiom/27/create-a-3-dimensional-array/452/rust
            self.cells =
                vec![
                    vec![
                        vec![
                            SimpleCell::new();
                            new_bounds as usize];
                        new_bounds as usize];
                    new_bounds as usize
                ];
            self.bounds = new_bounds;
        }
        self.bounds as i32
    }

    // Update the cells
    fn update(&mut self, rule: &Rule) {
        //Initialize two empty vectors, spawns and deaths, to store cell positions
        let mut spawns: Vec<Position> = vec![];
        let mut deaths: Vec<Position> = vec![];

        for x in 0..=self.bounds - 1 {
            for y in 0..=self.bounds - 1 {
                for z in 0..=self.bounds - 1 {
                    let index = Position::new(x, y, z);
                    let mut cell = self.cells[index.x][index.y][index.z];

                    // Check cell state (dead/alive)
                    match cell.is_dead() {
                        // Dead cell
                        true => {
                            // Spawn a new cell if it has a valid number of neighbours
                            if rule.birth.is_valid(cell.neighbours) {
                                cell.state = rule.states;
                                spawns.push(index)
                            }
                        }
                        // Alive cell
                        false => {
                            let num_states = rule.states;
                            let valid_survival = rule.survival.is_valid(cell.neighbours);

                            // Kill cell if it has too few states, or does not have enough to survive
                            if cell.state < num_states || !valid_survival {
                                if cell.state == num_states {
                                    deaths.push(index);
                                }
                                // Decrement cell state
                                cell.state -= 1;
                            }
                        }
                    }
                    // Update the cell
                    self.cells[index.x][index.y][index.z] = cell;
                }
            }
        }
        // Update each cell's neighbours
        for position in spawns {
            self.update_neighbours(rule, position, true);
        }

        for position in deaths {
            self.update_neighbours(rule, position, false);
        }
    }

    fn update_neighbours(&mut self, rule: &Rule, pos: Position, inc: bool) {
        // Iterate through each element in the neighbourhood
        for n in rule.neighbourhood.get_neighbourhood_iter() {
            let neighbour_pos = Position::from_vec(self.wrap(
                IVec3 {
                    x: pos.x as i32,
                    y: pos.y as i32,
                    z: pos.z as i32,
                } + *n,
            ));

            // Increment or decrement cell's value
            match inc {
                true => {
                    self.cells[neighbour_pos.x][neighbour_pos.y][neighbour_pos.z].neighbours += 1;
                }
                false => {
                    self.cells[neighbour_pos.x][neighbour_pos.y][neighbour_pos.z].neighbours -= 1;
                }
            }
        }
    }

    fn wrap(&self, pos: IVec3) -> IVec3 {
        wrap(pos, self.bounds)
    }

    // Spawn a random amount of cells in the centre
    fn spawn_noise(&mut self, rule: &Rule) {
        default_noise(get_centre(self.bounds), |pos| {
            // Update a cell if it is dead, if not, leave it
            if self.cells[pos.x as usize][pos.y as usize][pos.z as usize].is_dead() {
                self.cells[pos.x as usize][pos.y as usize][pos.z as usize].state = rule.states;
                self.update_neighbours(rule,  Position::from_vec(pos), true);
            }
        });
    }
}

impl crate::cells::Sim for MultiDimensional {
    fn update(&mut self, rule: &Rule, _task_pool: &TaskPool) {
        self.update(rule);
    }

    fn render(&self, renderer: &mut CellRenderer) {
        // Convert 3D vector into 1D vector
        for (index, cell) in self
            .cells
            // Flatten vec
            .concat()
            // Flatten vector again
            .concat()
            // Iterate through each cell
            .iter()
            // Add a counter to determine index
            .enumerate()
        {
            // Render each cell
            renderer.set(index, cell.state, cell.neighbours);
        }
    }

    fn spawn_noise(&mut self, rule: &Rule) {
        self.spawn_noise(rule);
    }

    fn count(&self) -> usize {
        self.get_count()
    }

    fn get_bounds(&self) -> i32 {
        self.get_bounds()
    }

    fn set_bounds(&mut self, new_bounds: i32) -> i32 {
        self.set_bounds(new_bounds)
    }
}

// TESTS
#[cfg(test)]
mod multi_dimensional {
    use super::*;

    #[test]
    fn test_count_cells() {
        let cells = vec![
            vec![
                vec![
                    SimpleCell {
                        state: 5,
                        neighbours: 0,
                    },
                    SimpleCell {
                        state: 1,
                        neighbours: 0,
                    },
                ],
                vec![
                    SimpleCell {
                        state: 0,
                        neighbours: 0,
                    },
                    SimpleCell {
                        state: 1,
                        neighbours: 0,
                    },
                ],
            ],
            vec![
                vec![
                    SimpleCell {
                        state: 2,
                        neighbours: 0,
                    },
                    SimpleCell {
                        state: 1,
                        neighbours: 0,
                    },
                ],
                vec![
                    SimpleCell {
                        state: 0,
                        neighbours: 0,
                    },
                    SimpleCell {
                        state: 1,
                        neighbours: 0,
                    },
                ],
            ],
        ];

        let bounds: usize = 2;

        let multi_dimensional = MultiDimensional {
            cells,
            bounds: bounds as i32,
        };

        // Validate there are two dead cells in the grid
        assert_eq!(multi_dimensional.get_count(), 6);
    }
}
