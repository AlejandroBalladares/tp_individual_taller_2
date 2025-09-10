use std::env::args;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use tp_individual_2::io::*;
static CLIENT_ARGS: usize = 4;

fn main() -> Result<(), ()> {
    let argv = args().collect::<Vec<String>>();
    if argv.len() != CLIENT_ARGS {
        eprintln!("Error: \"Cantidad de argumentos inválido\"");
        let app_name = &argv[0];
        println!("{:?} <host> <puerto>", app_name);
        return Ok(());
    }
    let ip = argv[1].to_owned();
    let address = ip + ":" + &argv[2];
    let nombre_archivo = &argv[3];
    println!("Conectándome a {:?}", address);
    match client_run(&address, nombre_archivo) {
        Ok(_) => {}
        Err(error) => {
            eprint!("Error: \"{}\"", error);
            //return Ok(());
        }
    }
    Ok(())
}

fn client_run(address: &str, nombre_archivo: &String) -> Result<(), Error> {
    let archivo = File::open(nombre_archivo)?;
    let reader = BufReader::new(archivo);
    let mut socket = TcpStream::connect(address)?;

    for linea in reader.lines() {
        let operacion = linea?;
        let mensaje = "OP".to_owned() + " " + &operacion;
        //println!("El mensaje es {}", mensaje);
        enviar_operacion(mensaje, &mut socket)?;

        //Leo si la operación salió bien o dio error
        //let respuesta = leer(&mut socket)?;
        //println!("{}",respuesta);
    }
    let fin = "GET".to_string();
    enviar_operacion(fin, &mut socket)?;
    let mut num_buffer = [0u8; 4];
    socket.read(&mut num_buffer)?;

    let resultado = u32::from_be_bytes(num_buffer);
    println!("{}", resultado);

    Ok(())
}
