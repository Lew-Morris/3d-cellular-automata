use crate::{
    render::CellRenderer,
    rule::Rule,
    utilities,
    utilities::{default_noise, get_centre, wrap},
};
use bevy::math::ivec3;
use bevy::{math::IVec3, tasks::TaskPool};
use bevy::utils::petgraph::visit::Walker;
use futures_lite::future;

extern crate num_cpus;

pub fn is_dead(value: u8) -> bool {
    value == 0
}

#[derive(Clone)]
pub struct ParallelCell {
    state: u8,
    neighbours: u8,
}

impl ParallelCell {
    fn new() -> ParallelCell {
        ParallelCell {
            state: 0,
            neighbours: 0,
        }
    }

    fn get_state(&self) -> u8 {
        self.state
    }

    fn set_state(&mut self, state: u8) -> u8 {
        self.state = state;
        self.state
    }

    fn is_dead(&self) -> bool {
        self.state == 0
    }
}

#[derive(Clone)]
pub struct Chunk {
    cells: Vec<Vec<Vec<ParallelCell>>>,
    bounds: usize,
    index: u8,
}

impl Chunk {
    fn new(index: u8) -> Chunk {
        Chunk {
            cells: vec![vec![vec![]]],
            bounds: 0,
            index,
        }
    }

    fn get_bounds(&self) -> usize {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: usize) -> usize {
        self.bounds = bounds;
        self.bounds
    }
}

#[derive(Clone)]
pub struct MultiThreaded {
    chunks: Vec<Chunk>,
    bounds: usize,
}

impl MultiThreaded {
    pub fn new() -> MultiThreaded {
        MultiThreaded {
            chunks: vec![],
            bounds: 0,
        }
    }

    // pub fn get_bounds(&self) -> usize {
    //     self.bounds
    // }

    pub fn set_bounds(&mut self, bounds: usize) -> usize {
        self.bounds = bounds;
        self.bounds
    }

    pub fn get_count(&self) -> usize {
        let mut total = 0;
        for chunks in self.chunks.iter() {
            let mut chunk_total = 0;
            for cell in chunks.cells.concat().concat().iter() {
                if !cell.is_dead() {
                    chunk_total += 1;
                }
            }
            total += chunk_total;
        }
        total
    }

    fn update(&mut self, rule: &Rule, tasks: &TaskPool) {
        let mut update_tasks = vec![];
        // for each chunk
        for (index, chunk) in self.chunks.iter().enumerate() {
            // copy all of the values
            let mut cells = chunk.cells.clone();
            let rule = rule.clone();

            // create two vectors to store spawns and deaths
            let mut chunk_spawns = vec![];
            let mut chunk_deaths = vec![];
            // Add this to the update tasks vec with async move
            update_tasks.push(tasks.spawn(async move {
                Self::update_values(&mut cells, &rule, &mut chunk_spawns, &mut chunk_deaths, index);
                (index, chunk_spawns, chunk_deaths)
            }));
        }

        // collect spawns & deaths.
        let mut chunk_spawns = vec![];
        let mut chunk_deaths = vec![];
        for task in update_tasks {
            let (index, spawns, deaths) = future::block_on(task);
            chunk_spawns.push((index, spawns));
            chunk_deaths.push(deaths);
        }

        // update neighbours.
        let mut neighbour_tasks = vec![];
        for ((index, spawns), deaths) in chunk_spawns.into_iter().zip(chunk_deaths) {
            let mut neighbours = self.chunks[index].clone(); // todo! need to get the index
            let rule = rule.clone(); // shrug

            neighbour_tasks.push(tasks.spawn(async move {
                for index in spawns.iter() {
                    Self::update_neighbours(&mut neighbours, &rule, index, true);
                }

                for index in deaths.iter() {
                    Self::update_neighbours(&mut neighbours, &rule, index, false);
                }
            }));
        }

        for task in neighbour_tasks {
            future::block_on(task);
        }
    }

