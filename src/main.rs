use core::Config;
use utils::Result;

mod core;
mod utils;

// pm add --space rust --path /home/jdorozco/Dev/rust
// pm list --space rust

fn main() -> Result<()> {
    /* let mut config = Config::start()?;
    println!("{:#?}", config);
    config.add_space("rust".to_string(), "/home/jdorozco/Dev/rust".to_string())?; */

    let config = Config::start()?;
    
    config.list_space("rust".to_string())?;

    Ok(())
}
