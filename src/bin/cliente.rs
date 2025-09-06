//! Se conecta mediante TCP a la dirección asignada por argv.
//! Lee lineas desde stdin y las manda mediante el socket.

use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

static CLIENT_ARGS: usize = 4;

fn main() -> Result<(), ()> {
    let argv = args().collect::<Vec<String>>();
    if argv.len() != CLIENT_ARGS {
        println!("Cantidad de argumentos inválido");
        let app_name = &argv[0];
        println!("{:?} <host> <puerto>", app_name);
        return Err(());
    }

    //NO SE PUEDE USAR CLONE
    let address = argv[1].clone() + ":" + &argv[2];
    let nombre_archivo = &argv[3];
    println!("Conectándome a {:?}", address);
    client_runn(&address, nombre_archivo).unwrap();
    Ok(())
}

fn client_runn(address: &str, nombre_archivo: &String) -> std::io::Result<()> {
    let archivo = File::open(nombre_archivo)?; // Abre el archivo en modo solo lectura
    let reader = BufReader::new(archivo);
    let mut socket = TcpStream::connect(address)?;

    for linea in reader.lines() {
        // Cada `line` es un Result<String> que puede ser un error o la línea actual

        let mensaje = linea?;
        println!("Enviando: {:?}", mensaje);
        let size_be = (mensaje.len() as u32).to_be_bytes();
        socket.write(&size_be)?;
        socket.write(mensaje.as_bytes())?;
    }
    let fin = "Fin del archivo";
    let size_be = (fin.len() as u32).to_be_bytes();
    socket.write(&size_be)?;
    socket.write(fin.as_bytes())?;

    Ok(())
}
