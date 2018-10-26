use std::io;
use std::io::Write;
use std::process::Command;
use  std::str::from_utf8;


fn main() {
  loop {
    println!("gush! ");

    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let len=input.len()-1;
    input.truncate(len);
    let mut args: Vec<&str> = input.split(' ').collect();

    let command=args.remove(0);
    let output=Command::new(&command)
            .args(&args)
            .output()
            .expect("Lee la documentacion, animal");
    
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write( &output.stdout );

    let stderr = io::stderr();
    let mut handle = stderr.lock();
    handle.write( &output.stderr );
  }
}
