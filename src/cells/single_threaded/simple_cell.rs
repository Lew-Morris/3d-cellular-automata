use bevy::{
    math::IVec3,
    tasks::TaskPool
};

use crate::{
    utilities::{
        idx_to_pos,
        pos_to_idx,
        wrap,
        get_centre,
        default_noise,
    },
    render::CellRenderer,
    rule::Rule,
};

#[derive(Clone, Copy)]
struct SimpleCell {
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

    // Return true if the cell has value 0
    pub fn is_dead(&self) -> bool {
        self.state == 0
    }

}

pub struct SingleThreaded {
    cells: Vec<SimpleCell>,
    bounds: i32,
    // group: u8, // todo! Different cell types that compete

}

impl SingleThreaded {
    pub fn new() -> Self {
        SingleThreaded {
            cells: vec![],
            bounds: 0,
        }
    }

    // Set the boundary for cells
    pub fn set_bounds(&mut self, new_bounds: i32) -> i32 {
        // Check if bounds has changed
        if new_bounds != self.bounds {
            // Clear the array
            self.cells.clear();
            // Initialise vector of cells, with length bounds^3
            self.cells.resize(
                (new_bounds.pow(3)) as usize,
                SimpleCell::new(),
            );
            // vec!(vec!(SimpleCell { value: 0, neighbours: 0 }));
            self.bounds = new_bounds;
        }
        self.bounds
    }

    // Count the number of live cells
    pub fn count_cells(&self) -> usize {
        let mut result: usize = 0;
        // Loop through each
        for cell in &self.cells {
            // Increment if the cell is not dead
            if !cell.is_dead() {
                result += 1;
            }
        }
        result
    }

    // Convert index in vector to a position (x, y, z) in a 3D cube
    fn idx_to_pos(&self, index: usize) -> IVec3 {
        idx_to_pos(index as i32, self.bounds)
    }

    // Convert position (x, y, z) in a 3D cube to an index in a 2D vector
    fn pos_to_idx(&self, position: IVec3) -> usize {
        pos_to_idx(position, self.bounds)
    }

    fn wrap(&self, pos: IVec3) -> IVec3 {
        wrap(pos, self.bounds)
    }

    fn update_neighbours(&mut self, rule: &Rule, index: usize, inc: bool) {
        let pos = self.idx_to_pos(index);
        for dir in rule.neighbourhood.get_neighbourhood_iter() {
            let neighbour_position = self.wrap(pos + *dir);
            let index = self.pos_to_idx(neighbour_position);
            if inc {
                self.cells[index].neighbours += 1;
            }
            else {
                self.cells[index].neighbours -= 1;
            }
        }
    }

    pub fn update(&mut self, rule: &Rule) {
        let mut spawns = vec![];
        let mut deaths = vec![];

        for (index, cell) in self.cells.iter_mut().enumerate() {
            if cell.is_dead() {
                if rule.birth.in_range(cell.neighbours) {
                    cell.state = rule.states;
                    spawns.push(index);
                }
            } else {
                if cell.state < rule.states || !rule.survival.in_range(cell.neighbours) {
                    if cell.state == rule.states {
                        deaths.push(index);
                    }
                    cell.state -= 1;
                }
            }
        }

        // Update neighbouring cells
        for index in spawns {
            self.update_neighbours(rule, index, true);
        }
        // Update dead cells
        for index in deaths {
            self.update_neighbours(rule, index, false);
        }
    }

    // #[allow(dead_code)]
    // pub fn validate(&self, rule: &Rule) {
    //     for index in 0..self.cells.len() {
    //         let pos = self.idx_to_pos(index);
    //
    //         let mut neighbours = 0;
    //         for dir in rule.neighbourhood.get_neighbourhood_iter() {
    //             let neighbour_pos = self.wrap(pos + *dir);
    //
    //             let index = self.pos_to_idx(neighbour_pos);
    //             if self.cells[index].value == rule.states {
    //                 neighbours += 1;
    //             }
    //         }
    //         assert_eq!(neighbours, self.cells[index].neighbours);
    //     }
    // }

    pub fn spawn_noise(&mut self, rule: &Rule) {
        default_noise(get_centre(self.bounds), |pos| {
            let index = self.pos_to_idx(self.wrap(pos));
            if self.cells[index].is_dead() {
                self.cells[index].state = rule.states;
                self.update_neighbours(rule, index, true);
            }
        });
    }
}

