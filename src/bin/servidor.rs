use std::env::args;
use std::fs::File;
use std::io::Error;
//use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use tp_individual_2::calculadora::*;
use tp_individual_2::io::*;
use tp_individual_2::logger::*;
//use std::sync::{MutexGuard, PoisonError};
static SERVER_ARGS: usize = 2;

use std::sync::mpsc;

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
            thread::spawn(move || leer_operacion(cliente, calculadora_mutex, &mut tx_clone));
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap()
    }
    Ok(())
}

///Lee los mensajes recibidos y realiza el calculo pertinente
fn leer_operacion(
    mut socket: TcpStream,
    calculadora: Arc<Mutex<Calculator>>,
    logger: &mut std::sync::mpsc::Sender<LogMessage>,
) {
    loop {
        let mensaje = match leer(&mut socket) {
            Ok(mensaje) => mensaje,
            Err(error) => {
                let error = "ERROR: \"".to_owned() + &error.to_string() + "\"";
                return error_irrecuperable(error, logger);
            }
        };
        let mensaje_log = mensaje.to_owned();
        let _ = logger.send(LogMessage::Info(mensaje_log));
        if mensaje == "GET" {
            break;
        }
        let operation = match Operation::from_str(&mensaje) {
            Ok(operation) => operation,
            Err(error) => {
                let mensaje_error = "ERROR: \"".to_owned() + error + "\"";
                error_recuperable(mensaje_error, logger, &mut socket);
                continue;
            }
        };
        let mut calculadora = match calculadora.lock() {
            Ok(calculadora) => calculadora,
            Err(error) => {
                let mensaje_error = "ERROR: \"".to_owned() + &error.to_string() + "\"";
                let _ = logger.send(LogMessage::Info(mensaje_error));
                //println!("Mensaje de error: {}", mensaje_error);
                return;
            }
        };
        calculadora.apply(operation);
        let _ = enviar_mensaje(&"OK".to_string(), &mut socket);
        logger.send(LogMessage::Info("OK".to_string())).unwrap();
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
            let mensaje_error = "ERROR: \"".to_owned() + &error.to_string() + "\"";
            println!("Error: {}", mensaje_error);
            let _ = logger.send(LogMessage::Error(mensaje_error));
            return;
        }
    };
    println!("VALUE {}", valor);

    let mensaje = "VALUE ".to_owned() + &valor.to_string();

    let mensaje_log = mensaje.to_owned();
    let _ = logger.send(LogMessage::Info(mensaje_log));
    let _ = enviar_mensaje(&mensaje, &mut socket);
}

fn error_recuperable(
    mensaje: String,
    logger: &mut std::sync::mpsc::Sender<LogMessage>,
    socket: &mut TcpStream,
) {
    let _ = enviar_mensaje(&mensaje, socket);
    let _ = logger.send(LogMessage::Error(mensaje));
}

fn error_irrecuperable(mensaje: String, logger: &mut std::sync::mpsc::Sender<LogMessage>) {
    println!("Error: {}", mensaje);
    let _ = logger.send(LogMessage::Error(mensaje));
}
