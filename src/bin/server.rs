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
    if partes.len() != 2 || partes[0].is_empty() || partes[1].is_empty() {
        eprintln!("Error: \"Dirección inválida (usar IP:PUERTO)\"");
        return Ok(());
    }
    let address = format!("{}:{}", partes[0], partes[1]);
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
    mut calculadora: Arc<Mutex<Calculator>>,
    logger: &mut std::sync::mpsc::Sender<LogMessage>,
) {
    loop {
        let mensaje = match recibir_mensaje(&mut socket) {
            Ok(mensaje) => mensaje,
            Err(error) => {
                let mensaje_error = format!("ERROR \"{}\"\n", error);
                return error_irrecuperable(mensaje_error, logger);
            }
        };
        let _ = logger.send(LogMessage::Info(mensaje.to_owned()));
        let tokens: Vec<&str> = mensaje.split_whitespace().collect();
        if tokens.is_empty() {
            let mensaje_error = "ERROR \"mensaje vacío\"\n".to_string();
            responder(mensaje_error, logger, &mut socket, ERROR);
            continue;
        }
        match tokens[0] {
            "GET" => return finalizar(&mut socket, &mut calculadora, logger),
            "OP" => match aplicar_operacion(&mut socket, &mut calculadora, logger, mensaje) {
                Ok(()) => {}
                Err(_e) => {
                    break;
                }
            },
            _ => {
                let mensaje_error = "ERROR: \"unexpected message\"\n".to_string();
                responder(mensaje_error, logger, &mut socket, ERROR);
            }
        }
    }
}

///Imprime el valor actual de la calculadora y lo envia al cliente que lo pidio
fn finalizar(
    socket: &mut TcpStream,
    calculadora: &mut Arc<Mutex<Calculator>>,
    logger: &mut std::sync::mpsc::Sender<LogMessage>,
) {
    let valor = match calculadora.lock() {
        Ok(mutex) => mutex.value() as u32,
        Err(error) => {
            let mensaje_error = format!("ERROR \"{}\"\n", error);
            return error_irrecuperable(mensaje_error, logger);
        }
    };
    println!("{}", valor);
    let mensaje = format!("VALUE {}", &valor.to_string());
    responder(mensaje, logger, socket, INFO);
}

fn aplicar_operacion(
    socket: &mut TcpStream,
    calculadora: &mut Arc<Mutex<Calculator>>,
    logger: &mut std::sync::mpsc::Sender<LogMessage>,
    mensaje: String,
) -> Result<(), Error> {
    let operation = match Operation::from_str(&mensaje) {
        Ok(operation) => operation,
        Err(error) => {
            let mensaje_error = format!("ERROR \"{}\"\n", error);
            responder(mensaje_error, logger, socket, ERROR);
            return Ok(());
        }
    };
    let mut calculadora = match calculadora.lock() {
        Ok(calculadora) => calculadora,
        Err(error) => {
            let mensaje_error = format!("ERROR \"{}\"\n", error);
            error_irrecuperable(mensaje_error, logger);
            return Err(Error::new(std::io::ErrorKind::InvalidData, "\"Error\""));
        }
    };
    calculadora.apply(operation);
    responder("OK\n".to_string(), logger, socket, INFO);
    Ok(())
}
