use crate::protocol::{Wall, Hero};

pub struct Grid {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, Debug)]
enum Cell{
    Empty,
    Wall,
    Hero(i32), //hero id
    LastSeenEnemy(i32), //hero id
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            cells: vec![vec![Cell::Empty; width]; height],
            width,
            height,
        }
    }

    pub fn grid_add_wall(&mut self, wall :&Wall) {
        let mut x = wall.x as usize;
        let mut y = wall.y as usize;
        x = (x-1)/3;
        y = (y-1)/3;
        if x < self.width && y < self.height {
            self.cells[y][x] = Cell::Wall;
        }
    }

    pub fn grid_add_hero(&mut self, hero: &Hero) {
        let mut x = hero.x as usize;
        let mut y = hero.y as usize;
        x = (x-1)/3;
        y = (y-1)/3; 
        if x < self.width && y < self.height {
            self.cells[y][x] = Cell::Hero(hero.id);
        }
    }

    pub fn grid_move_hero(&mut self, hero: &Hero, new_x: usize, new_y: usize) {
        let old_x = hero.x as usize;
        let old_y = hero.y as usize;
        if old_x < self.width && old_y < self.height {
            self.cells[old_y][old_x] = Cell::Empty; // Clear old position
        }
        if new_x < self.width && new_y < self.height {
            self.cells[new_y][new_x] = Cell::Hero(hero.id); // Set new position
        }
    }

    pub fn update_grid(&mut self, heroes: &[Hero], walls: &[Wall])
    {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                if let Cell::Hero(_) = cell {
                    *cell = Cell::Empty; // Clear hero positions, will be updated later
                }
            }
        }
        
        for wall in walls {
            self.grid_add_wall(wall);
        }
        for hero in heroes {
            self.grid_add_hero(hero);
        }
    }

    pub fn print_grid(&self) {
        for row in &self.cells {
            for cell in row {
                let symbol = match cell {
                    Cell::Empty => '.',
                    Cell::Wall => '#',
                    Cell::Hero(_) => 'H',
                    Cell::LastSeenEnemy(_) => 'E',
                };
                print!("{}", symbol);
            }
            println!();
        }
    }

    pub fn has_enemy_hero(&self, hero: &Hero) -> bool {
        for row in &self.cells {
            for cell in row {
                if let Cell::Hero(other_id) = cell {
                    if *other_id != hero.id {
                        return true; // Found an enemy hero
                    }
                }
            }
        }
        false // No enemy heroes found
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn world_to_cell(&self, x: i32, y: i32) -> Option<(usize, usize)> {
        if x <= 0 || y <= 0 {
            return None;
        }

        let cell_x = ((x as usize) - 1) / 3;
        let cell_y = ((y as usize) - 1) / 3;

        if cell_x < self.width && cell_y < self.height {
            Some((cell_x, cell_y))
        } else {
            None
        }
    }

    pub fn cell_to_world(&self, cell_x: usize, cell_y: usize) -> (i32, i32) {
        ((cell_x as i32) * 3 + 1, (cell_y as i32) * 3 + 1)
    }

    pub fn is_walkable_cell(&self, cell_x: usize, cell_y: usize) -> bool {
        if cell_x >= self.width || cell_y >= self.height {
            return false;
        }

        matches!(self.cells[cell_y][cell_x], Cell::Empty | Cell::LastSeenEnemy(_))
    }

    pub fn is_wall_cell(&self, cell_x: usize, cell_y: usize) -> bool {
        if cell_x >= self.width || cell_y >= self.height {
            return false;
        }

        matches!(self.cells[cell_y][cell_x], Cell::Wall)
    }

    pub fn bresenham_line(&self, start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
        let mut points = Vec::new();

        let (x0, y0) = (start.0 as i32, start.1 as i32);
        let (x1, y1) = (end.0 as i32, end.1 as i32);

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x0;
        let mut y = y0;

        loop {
            if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
                points.push((x as usize, y as usize));
            }

            if x == x1 && y == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }

        points
    }

    pub fn cardinal_neighbors(&self, cell_x: usize, cell_y: usize) -> Vec<(usize, usize)> {
        // Return 8-directional neighbors (including diagonals), skipping out-of-bounds
        let mut neighbors = Vec::with_capacity(8);

        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = (cell_x as i32) + dx;
                let ny = (cell_y as i32) + dy;

                if nx >= 0 && ny >= 0 {
                    let (nxu, nyu) = (nx as usize, ny as usize);
                    if nxu < self.width && nyu < self.height {
                        neighbors.push((nxu, nyu));
                    }
                }
            }
        }

        neighbors
    }
}
