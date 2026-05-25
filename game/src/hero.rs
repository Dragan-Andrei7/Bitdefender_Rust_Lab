use std::collections::VecDeque;

use crate::{grid::Grid, protocol::{Hero, Projectile}};
use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeroState {
    PatrolDown,
    PatrolUp,
    EnemySpotted,
}

pub struct Hero_s {
    pub hero: Hero,
    pub state: HeroState,
    // Future pathfinding output: queue of world coordinates to execute.
    pub planned_moves: VecDeque<(i32, i32)>,
}

impl Hero_s {
    pub fn new(hero: &Hero) -> Self {
        Hero_s {
            hero: hero.clone(),
            state: HeroState::PatrolDown,
            planned_moves: VecDeque::new(),
        }
    }

    fn hero_go_up(&self) -> (i32, i32) {
        (self.hero.x, self.hero.y - 3)
    }

    fn hero_go_down(&self) -> (i32, i32) {
        (self.hero.x, self.hero.y + 3)
    }

    pub fn move_away_from(&self, enemy: &Hero) -> (i32, i32) {
        // Compute direction away from enemy and move in that direction (step of 3).
        // Clamp result to grid bounds.
        let dx = self.hero.x - enemy.x;
        let dy = self.hero.y - enemy.y;
        
        let magnitude = ((dx * dx + dy * dy) as f32).sqrt();
        if magnitude < 0.001 {
            // Exactly at enemy position (very unlikely), don't move.
            return (self.hero.x, self.hero.y);
        }
        
        let norm_dx = (dx as f32) / magnitude;
        let norm_dy = (dy as f32) / magnitude;
        
        let new_x = (self.hero.x as f32 + norm_dx * 3.0).round() as i32;
        let new_y = (self.hero.y as f32 + norm_dy * 3.0).round() as i32;
        
        // Clamp to reasonable bounds (stay within world grid)
        let clamped_x = new_x.max(1).min(73);
        let clamped_y = new_y.max(1).min(88);
        
        (clamped_x, clamped_y)
    }

    pub fn distance_to(&self, other: &Hero) -> i32 {
        let my_cell_x = (self.hero.x - 1) / 3;
        let my_cell_y = (self.hero.y - 1) / 3;
        let other_cell_x = (other.x - 1) / 3;
        let other_cell_y = (other.y - 1) / 3;

        (my_cell_x - other_cell_x).abs().max((my_cell_y - other_cell_y).abs())
    }

    fn current_cell(&self, grid: &Grid) -> Option<(usize, usize)> {
        grid.world_to_cell(self.hero.x, self.hero.y)
    }

    fn candidate_cells(&self, grid: &Grid) -> Vec<(usize, usize)> {
        let Some((cell_x, cell_y)) = self.current_cell(grid) else {
            return Vec::new();
        };

        let mut candidates = Vec::with_capacity(9);
        candidates.push((cell_x, cell_y));
        candidates.extend(grid.cardinal_neighbors(cell_x, cell_y));
        candidates.retain(|(next_x, next_y)| grid.is_walkable_cell(*next_x, *next_y));
        candidates
    }

    fn cell_distance_to_enemy(cell: (usize, usize), grid: &Grid, enemy: &Hero) -> i32 {
        let Some(enemy_cell) = grid.world_to_cell(enemy.x, enemy.y) else {
            return i32::MAX;
        };

        (cell.0 as i32 - enemy_cell.0 as i32).abs().max((cell.1 as i32 - enemy_cell.1 as i32).abs())
    }

    fn choose_random_best_cell<F>(&self, grid: &Grid, score_fn: F) -> Option<(usize, usize)>
    where
        F: Fn((usize, usize)) -> i32,
    {
        let mut best_cells: Vec<(usize, usize)> = Vec::new();
        let mut best_score: Option<i32> = None;

        for cell in self.candidate_cells(grid) {
            let score = score_fn(cell);
            match best_score {
                None => {
                    best_score = Some(score);
                    best_cells.push(cell);
                }
                Some(current_best) if score > current_best => {
                    best_score = Some(score);
                    best_cells.clear();
                    best_cells.push(cell);
                }
                Some(current_best) if score == current_best => {
                    best_cells.push(cell);
                }
                _ => {}
            }
        }

        best_cells.choose(&mut thread_rng()).copied()
    }

