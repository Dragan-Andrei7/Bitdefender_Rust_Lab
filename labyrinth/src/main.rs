use std::fs::File;
use std::io;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]

struct GridCell {
    x: usize,
    y: usize,    
}

#[derive(Debug, Deserialize, Clone)]

struct Labyrinth {
    width: usize,
    height: usize,
    start: GridCell,
    goal: GridCell,
    grid: Vec<GridCell>,

    #[serde(skip)]
    solution: Option<Vec<GridCell>>,
}

fn print_labyrinth(labyrinth: &Labyrinth) {
    for y in 0..labyrinth.height {
        for x in 0..labyrinth.width {
            if labyrinth.start.x == x && labyrinth.start.y == y {
                print!("S ");
            } else if labyrinth.goal.x == x && labyrinth.goal.y == y {
                print!("G ");
            } else if labyrinth.grid.iter().any(|cell| cell.x == x && cell.y == y) {
                print!("# ");
            } else if labyrinth
                .solution
                .as_ref()
                .is_some_and(|solution| solution.iter().any(|cell| cell.x == x && cell.y == y))
            {
                print!("\x1b[31m* \x1b[0m");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

fn solve_labyrinth(labyrinth: &mut Labyrinth){
    labyrinth.solution = solve_labirinth_backtracking(
        labyrinth,
        labyrinth.start.clone(),
        vec![labyrinth.start.clone()],
    );
}

fn solve_labirinth_backtracking(labyrinth: &Labyrinth, current_position: GridCell, path: Vec<GridCell>) -> Option<Vec<GridCell>> {
    if current_position.x == labyrinth.goal.x && current_position.y == labyrinth.goal.y {
        return Some(path);
    }
    //Check south east north west
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    for (dx, dy) in directions {
        let new_x = current_position.x as isize + dx;
        let new_y = current_position.y as isize + dy;
        if new_x >= 0 && new_x < labyrinth.width as isize && new_y >= 0 && new_y < labyrinth.height as isize {
            let new_cell = GridCell { x: new_x as usize, y: new_y as usize };
            if !labyrinth.grid.iter().any(|cell| cell.x == new_cell.x && cell.y == new_cell.y) && !path.iter().any(|cell| cell.x == new_cell.x && cell.y == new_cell.y) {
                let mut new_path = path.clone();
                new_path.push(new_cell.clone());
                if let Some(result) = solve_labirinth_backtracking(labyrinth, new_cell, new_path) {
                    return Some(result);
                }
            }
        }
    }
    None
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();   
    println!("Arguments: {:?}", args);

    let file = File::open(&args[1])?;
    println!("File opened successfully: {}", &args[1]);

    let mut labyrinth: Labyrinth = serde_json::from_reader(file)?;
    println!("Labyrinth loaded successfully");

    solve_labyrinth(&mut labyrinth);
    print_labyrinth(&labyrinth);

    Ok(())
}