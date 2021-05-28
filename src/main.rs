use std::env;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::SystemTime;

// You should read the ${HISTFILE} env var instead ;)
const HISTORY_FILE: &str = "/Users/mendler/.zhistory";

fn timestamp() -> Result<u64, Box<dyn Error>> {
    let n = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    Ok(n.as_secs())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut history = OpenOptions::new()
        .create(true)
        .append(true)
        .open(HISTORY_FILE)?;

    if let Some(command) = env::args().nth(3) {
        writeln!(history, ": {}:0;{}", timestamp()?, command)?;
    };
    Ok(())
}
