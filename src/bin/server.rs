use std::env::args;
use std::fs::File;
use std::io::Error;
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};
use tp_individual_2::calculadora::*;
use tp_individual_2::io::*;
use tp_individual_2::logger::*;
static SERVER_ARGS: usize = 2;
static ERROR: bool = true;
static INFO: bool = false;

fn main() -> Result<(), ()> {
    let argv = args().collect::<Vec<String>>();
    if argv.len() != SERVER_ARGS {
        eprintln!("Error: \"Cantidad de argumentos inválido\"");
        return Ok(());
    }
    let partes: Vec<&str> = argv[1].split(':').collect();
    let ip = partes[0].to_owned();
    let address = ip + ":" + partes[1];
    match server_run(&address) {
        Ok(_) => {}
        Err(error) => {
            eprint!("Error: \"{}\"", error);
        }
    }
    Ok(())
}

///Se conecta a una dirección para correr el servidor, recibe mensajes de varios clientes distintos
/// y realiza las operaciones pertinentes
fn server_run(address: &str) -> Result<(), Error> {
    let (tx, rx) = mpsc::channel::<LogMessage>();
    let archivo = File::create("./log.txt")?;
    let mut logger = Logger::new(rx, archivo);
    thread::spawn(move || {
        loop {
            logger.loggear_info();
        }
    });
    let calculadora = Calculator::default();
    let mut handles: Vec<JoinHandle<()>> = vec![];
    let lock = Arc::new(Mutex::new(calculadora));
    let listener = TcpListener::bind(address)?;
    for client_stream in listener.incoming() {
        let calculadora_mutex = Arc::clone(&lock);
        let mut tx_clone = tx.clone();
        let cliente = client_stream?;
        let handle =
            thread::spawn(move || handle_conection(cliente, calculadora_mutex, &mut tx_clone));
        handles.push(handle);
    }
    for handle in handles {
        match handle.join() {
            Ok(_) => {}
            Err(e) => {
                eprint!("Error: \"{:?}\"", e);
            }
        }
    }
    Ok(())
}

///Lee los mensajes recibidos y realiza el calculo pertinente
fn handle_conection(
    mut socket: TcpStream,
    calculadora: Arc<Mutex<Calculator>>,
    logger: &mut std::sync::mpsc::Sender<LogMessage>,
) {
    loop {
        let mensaje = match recibir_mensaje(&mut socket) {
            Ok(mensaje) => mensaje,
            Err(error) => {
                let error = "ERROR: \"".to_owned() + &error.to_string() + "\"" + "\n";
                return error_irrecuperable(error, logger);
            }
        };
        let _ = logger.send(LogMessage::Info(mensaje.to_owned()));
        let tokens: Vec<&str> = mensaje.split_whitespace().collect();
        match tokens[0] {
            "GET" => break,
            "OP" => {}
            _ => {
                let mensaje_error = "ERROR: \"unexpected message\"".to_string() + "\n";
                responder(mensaje_error, logger, &mut socket, ERROR);
            }
        }

        let operation = match Operation::from_str(&mensaje) {
            Ok(operation) => operation,
            Err(error) => {
                let mensaje_error = "ERROR: \"".to_owned() + error + "\"" + "\n";
                responder(mensaje_error, logger, &mut socket, ERROR);
                continue;
            }
        };
        let mut calculadora = match calculadora.lock() {
            Ok(calculadora) => calculadora,
            Err(error) => {
                let mensaje_error = "ERROR: \"".to_owned() + &error.to_string() + "\"" + "\n";
                return error_irrecuperable(mensaje_error, logger);
            }
        };
        calculadora.apply(operation);
        responder("OK\n".to_string(), logger, &mut socket, INFO);
    }
    finalizar(socket, calculadora, logger);
}

///Imprime el valor actual de la calculadora y lo envia al cliente que lo pidio
fn finalizar(
    mut socket: TcpStream,
    calculadora: Arc<Mutex<Calculator>>,
    logger: &mut std::sync::mpsc::Sender<LogMessage>,
) {
    let valor = match calculadora.lock() {
        Ok(mutex) => mutex.value() as u32,
        Err(error) => {
            let mensaje_error = "ERROR: \"".to_owned() + &error.to_string() + "\"" + "\n";
            return error_irrecuperable(mensaje_error, logger);
        }
    };
    println!("{}", valor);
    let mensaje = "VALUE ".to_owned() + &valor.to_string() + "\n";
    responder(mensaje, logger, &mut socket, INFO);
}
