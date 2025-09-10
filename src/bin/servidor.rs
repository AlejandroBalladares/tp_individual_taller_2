use std::env::args;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::io::Error;
use tp_individual_2::calculadora::*;

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
        let mut num_buffer = [0u8; 4];
        match stream.read_exact(&mut num_buffer) {
            Ok(line) => line,
            Err(error) => {
                eprintln!("Error: \"{}\"", error);
                break;
            }
        }
        // Una vez que leemos los bytes, los convertimos a un u32
        let size = u32::from_be_bytes(num_buffer);

        // Creamos un buffer para el nombre
        let mut mensaje_buf = vec![0; size as usize];
        match stream.read_exact(&mut mensaje_buf) {
            Ok(line) => line,
            Err(error) => {
                eprintln!("Error: \"{}\"", error);
                break;
            }
        }
        // Convierto de bytes a string.
        let mensaje_str = std::str::from_utf8(&mensaje_buf).expect("Error al leer nombre");
        let mensaje = mensaje_str.to_owned();
        if mensaje == "GET" {
            break;
        }
        let operation = match Operation::from_str(&mensaje) {
            Ok(operation) => operation,
            Err(error) => {
                eprintln!("Error: \"{}\"", error);
                continue;
            }
        };
        let mut calculadora = match calculadora.lock() {
            Ok(calculadora) => calculadora,
            Err(e) => {
                print!("Error: \"{}\"", e);
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
