use std::env::args;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use tp_individual_2::io::*;
static CLIENT_ARGS: usize = 3;

fn main() -> Result<(), ()> {
    let argv = args().collect::<Vec<String>>();
    if argv.len() != CLIENT_ARGS {
        eprintln!("Error: \"Cantidad de argumentos inválido\"");
        return Ok(());
    }
    let partes: Vec<&str> = argv[1].split(':').collect();
    let ip = partes[0].to_owned();
    let address = ip + ":" + partes[1];
    let nombre_archivo = &argv[2];
    match client_run(&address, nombre_archivo) {
        Ok(_) => {}
        Err(error) => {
            eprint!("ERROR: \"{}\"", error);
        }
    }
    Ok(())
}

///Recibe una dirección y un archivo, se conecta a la dirección y envia todas las lineas del archivo
fn client_run(address: &str, nombre_archivo: &String) -> Result<(), Error> {
    let archivo = File::open(nombre_archivo)?;
    let reader = BufReader::new(archivo);
    let mut socket = TcpStream::connect(address)?;

    for linea in reader.lines() {
        let operacion = linea?;
        let mensaje = "OP".to_owned() + " " + &operacion + "\n";
        enviar_mensaje(&mensaje, &mut socket)?;
        let respuesta = recibir_mensaje(&mut socket)?;
        if respuesta != "OK\n" {
            eprint!("{}", respuesta);
        }
    }
    let fin = "GET\n".to_string();
    enviar_mensaje(&fin, &mut socket)?;
    let mensaje = recibir_mensaje(&mut socket)?;
    let tokens: Vec<&str> = mensaje.split_whitespace().collect();
    println!("{}", tokens[1]);
    Ok(())
}