    fn move_towards_enemy(&mut self, grid: &Grid, enemy: &Hero) -> (i32, i32) {
        let Some(best_cell) = self
            .choose_random_best_cell(grid, |cell| {
                let dist = Self::cell_distance_to_enemy(cell, grid, enemy);
                let manhattan = (cell.0 as i32 - enemy.x).abs() + (cell.1 as i32 - enemy.y).abs();
                -dist * 100 - manhattan
            })
        else {
            return (self.hero.x, self.hero.y);
        };

        grid.cell_to_world(best_cell.0, best_cell.1)
    }

    fn move_away_from_enemy(&mut self, grid: &Grid, enemy: &Hero) -> (i32, i32) {
        let Some(best_cell) = self
            .choose_random_best_cell(grid, |cell| {
                let dist = Self::cell_distance_to_enemy(cell, grid, enemy);
                let manhattan = (cell.0 as i32 - enemy.x).abs() + (cell.1 as i32 - enemy.y).abs();
                dist * 100 - manhattan
            })
        else {
            return (self.hero.x, self.hero.y);
        };

        grid.cell_to_world(best_cell.0, best_cell.1)
    }

    fn keep_distance_from_enemy(&mut self, grid: &Grid, enemy: &Hero) -> (i32, i32) {
        let Some(best_cell) = self
            .choose_random_best_cell(grid, |cell| {
                let dist = Self::cell_distance_to_enemy(cell, grid, enemy);
                let band_score = 3 - (dist - 3).abs();
                band_score * 100 - dist
            })
        else {
            return (self.hero.x, self.hero.y);
        };

        grid.cell_to_world(best_cell.0, best_cell.1)
    }

    fn projectile_is_threat(&self, grid: &Grid, projectile: &Projectile) -> bool {
        let Some(hero_cell) = grid.world_to_cell(self.hero.x, self.hero.y) else {
            return false;
        };
        let Some(projectile_cell) = grid.world_to_cell(projectile.x, projectile.y) else {
            return false;
        };

        let dx = (hero_cell.0 as i32 - projectile_cell.0 as i32).abs();
        let dy = (hero_cell.1 as i32 - projectile_cell.1 as i32).abs();
        if !(dx == 0 || dy == 0 || dx == dy) {
            return false;
        }

        let line = grid.bresenham_line(projectile_cell, hero_cell);
        for (index, (cell_x, cell_y)) in line.iter().enumerate() {
            if index == 0 || index + 1 == line.len() {
                continue;
            }

            if grid.is_wall_cell(*cell_x, *cell_y) {
                return false;
            }
        }

        true
    }

    fn dodge_projectiles(&mut self, grid: &Grid, enemy: &Hero, projectiles: &[Projectile]) -> (i32, i32) {
        let Some(best_cell) = self
            .choose_random_best_cell(grid, |cell| {
                let projectile_distance = projectiles
                    .iter()
                    .filter(|projectile| projectile.owner_id != self.hero.owner_id && self.projectile_is_threat(grid, projectile))
                    .filter_map(|projectile| grid.world_to_cell(projectile.x, projectile.y))
                    .map(|projectile_cell| {
                        (cell.0 as i32 - projectile_cell.0 as i32)
                            .abs()
                            .max((cell.1 as i32 - projectile_cell.1 as i32).abs())
                    })
                    .min()
                    .unwrap_or(i32::MAX / 2);

                let enemy_distance = Self::cell_distance_to_enemy(cell, grid, enemy);
                let band_score = 3 - (enemy_distance - 3).abs();
                projectile_distance * 100 + band_score * 10 - enemy_distance
            })
        else {
            return (self.hero.x, self.hero.y);
        };

        grid.cell_to_world(best_cell.0, best_cell.1)
    }

