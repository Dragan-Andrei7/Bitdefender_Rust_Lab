use std::fs::File;
use std::io;
use serde::Deserialize;

#[derive(Debug, Deserialize)]

struct GridCell {
    x: usize,
    y: usize,    
}

#[derive(Debug, Deserialize)]

struct Labyrinth {
    width: usize,
    height: usize,
    start: GridCell,
    goal: GridCell,
    grid: Vec<GridCell>,
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
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

fn solve_labyrinth(labyrinth: &Labyrinth) {
    let mut current_position = labyrinth.start.clone();
    let mut path = vec![current_position.clone()];

    while current_position != labyrinth.goal {
        
        
        println!("Current position: ({}, {})", current_position.x, current_position.y);
        break; // Remove this break when implementing the actual pathfinding logic
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();   
    println!("Arguments: {:?}", args);

    let file = File::open(&args[1])?;
    println!("File opened successfully: {}", &args[1]);

    let labyrinth: Labyrinth = serde_json::from_reader(file)?;
    println!("Labyrinth loaded successfully");

    print_labyrinth(&labyrinth);
    Ok(())
}