use std::io;
use std::io::Write;
use std::io::Read;
use std::error::Error;   
use std::process::Command;
use std::process::Stdio;
use std::process::exit;

#[allow(dead_code)]
#[derive(Debug)]
struct Comando<'a> {
    binario: String,
    argumentos: Vec<&'a str>,
}  //Comando

fn parserv2 (comandos :  &String , separador: String  ) -> Vec<Comando >  {
    let mut lote : Vec<Comando> = Vec::new();
    let commands: Vec<&str> = comandos.split(&separador).collect();
    
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
fn test_parserv2_sequential() {
   let comandos = "ls -l; pwd ; ps -fea ".to_string();
   let lote =parserv2(&comandos, ";".to_string());
   assert!(lote.len() == 3);
   assert!(lote[0].binario == "ls" );
   assert!(lote[0].argumentos[0] == "-l" );
   assert!(lote[2].binario == "ps" );
   assert!(lote[2].argumentos[0] == "-fea" );
}

#[test]
fn test_parserv2_pipe() {
   let comandos = "ls -l  | wc -l ".to_string();
   let lote =parserv2(&comandos, "|".to_string());
   assert!(lote.len() == 2);
   assert!(lote[0].binario == "ls" );
   assert!(lote[0].argumentos[0] == "-l" );
   assert!(lote[1].binario == "wc" );
   assert!(lote[1].argumentos[0] == "-l" );
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

fn ejecutar_pipe(pipe: &Vec<Comando> ) {
        let comando= &pipe[0];
        let  child=Command::new(&comando.binario)
            .args(&comando.argumentos)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Lee la documentacion, animal");
        let output= child.wait_with_output().expect("Failed to wait child");
        let salida= output.stdout;
//arrancar siguiente comando de la tuberia
        let comando_despues= &pipe[1];
        let  child_despues=match Command::new(&comando_despues.binario)
            .args(&comando_despues.argumentos)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn() {
          Err(why) => panic!("couldn't spawn wc: {}", why.description()),
          Ok(child_despues) => child_despues,
        };
//tomar entrada estandar del siguiente comando
        match child_despues.stdin.unwrap().write_all(&salida) {
        Err(why) => panic!("couldn't write to stdin: {}",
                           why.description()),
           Ok(_) => println!("sent to {}", comando_despues.binario ),
        }
        let mut s = String::new();
       match child_despues.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read stdout: {}",
                           why.description()),
        Ok(_) => print!("{} responded with:\n{}", comando_despues.binario , s),
       }
} //ejecutar_pipe

#[test]
fn test_ejecutar_pipe( ) {
  let pipe : Vec<Comando> = vec![ 
     Comando { binario: "ls".to_string(), argumentos: vec!["-l"] }, 
     Comando { binario: "wc".to_string(), argumentos: vec!["-l"] },
  ];
  ejecutar_pipe(&pipe);
} //test_ejecutar_lote_un_comando 

fn main() {
  loop {
    println!("gush! ");

    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let len=input.len()-1;
    input.truncate(len);

    if ! input.is_empty() {
        if input.contains("|") { 
           let  pipe =   parserv2(&input,"|".to_string()) ;
           ejecutar_pipe(&pipe);
        } else if input.contains("exit")  {
           exit(0);
        } else {
           let  lote =   parserv2(&input,";".to_string()) ;
           ejecutar_lote(&lote);
        }
   } //if
  }
}
