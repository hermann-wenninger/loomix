use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
 let file_path = Path::new("data/data.txt");
 let content = fs::read_to_string(file_path)?;
 println!("File content: {}", content);
 Ok(())
 //why OK(())?
}
