use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Building dex...");
    let start = Instant::now();
    dex_builder::build_dex("pokemon", "moves", "textures", "output/dex.bin")?;
    println!("Finished in {}ms!", start.elapsed().as_millis());
    Ok(())
}