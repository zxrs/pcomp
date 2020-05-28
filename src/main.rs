use std::io::{self, prelude::*};
use std::time::Instant;
use anyhow::Result;

mod app;
use app::App;

fn main() {
    let start = Instant::now();

    if let Err(e) = app_start() {
        println!("{}", e);
    }
    
    println!("Total Time: {:?}", start.elapsed());
    pause();
}

fn app_start() -> Result<()> {
    let app = App::new()?;
    app.start()?;
    Ok(())
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}
