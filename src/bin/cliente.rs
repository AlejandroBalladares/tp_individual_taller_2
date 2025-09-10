use std::env::args;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use tp_individual_2::io::*;

static CLIENT_ARGS: usize = 4;
//static GET: String = "GET";
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
        println!("Enviando: {:?}", mensaje);
        let size_be = (mensaje.len() as u32).to_be_bytes();
        let _ = socket.write(&size_be)?;
        let _ = socket.write(mensaje.as_bytes())?;


        //Leo si la operación salió bien o dio error
        //let respuesta = leer(&mut socket)?;
        //println!("{}",respuesta);
    }
    let fin = "GET";
    let size_be = (fin.len() as u32).to_be_bytes();
    let _ = socket.write(&size_be)?;
    let _ = socket.write(fin.as_bytes())?;

    let mut num_buffer = [0u8; 4];
    socket.read(&mut num_buffer)?;

    let resultado = u32::from_be_bytes(num_buffer);
    println!("{}", resultado);

    Ok(())
}
