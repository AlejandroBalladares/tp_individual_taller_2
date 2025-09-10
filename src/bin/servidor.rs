use std::env::args;
use std::io::Error;
use std::io::{Write};
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use tp_individual_2::calculadora::*;
use tp_individual_2::io::*;


static SERVER_ARGS: usize = 2;

fn main() -> Result<(), ()> {
    let argv = args().collect::<Vec<String>>();
    if argv.len() != SERVER_ARGS {
        eprintln!("Error: \"Cantidad de argumentos inválido\"");
        let app_name = &argv[0];
        println!("{:?} <host> <puerto>", app_name);
        return Ok(());
    }
    let address = "0.0.0.0:".to_owned() + &argv[1];
    match server_run(&address) {
        Ok(_) => {}
        Err(error) => {
            eprint!("Error: \"{}\"", error);
        }
    }
    Ok(())
}

fn server_run(address: &str) -> Result<(), Error> {
    let calculadora = Calculator::default();
    let mut handles: Vec<JoinHandle<()>> = vec![];
    let lock = Arc::new(Mutex::new(calculadora));
    let listener = TcpListener::bind(address)?;
    for client_stream in listener.incoming() {
        let calculadora_mutex = Arc::clone(&lock);
        let cliente = client_stream?;
        let handle = thread::spawn(move || leer_operacion(cliente, calculadora_mutex));
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap()
    }
    Ok(())
}

pub fn leer_operacion(mut stream: TcpStream, calculadora: Arc<Mutex<Calculator>>) {
    loop {
        let mensaje = match leer(&mut stream){
            Ok(mensaje) =>{mensaje}
            Err(_e) =>{
                continue;
            }
        };
        if mensaje == "GET" {
            break;
        }
        let operation = match Operation::from_str(&mensaje) {
            Ok(operation) => operation,
            Err(_error) => {
                //let _ = enviar(error, &mut stream);
                continue;
            }
        };
        let mut calculadora = match calculadora.lock() {
            Ok(calculadora) => calculadora,
            Err(e) => {
                print!("Error: \"{}\"", e); //corregir
                continue; //cual de las 2 irá?
                //return;
            }
        };
        calculadora.apply(operation);
    }
    let valor = match calculadora.lock() {
        Ok(mutex) => mutex.value() as u32,
        Err(e) => {
            eprint!("Error: \"{}\"", e);
            return;
        }
    };
    println!("{}", valor);
    let _ = stream.write(&valor.to_be_bytes());
}

