use std::io::{self, Write};

pub fn prompt_folder() -> io::Result<String> {
    print!("Enter folder path: ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(input.trim().to_string())
} 