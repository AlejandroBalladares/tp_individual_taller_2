use std::env::args;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use tp_individual_2::protocol::*;
static CLIENT_ARGS: usize = 3;

fn main() -> Result<(), ()> {
    let argv = args().collect::<Vec<String>>();
    if argv.len() != CLIENT_ARGS {
        eprintln!("Error: \"Cantidad de argumentos inválido\"");
        return Ok(());
    }
    let partes: Vec<&str> = argv[1].split(':').collect();
    if partes.len() != 2 || partes[0].is_empty() || partes[1].is_empty() {
        eprintln!("Error: \"Dirección inválida (usar IP:PUERTO)\"");
        return Ok(());
    }
    let address = format!("{}:{}", partes[0], partes[1]);
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
        let mensaje = format!("OP {}\n", operacion);
        enviar_mensaje(&mensaje, &mut socket)?;
        let respuesta = recibir_mensaje(&mut socket)?;
        let respuesta = respuesta.trim();
        if respuesta != "OK" {
            eprintln!("{}", respuesta);
        }
    }
    let fin = "GET\n".to_string();
    enviar_mensaje(&fin, &mut socket)?;
    let mensaje = recibir_mensaje(&mut socket)?;
    let tokens: Vec<&str> = mensaje.split_whitespace().collect();
    if tokens.is_empty() {
        eprintln!("ERROR: no se pudo imprimir el valor");
    } else {
        println!("{}", tokens[1]);
    }
    Ok(())
}
