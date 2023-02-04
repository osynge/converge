mod config;

fn main() -> Result<(), u8> {
    let config = crate::config::get_config(std::env::args_os(), std::env::vars_os());

    match config {
        Ok(cfg) => {
            println!("config={:#?}", cfg);
            Ok(())
        }
        Err(err) => {
            println!("Failed to load config {:#?}", err);
            Err(3)
        }
    }
}