    fn update_values(
        cells: &mut Vec<Vec<Vec<ParallelCell>>>,
        rule: &Rule,
        chunk_spawns: &mut Vec<Vec<usize>>,
        chunk_deaths: &mut Vec<Vec<usize>>,
        index: usize,
    ) -> usize {
        // Loop through each cell in the chunk
        for (x, outer) in cells.iter_mut().enumerate() {
            for (y, inner) in outer.iter_mut().enumerate() {
                for (z, mut cell) in inner.iter_mut().enumerate() {
                    let position = vec![x, y, z];
                    // Check the rules
                    if cell.is_dead() {
                        if rule.birth.is_valid(cell.neighbours) {
                            // Set cell state to rule states
                            cell.state = rule.states;
                            // Add position to spawns
                            chunk_spawns.push(position)
                        }
                    } else if cell.state < rule.states || !rule.survival.is_valid(cell.neighbours) {
                        // Dead cell
                        if cell.is_dead() {
                            // Spawn a new cell if it has a valid number of neighbours
                            if rule.birth.is_valid(cell.neighbours) {
                                cell.state = rule.states;
                                chunk_spawns.push(position)
                            }
                        }
                        // Live cell
                        else {
                            let num_states = rule.states;
                            let valid_survival = rule.survival.is_valid(cell.neighbours);

                            // Kill cell if it has too few states, or does not have enough to survive
                            if cell.state < num_states || !valid_survival {
                                if cell.state == num_states {
                                    chunk_deaths.push(position);
                                }
                                // Decrement cell's state
                                cell.state -= 1;
                            }
                        }
                    }
                }
            }
        }
        index
    }

    fn update_neighbours(neighbours: &mut Chunk, rule: &Rule, pos: &Vec<usize>, inc: bool) {
        for n in rule.neighbourhood.get_neighbourhood_iter() {
            let pos = IVec3::new(pos[1] as i32, pos[2] as i32, pos[3] as i32);
            let neighbour_pos = wrap(pos + *n, neighbours.bounds as i32);

            match inc {
                true => {
                    neighbours.cells[neighbour_pos.x as usize][neighbour_pos.y as usize]
                        [neighbour_pos.z as usize]
                        .neighbours += 1;
                }
                false => {
                    neighbours.cells[neighbour_pos.x as usize][neighbour_pos.y as usize]
                        [neighbour_pos.z as usize]
                        .neighbours -= 1;
                }
            }
        }
    }

    fn get_centre(&self) -> IVec3 {
        let centre = (self.bounds / 2) as i32;
        ivec3(
            centre,
            centre,
            centre,
        )
    }

    fn wrap(&self, pos: IVec3) -> IVec3 {
        todo!()
    }

    fn spawn_noise(&mut self, rule: &Rule) {
        let center = self.get_centre();
        let bounds = self.bounds;

        default_noise(center, |pos| {
            let index = utilities::pos_to_idx(
                wrap(pos, bounds as i32),
                self.bounds as i32
            );

            // todo! Update cell values
        });
    }
}

impl crate::cells::Sim for MultiThreaded {
    fn update(&mut self, rule: &Rule, task_pool: &TaskPool) {
        /*
            Domain Decomposition
            - Split the cube into x amount of chunks
                - Likely to be the number of CPUS/threads on the system
            - Give each thread its chunk of the cube, with an extra layer on each side as padding
                - The thread would only need to do the cells which are not in the boundary
            - Wait for each thread to finish, then render
            - This is called `Domain Decomposition`

            Cell Index-Based
            - assign a single cell to each thread
            - Write the output from the thread to a new MultiThreaded instance
            - Do for every cell

        */

        self.update(rule, task_pool);
    }

    fn render(&self, renderer: &mut CellRenderer) {
        for chunk in self.chunks.iter() {
            for (index, cell) in chunk.cells.concat().concat().iter().enumerate() {
                renderer.set(
                    index,
                    cell.state,
                    cell.neighbours
                )
            }
        }
    }

    fn spawn_noise(&mut self, rule: &Rule) {
        self.spawn_noise(rule);
    }

    fn count(&self) -> usize {
        self.get_count() * num_cpus::get()
    }

    fn get_bounds(&self) -> i32 {
        self.get_bounds() as i32
    }

    fn set_bounds(&mut self, new_bounds: i32) -> i32 {
        self.set_bounds(new_bounds as usize) as i32
    }
}
