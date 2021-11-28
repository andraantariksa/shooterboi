use bitflags::bitflags;
use rand::distributions::Uniform;
use rand::prelude::SmallRng;
use rand::{Rng, RngCore, SeedableRng};
use std::borrow::BorrowMut;
use std::collections::VecDeque;
use std::rc::Rc;

bitflags! {
    pub struct Wall: u8 {
        const LEFT = 0b00000001;
        const TOP = 0b00000010;
        const RIGHT = 0b00000100;
        const BOTTOM = 0b00001000;
    }
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub wall: Wall,
    pub visited: bool,
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct MazeGrid {
    cells: Vec<Vec<Cell>>,
}

impl MazeGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = vec![];
        for y in 0..height {
            let mut cells_x = vec![];
            for x in 0..width {
                cells_x.push(Cell {
                    visited: false,
                    wall: Wall::all(),
                    x,
                    y,
                });
            }
            cells.push(cells_x);
        }
        Self { cells }
    }

    pub fn make_visited(&mut self, x: usize, y: usize) {
        self.cells[y][x].visited = true;
    }

    pub fn remove_wall(&mut self, x: usize, y: usize, wall: Wall) {
        self.cells[y][x].wall.remove(wall);
    }

    pub fn count_cells(&self) -> usize {
        self.cells.len() * self.cells[0].len()
    }

    pub fn size(&self) -> (usize, usize) {
        (self.cells[0].len(), self.cells.len())
    }

    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.cells[y][x]
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y][x]
    }
}

struct CellNeighbours {
    neighbours: Vec<Cell>,
}

impl CellNeighbours {
    pub fn new() -> Self {
        Self { neighbours: vec![] }
    }

    pub fn add_neighbour(&mut self, neighbour: Cell) {
        self.neighbours.push(neighbour);
    }

    pub fn len(&self) -> usize {
        self.neighbours.len()
    }

    pub fn get_neighbour(&self, idx: usize) -> &Cell {
        &self.neighbours[idx]
    }
}

fn generate_maze(grid: &mut MazeGrid) {
    let mut s = VecDeque::<Cell>::new();
    let mut rng = SmallRng::from_entropy();

    s.push_back(grid.get_cell(0, 0).clone());
    grid.make_visited(0, 0);

    let (width, height) = grid.size();

    loop {
        if let Some(current) = s.pop_back() {
            let LEFT = (current.x as i32 - 1);
            let top = (current.y as i32 - 1);
            let RIGHT = (current.x as i32 + 1);
            let BOTTOM = (current.y as i32 + 1);

            let mut unvisited_neighbours = CellNeighbours::new();

            if LEFT >= 0 && (LEFT as usize) < width {
                let c = grid.get_cell(LEFT as usize, current.y);
                if !c.visited {
                    unvisited_neighbours.add_neighbour(c.clone());
                }
            }

            if top >= 0 && (top as usize) < height {
                let c = grid.get_cell(current.x, top as usize);
                if !c.visited {
                    unvisited_neighbours.add_neighbour(c.clone());
                }
            }

            if RIGHT >= 0 && (RIGHT as usize) < width {
                let c = grid.get_cell(RIGHT as usize, current.y);
                if !c.visited {
                    unvisited_neighbours.add_neighbour(c.clone());
                }
            }

            if BOTTOM >= 0 && (BOTTOM as usize) < height {
                let c = grid.get_cell(current.x, BOTTOM as usize);
                if !c.visited {
                    unvisited_neighbours.add_neighbour(c.clone());
                }
            }

            let unvisited_count = unvisited_neighbours.len();
            if unvisited_count > 0 {
                let idx = rng.sample(Uniform::new(0, unvisited_count));
                let c = unvisited_neighbours.get_neighbour(idx);

                s.push_back(current.clone());
                s.push_back(c.clone());

                if c.x > current.x {
                    grid.remove_wall(current.x, current.y, Wall::RIGHT);
                    grid.remove_wall(c.x, c.y, Wall::LEFT);
                } else if c.x < current.x {
                    grid.remove_wall(current.x, current.y, Wall::LEFT);
                    grid.remove_wall(c.x, c.y, Wall::RIGHT);
                }

                if c.y > current.y {
                    grid.remove_wall(current.x, current.y, Wall::BOTTOM);
                    grid.remove_wall(c.x, c.y, Wall::TOP);
                } else if c.y < current.y {
                    grid.remove_wall(current.x, current.y, Wall::TOP);
                    grid.remove_wall(c.x, c.y, Wall::BOTTOM);
                }

                grid.make_visited(c.x, c.y);
            }
        } else {
            break;
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn print_maze() {
        let mut maze_grid = MazeGrid::new(10, 10);
        generate_maze(&mut maze_grid);
    }
}
