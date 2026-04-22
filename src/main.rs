use std::fs::File;
use std::io::{self};
use zip::ZipArchive;

fn fibonaci(n : u8) -> u64 {
    if n==0 || n==1 {
        return n as u64;
    }
    else {
        return fibonaci(n-1) + fibonaci(n-2);
    }
}



fn main() -> io::Result<()> {
    // Open the ZIP file
    let file = File::open("DMD.zip")?;
    let mut archive = ZipArchive::new(file)?;

    let mut smallest_size = 9999999;

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        if file.size() < smallest_size {
            smallest_size = file.size();
        }
        println!("Filename: {}", file.name());
        println!("Size: {} bytes", file.size());
        println!("Compressed Size: {} bytes", file.compressed_size());
        if file.is_dir() {
            println!("This is a directory.");
        } else {
            println!("This is a file.");
        }
        println!();
    }

    println!("Fibonaci of 10 is: {}", fibonaci(10));

    println!("Smallest file: {}", smallest_size);
    Ok(())
}