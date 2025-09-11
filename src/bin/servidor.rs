use std::env::args;
use std::io::Error;
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::fs::File;
use std::io::Write;
use tp_individual_2::calculadora::*;
use tp_individual_2::io::*;
static SERVER_ARGS: usize = 2;

use std::sync::mpsc;


    enum LogMessage {
        Info(String),
        Error(String),
    }


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
    let mut archivo = File::create("./logger.txt")?;
    thread::spawn(move || {
            for msg in rx {
                match msg {
                    LogMessage::Info(s) =>{
                        let _ = archivo.write_all(b"[INFO]: ");
                        let _ = archivo.write_all(s.as_bytes());
                        let _ = archivo.write_all(b"\n");
                    }
                    LogMessage::Error(s) => {
                        let _ = archivo.write_all(b"[Error]: ");
                        let _ = archivo.write_all(s.as_bytes());
                        let _ = archivo.write_all(b"\n");
                    }
                }
            }
        });
    let calculadora = Calculator::default();
    let mut handles: Vec<JoinHandle<()>> = vec![];
    let lock = Arc::new(Mutex::new(calculadora));
    let listener = TcpListener::bind(address)?;
    for client_stream in listener.incoming() {
        let calculadora_mutex = Arc::clone(&lock);
        let tx_clone = tx.clone();
        let cliente = client_stream?;
        let handle = thread::spawn(move || leer_operacion(cliente, calculadora_mutex, tx_clone));
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap()
    }
    Ok(())
}

///Lee los mensajes recibidos y realiza el calculo pertinente
fn leer_operacion(mut socket: TcpStream, calculadora: Arc<Mutex<Calculator>>, logger: std::sync::mpsc::Sender<LogMessage>) {
    loop {
        let mensaje = match leer(&mut socket) {
            Ok(mensaje) => mensaje,
            Err(error) => {
                let mensaje_error = "ERROR: \"".to_owned() + &error.to_string() + "\"";
                println!("Error: {}", mensaje_error);
                logger.send(LogMessage::Error(format!("{}", mensaje_error))).unwrap();
                return;
            }
        };
        
        logger.send(LogMessage::Info(format!("{}", mensaje))).unwrap();
        if mensaje == "GET" {
            break;
        }
        let operation = match Operation::from_str(&mensaje) {
            Ok(operation) => operation,
            Err(error) => {
                let mensaje_error = "ERROR: \"".to_owned() + error + "\"";
                logger.send(LogMessage::Error(format!("{}", mensaje_error))).unwrap();
                let _ = enviar_mensaje(mensaje_error, &mut socket);
                continue;
            }
        };
        let mut calculadora = match calculadora.lock() {
            Ok(calculadora) => calculadora,
            Err(error) => {
                let mensaje_error = "ERROR: \"".to_owned() + &error.to_string() + "\"";
                logger.send(LogMessage::Info(format!("{}", mensaje_error))).unwrap();
                //println!("Mensaje de error: {}", mensaje_error);
                return;
            }
        };
        calculadora.apply(operation);
        let _ = enviar_mensaje("OK".to_string(), &mut socket);
        logger.send(LogMessage::Info(format!("{}", "OK"))).unwrap();
    }
    finalizar(socket, calculadora, logger);
    //logger.send(LogMessage::Error("Shutting down...".to_string())).unwrap();
    //drop(logger); // Close the sending end of the channel
}

///Imprime el valor actual de la calculadora y lo envia al cliente que lo pidio
fn finalizar(mut socket: TcpStream, calculadora: Arc<Mutex<Calculator>>, logger: std::sync::mpsc::Sender<LogMessage>) {
    let valor = match calculadora.lock() {
        Ok(mutex) => mutex.value() as u32,
        Err(error) => {
            let mensaje_error = "ERROR: \"".to_owned() + &error.to_string() + "\"";
            logger.send(LogMessage::Error(format!("{}", mensaje_error))).unwrap();
            let _ = enviar_mensaje(mensaje_error, &mut socket);
            return;
        }
    };
    println!("VALUE {}", valor);
    
    let mensaje = "VALUE ".to_owned() + &valor.to_string();
    logger.send(LogMessage::Info(format!("{}", mensaje))).unwrap();
    let _ = enviar_mensaje(mensaje, &mut socket);
}
