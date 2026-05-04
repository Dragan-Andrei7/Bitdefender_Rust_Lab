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
}
