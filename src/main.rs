use std::io;
use std::io::Write;
use std::process::Command;

#[allow(dead_code)]
#[derive(Debug)]
struct Comando<'a> {
    binario: String,
    argumentos: Vec<&'a str>,
    
}

fn parserv2(comandos : &String ) ->  Vec<Comando>  {
    let mut lote : Vec<Comando> = Vec::new();
    let commands: Vec<&str> = comandos.split(';').collect();
    
    for c in commands {
         let mut args : Vec<&str> = c.split_whitespace().collect();
         let ejecutable = args.remove(0);
         let mut comando = Comando {
             binario: ejecutable.to_string(),
             argumentos: args,
         };
         lote.push(comando);
         
    }
    lote
}

#[test]
fn test_parserv2() {
   let comandos = "ls -l; pwd ; ps -fea ".to_string();
   let lote =parserv2(&comandos);
   assert!(lote.len() == 3);
   assert!(lote[0].binario == "ls" );
   assert!(lote[0].argumentos[0] == "-l" );
   assert!(lote[2].binario == "ps" );
   assert!(lote[2].argumentos[0] == "-fea" );
}

fn ejecutar_lote(lote: &Vec<Comando> ) {
  for comando in lote {

        let mut child=Command::new(&comando.binario)
            .args(&comando.argumentos)
            .spawn()
            .expect("Lee la documentacion, animal");
        let output= child.wait_with_output().
                         expect("Failed to wait child");
  
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write( &output.stdout );

        let stderr = io::stderr();
        let mut handle = stderr.lock();
        handle.write( &output.stderr );
  }//for
}//ejecutar_lote

#[test]
fn test_ejecutar_lote_un_comando( ) {
  let lote : Vec<Comando> = vec![ 
     Comando { binario: "ls".to_string(), argumentos: vec!["-l"] }, 
  ];
  ejecutar_lote(&lote);
} //test_ejecutar_lote_un_comando 

#[test]
fn test_ejecutar_lote_tres_comandos( ) {
  let lote : Vec<Comando> = vec![ 
     Comando { binario: "ls".to_string(), argumentos: vec!["-l"] }, 
     Comando { binario: "pwd".to_string(), argumentos: vec![] }, 
     Comando { binario: "ps".to_string(), argumentos: vec!["-a"] },
  ];
  ejecutar_lote(&lote);
} //test_ejecutar_lote_tres_comandos

fn main() {
  loop {
    println!("gush! ");

    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let len=input.len()-1;
    input.truncate(len);

    if ! input.is_empty() {
        let  lote =   parserv2(&input) ;
        ejecutar_lote(&lote);
   } //if
  }
}