impl crate::cells::Sim for SingleThreaded {
    fn update(&mut self, rule: &Rule, _task_pool: &TaskPool) {
        self.update(rule);
    }

    fn render(&self, renderer: &mut CellRenderer) {
        for (index, cell) in self.cells.iter().enumerate() {
            renderer.set(index, cell.state, cell.neighbours);
        }
    }

    fn spawn_noise(&mut self, rule: &Rule) {
        self.spawn_noise(rule);
    }

    fn get_count(&self) -> usize {
        self.count_cells()
    }

    fn get_bounds(&self) -> i32 {
        self.bounds
    }

    fn set_bounds(&mut self, new_bounds: i32) -> i32 {
        self.set_bounds(new_bounds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_dead() {
        let cell = SimpleCell { state: 0, neighbours: 0 };
        assert!(cell.is_dead());

        let cell = SimpleCell { state: 1, neighbours: 0 };
        assert!(!cell.is_dead());
    }

    #[test]
    fn test_set_bounds() {
        let mut sim = SingleThreaded::new();

        // Check initial bounds
        assert_eq!(sim.bounds, 0);

        // Set bounds to 10
        let bounds = sim.set_bounds(10);
        assert_eq!(bounds, 10);

        // Check that the vector has the correct length
        assert_eq!(sim.cells.len(), 1000);

        // Set bounds to 5
        let bounds = sim.set_bounds(5);
        assert_eq!(bounds, 5);

        // Check that the vector has the correct length
        assert_eq!(sim.cells.len(), 125);
    }

    #[test]
    fn test_count_cells() {
        let mut sim = SingleThreaded::new();
        sim.set_bounds(10);

        // Initially all cells should be dead
        assert_eq!(sim.count_cells(), 0);

        // Set some cells to be alive
        sim.cells[0].state = 1;
        sim.cells[10].state = 1;
        sim.cells[20].state = 1;

        // Check that count_cells returns the correct number of live cells
        assert_eq!(sim.count_cells(), 3);
    }

    // #[test]
    // fn test_update_neighbours() {
    //
    //
    //     let rule = Rule {
    //         birth: Value::from_range(3..=3),
    //         survival_rule: Value::from_range(2..=3),
    //         neighbourhood: Neighbourhood::Moore,
    //         states: 2,
    //     };
    //     let mut cells = vec![
    //         SimpleCell { state: 0, neighbours: 0 },
    //         SimpleCell { state: 0, neighbours: 0 },
    //         SimpleCell { state: 0, neighbours: 0 },
    //         SimpleCell { state: 0, neighbours: 0 },
    //     ];
    //     let mut single_threaded = SingleThreaded {
    //         cells,
    //         bounds: 2,
    //     };
    //
    //     single_threaded.update_neighbours(&rule, 0, true);
    //     assert_eq!(single_threaded.cells[1].neighbours, 1);
    //     assert_eq!(single_threaded.cells[2].neighbours, 1);
    //     assert_eq!(single_threaded.cells[3].neighbours, 1);
    //
    //     single_threaded.update_neighbours(&rule, 3, true);
    //     assert_eq!(single_threaded.cells[2].neighbours, 2);
    //     assert_eq!(single_threaded.cells[1].neighbours, 1);
    //
    //     single_threaded.update_neighbours(&rule, 1, false);
    //     assert_eq!(single_threaded.cells[0].neighbours, 1);
    //     assert_eq!(single_threaded.cells[2].neighbours, 1);
    //
    //     single_threaded.update_neighbours(&rule, 2, false);
    //     assert_eq!(single_threaded.cells[1].neighbours, 2);
    //     assert_eq!(single_threaded.cells[3].neighbours, 1);
    // }
    //
    // #[test]
    // fn test_update() {
    //     let mut single_threaded = SingleThreaded::new();
    //     single_threaded.set_bounds(3);
    //
    //     let rule = Rule {
    //         states: 3,
    //         birth: Value::from_range(4..=4),
    //         survival_rule: Value::from_range(5..=5),
    //         neighbourhood: Neighbourhood::Moore,
    //     };
    //
    //     single_threaded.spawn_noise(&rule);
    //
    //     for _i in 0..10 {
    //         single_threaded.update(&rule);
    //     }
    //
    //     assert_eq!(single_threaded.count_cells(), 19);
    // }
}