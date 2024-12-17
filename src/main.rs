use config::Config;
use utils::Result;
// use utils::Result;

mod actions;
mod config;
mod utils;

// use walkdir::WalkDir;

// fn read_dir() -> Result<()> {
//     for entry in WalkDir::new("/home/jdorozco/Dev").max_depth(1) {
//         println!("{:?}", entry?.path())
//     }

//     Ok(())
// }

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
