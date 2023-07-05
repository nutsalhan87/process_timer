use std::{env, error::Error, process::Command, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();

    let mut arguments = env::args();
    arguments.next().expect("There is must be first argument - the name of the program");
    let cmd = arguments.next().expect("There is no command");
    let mut command = Command::new(cmd);
    command.args(arguments.collect::<Vec<String>>());
    let child = command.spawn()?.wait()?;
    
    let end_time = start_time.elapsed();
    
    println!();
    match child.code() {
        Some(code) => println!("Exited with status code: {code}"),
        None => println!("Process terminated by signal"),
    };
    println!("Execution time is {}μs or {}ms", end_time.as_micros(), end_time.as_millis());

    Ok(())
}