    pub fn plan_keep_distance_from_enemy(&mut self, grid: &Grid, enemy: &Hero, cooldown: i32, projectiles: &[Projectile]) -> (i32, i32, bool) {
        let threatened = projectiles.iter().any(|projectile| {
            projectile.owner_id != self.hero.owner_id && self.projectile_is_threat(grid, projectile)
        });

        if threatened {
            let (x, y) = self.dodge_projectiles(grid, enemy, projectiles);
            return (x, y, false);
        }

        let distance = self.distance_to(enemy);

        if cooldown == 0 {
            if distance <= 3 && self.validate_shoot(grid, enemy.x, enemy.y) {
                return (self.hero.x, self.hero.y, true);
            }

            if distance > 3 {
                let (x, y) = self.move_towards_enemy(grid, enemy);
                return (x, y, false);
            }

            let (x, y) = self.keep_distance_from_enemy(grid, enemy);
            return (x, y, false);
        }

        if distance >= 3 {
            let (x, y) = self.move_towards_enemy(grid, enemy);
            return (x, y, false);
        }

        let (x, y) = self.keep_distance_from_enemy(grid, enemy);
        (x, y, false)
    }

    pub fn plan_retreat_from_enemy(&mut self, grid: &Grid, enemy: &Hero) -> bool {
        self.planned_moves.clear();

        let Some(start) = grid.world_to_cell(self.hero.x, self.hero.y) else {
            return false;
        };

        let Some((enemy_cell_x, enemy_cell_y)) = grid.world_to_cell(enemy.x, enemy.y) else {
            return false;
        };

        let mut best_target: Option<(usize, usize)> = None;
        let mut best_score = i32::MIN;

        let (grid_width, grid_height) = grid.dimensions();

        for cell_y in 0..grid_height {
            for cell_x in 0..grid_width {
                if !grid.is_walkable_cell(cell_x, cell_y) {
                    continue;
                }

                let dx = cell_x as i32 - enemy_cell_x as i32;
                let dy = cell_y as i32 - enemy_cell_y as i32;
                let score = dx * dx + dy * dy;

                if score > best_score {
                    best_score = score;
                    best_target = Some((cell_x, cell_y));
                }
            }
        }

        let Some((target_x, target_y)) = best_target else {
            return false;
        };

        let target_world_coords = grid.cell_to_world(target_x, target_y);
        if start == grid.world_to_cell(target_world_coords.0, target_world_coords.1).unwrap_or(start) {
            return true;
        }

        self.plan_path_to(grid, target_world_coords.0, target_world_coords.1)
    }

    pub fn plan_path_to(&mut self, grid: &Grid, end_x: i32, end_y: i32) -> bool {
        self.planned_moves.clear();

        let Some(start) = grid.world_to_cell(self.hero.x, self.hero.y) else {
            return false;
        };
        let Some(goal) = grid.world_to_cell(end_x, end_y) else {
            return false;
        };

        if start == goal {
            return true;
        }

        let mut queue = VecDeque::new();
        let mut visited = std::collections::HashSet::new();
        let mut previous = std::collections::HashMap::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            if current == goal {
                break;
            }

            for next in grid.cardinal_neighbors(current.0, current.1) {
                if visited.contains(&next) {
                    continue;
                }

                if next != goal && !grid.is_walkable_cell(next.0, next.1) {
                    continue;
                }

                visited.insert(next);
                previous.insert(next, current);
                queue.push_back(next);
            }
        }

        if !visited.contains(&goal) {
            return false;
        }

        let mut path_cells = Vec::new();
        let mut cursor = goal;
        while cursor != start {
            path_cells.push(cursor);
            let Some(prev) = previous.get(&cursor).copied() else {
                return false;
            };
            cursor = prev;
        }

