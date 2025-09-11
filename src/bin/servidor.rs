use std::env::args;
use std::io::Error;
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use tp_individual_2::calculadora::*;
use tp_individual_2::io::*;
static SERVER_ARGS: usize = 3;

fn main() -> Result<(), ()> {
    let argv = args().collect::<Vec<String>>();
    if argv.len() != SERVER_ARGS {
        eprintln!("Error: \"Cantidad de argumentos inválido\"");
        let app_name = &argv[0];
        println!("{:?} <host> <puerto>", app_name);
        return Ok(());
    }
    let ip = argv[1].to_owned();
    let address = ip + ":" + &argv[2];
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
        let mensaje = match leer(&mut stream) {
            Ok(mensaje) => mensaje,
            Err(error) => {
                let mensaje_error = "ERROR: \"".to_owned() + &error.to_string() + "\"";
                println!("Mensaje de error: {}", mensaje_error);
                //let _ = enviar_mensaje(mensaje_error, &mut stream);
                return;
            }
        };
        if mensaje == "GET" {
            break;
        }
        let operation = match Operation::from_str(&mensaje) {
            Ok(operation) => operation,
            Err(error) => {
                let mensaje_error = "ERROR: \"".to_owned() + &error + "\"";
                println!("Mensaje de error: {}", mensaje_error);
                let _ = enviar_mensaje(mensaje_error, &mut stream);
                continue;
            }
        };
        let mut calculadora = match calculadora.lock() {
            Ok(calculadora) => calculadora,
            Err(error) => {
                let mensaje_error = "ERROR: \"".to_owned() + &error.to_string() + "\"";
                println!("Mensaje de error: {}", mensaje_error);
                let _ = enviar_mensaje(mensaje_error, &mut stream);
                continue; //cual de las 2 irá?
                //return;
            }
        };
        calculadora.apply(operation);
        let _ = enviar_mensaje("OK".to_string(), &mut stream);
    }
    let valor = match calculadora.lock() {
        Ok(mutex) => mutex.value() as u32,
        Err(error) => {
            let mensaje_error = "ERROR: \"".to_owned() + &error.to_string() + "\"";
            let _ = enviar_mensaje(mensaje_error, &mut stream);
            return;
        }
    };
    println!("VALUE {}", valor);
    let mensaje = "VALUE ".to_owned() + &valor.to_string();
    let _ = enviar_mensaje(mensaje, &mut stream);
}
