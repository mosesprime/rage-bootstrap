use std::{path::{Path, PathBuf}, time::SystemTime};

use rage_bootstrap::{logger::init_logger, parse::Parser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger().expect("unable to start logger");
    let args: Vec<String> = std::env::args().collect();
    let src_path = match &args.get(1) {
        Some(p) => {
            let path = Path::new(p);
            if !path.exists() {
                log::error!("path does not exist");
                println!("{}", HELP_MSG);
                return Ok(());
            }
            PathBuf::from(path)
        },
        None => {
            log::error!("missing source path argument");
            println!("{}", HELP_MSG);
            return Ok(());
        },
    };
    log::info!("starting bootstrap");
    let start_time = SystemTime::now();

    log::debug!("loading {}", src_path.display());
    let content = std::fs::read_to_string(src_path)?;

    let parser = Parser::new(&content);
    let ast = parser.run()?;
    log::debug!("ast {:#?}", ast);

    log::info!("completed bootstrap in {} seconds", start_time.elapsed()?.as_secs_f64());
    Ok(())
}

const HELP_MSG: &'static str = "
rage-bootstrap <SRC_PATH>
    <SRC_PATH>: path to the compiler source code
";
