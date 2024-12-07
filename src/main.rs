mod day01;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = day01::run()?;
    println!("Total Distance: {} ", result);
    Ok(())
}