        path_cells.reverse();
        for (cell_x, cell_y) in path_cells {
            self.planned_moves.push_back(grid.cell_to_world(cell_x, cell_y));
        }

        true
    }

    pub fn validate_shoot(&self, grid: &Grid, target_x: i32, target_y: i32) -> bool {
        let Some(start_cell) = grid.world_to_cell(self.hero.x, self.hero.y) else {
            return false;
        };
        let Some(target_cell) = grid.world_to_cell(target_x, target_y) else {
            return false;
        };

        let line_cells = grid.bresenham_line(start_cell, target_cell);
        for (index, (cell_x, cell_y)) in line_cells.iter().enumerate() {
            if index == 0 || index + 1 == line_cells.len() {
                continue;
            }

            if grid.is_wall_cell(*cell_x, *cell_y) {
                return false;
            }
        }

        true
    }

    pub fn shoot_target_beyond_enemy(&self, grid: &Grid, enemy: &Hero) -> (i32, i32) {
        let Some(start_cell) = grid.world_to_cell(self.hero.x, self.hero.y) else {
            return (enemy.x, enemy.y);
        };
        let Some(enemy_cell) = grid.world_to_cell(enemy.x, enemy.y) else {
            return (enemy.x, enemy.y);
        };

        let step_x = (enemy_cell.0 as i32 - start_cell.0 as i32).signum();
        let step_y = (enemy_cell.1 as i32 - start_cell.1 as i32).signum();

        if step_x == 0 && step_y == 0 {
            return (enemy.x, enemy.y);
        }

        let mut best_cell = enemy_cell;
        let mut current_cell = enemy_cell;

        loop {
            let next_x = current_cell.0 as i32 + step_x;
            let next_y = current_cell.1 as i32 + step_y;

            if next_x < 0 || next_y < 0 {
                break;
            }

            let next_cell = (next_x as usize, next_y as usize);
            if next_cell.0 >= grid.dimensions().0 || next_cell.1 >= grid.dimensions().1 {
                break;
            }

            let next_world = grid.cell_to_world(next_cell.0, next_cell.1);
            if !self.validate_shoot(grid, next_world.0, next_world.1) {
                break;
            }

            best_cell = next_cell;
            current_cell = next_cell;
        }

        grid.cell_to_world(best_cell.0, best_cell.1)
    }

    pub fn validate_path(&mut self, grid: &Grid) {
        // Check if the next planned move is still walkable after grid discovery.
        // If blocked, clear the entire plan to force replanning with new information.
        if let Some(&(next_x, next_y)) = self.planned_moves.front() {
            if let Some((cell_x, cell_y)) = grid.world_to_cell(next_x, next_y) {
                if !grid.is_walkable_cell(cell_x, cell_y) {
                    self.planned_moves.clear();
                }
            } else {
                // Target is out of bounds, clear the plan.
                self.planned_moves.clear();
            }
        }
    }

    pub fn hero_logic(&mut self, grid: &Grid) -> (i32, i32) {
        // If behavior changes, clear queued instructions to avoid stale moves.
        if grid.has_enemy_hero(&self.hero) {
            if self.state != HeroState::EnemySpotted {
                self.planned_moves.clear();
                self.state = HeroState::EnemySpotted;
            }
            return (self.hero.x, self.hero.y);
        }

        if self.state == HeroState::EnemySpotted {
            self.planned_moves.clear();
            self.state = HeroState::PatrolDown;
        }

        match self.state {
            HeroState::PatrolDown => {
                if self.hero.y >= 27 {
                    self.state = HeroState::PatrolUp;
                    self.hero_go_up()
                } else {
                    self.hero_go_down()
                }
            }
            HeroState::PatrolUp => {
                if self.hero.y <= 3 {
                    self.state = HeroState::PatrolDown;
                    self.hero_go_down()
                } else {
                    self.hero_go_up()
                }
            }
            HeroState::EnemySpotted => (self.hero.x, self.hero.y),
        }
    }
}