use std::io;
use std::io::Write;
use std::process::Command;

fn parser(input : &String ) -> Result < Vec<&str> , String  > {
    let mut args: Vec<&str> = input.split(' ').collect();
    let command=args[0];
    if command == "exit" {
      Err("Mr Stark I don´t want to die :/".to_string())
    } else {
      Ok(args)
    }
}
 
fn main() {
  loop {
    println!("gush! ");

    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let len=input.len()-1;
    input.truncate(len);

    if ! input.is_empty() {
        let mut args =  match parser(&input) {
          Ok(a) => { println!("{:?}",a);  a } ,
          Err(e) => { println!("{}",e); break; } ,
        };
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
   } //if
  }
}
