use crate::io::*;
use std::fs::File;
use std::io::Write;
use std::net::TcpStream;

pub enum LogMessage {
    Info(String),
    Error(String),
}

pub struct Logger {
    canal: std::sync::mpsc::Receiver<LogMessage>,
    archivo: File,
}

impl Logger {
    pub fn new(canal: std::sync::mpsc::Receiver<LogMessage>, archivo: File) -> Logger {
        Logger { canal, archivo }
    }
    pub fn loggear_info(&mut self) {
        let mensaje = match self.canal.recv() {
            Ok(mensaje) => mensaje,
            Err(_) => {
                return;
            }
        };
        match mensaje {
            LogMessage::Info(s) => {
                let _ = self.archivo.write_all(b"[INFO]: ");
                let _ = self.archivo.write_all(s.as_bytes());
            }
            LogMessage::Error(s) => {
                let _ = self.archivo.write_all(b"[Error]: ");
                let _ = self.archivo.write_all(s.as_bytes());
            }
        }
        let _ = self.archivo.write_all(b"\n");
    }
}

pub fn responder(
    mensaje: String,
    logger: &mut std::sync::mpsc::Sender<LogMessage>,
    socket: &mut TcpStream,
    error: bool,
) {
    let _ = enviar_mensaje(&mensaje, socket);
    if error {
        let _ = logger.send(LogMessage::Error(mensaje));
    } else {
        let _ = logger.send(LogMessage::Info(mensaje));
    }
}

pub fn error_irrecuperable(mensaje: String, logger: &mut std::sync::mpsc::Sender<LogMessage>) {
    print!("{}", mensaje);
    let _ = logger.send(LogMessage::Error(mensaje));
}
