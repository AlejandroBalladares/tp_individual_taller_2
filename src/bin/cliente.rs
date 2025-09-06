//! Se conecta mediante TCP a la dirección asignada por argv.
//! Lee lineas desde stdin y las manda mediante el socket.
use tp_individual_2::alumno;
use alumno::Alumno;
use std::env::args;
use std::net::TcpStream;
use std::io::{Read, Write};

static CLIENT_ARGS: usize = 3;

fn main() -> Result<(), ()> {
    let argv = args().collect::<Vec<String>>();
    if argv.len() != CLIENT_ARGS {
        println!("Cantidad de argumentos inválido");
        let app_name = &argv[0];
        println!("{:?} <host> <puerto>", app_name);
        return Err(());
    }

    let address = argv[1].clone() + ":" + &argv[2];
    println!("Conectándome a {:?}", address);

    client_runn(&address).unwrap();
    Ok(())
}

fn client_runn(address: &str)-> std::io::Result<()>{
    let mut socket = TcpStream::connect(address)?;
    let mensaje = "Hola servidor";
    println!("Enviando: {:?}", mensaje);

    let size_be = (mensaje.len() as u32).to_be_bytes();
    socket.write(&size_be)?;
    socket.write(mensaje.as_bytes())?;

    Ok(())
}

/*
// Debo convertir una variable u32 a una cadena de bytes big_endian
        // En algunos lenguajes se convierte a big-endian con la funcion ntohl/ntohs
        let padron_be = self.padron.to_be_bytes();
        stream.write(&padron_be)?;
        // Es importante aclarar el tipo de variable de len(), sino será usize
        let size_be = (self.nombre.len() as u32).to_be_bytes();
        stream.write(&size_be)?;
        stream.write(&self.nombre.as_bytes())?;
        Ok(()) */
fn client_run(address: &str) -> std::io::Result<()> {
    // Vamos a mandar datos crudos
    let mut socket = TcpStream::connect(address)?;
    let alumno = Alumno {nombre: "Pepe Muleiro".to_owned(), padron: 80880};
    println!("Enviando: {:?}", alumno);
    alumno.write_to(&mut socket).unwrap();
    Ok(())
}
