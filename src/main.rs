use std::fs;

fn main() -> Result<(), std::io::Error> {
    let content = read_file("assets/demo.csv");
    match content {
        Ok(content) => println!("{}", content),
        Err(e) => println!("Error: {}", e),
    }
    // 同上功能，语法糖写法：
    let content = read_file("assets/demo.csv")?;
    println!("{}", content);
    Ok(())
}

fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}
