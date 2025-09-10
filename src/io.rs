use std::net::{TcpStream};
use std::io::{Read, Write};
use std::io::Error;

pub fn enviar_operacion(mensaje: String, socket: &mut TcpStream) -> Result<(), Error> {
    let size_be = (mensaje.len() as u32).to_be_bytes();
    let _ = socket.write(&size_be)?;
    let _ = socket.write(&mensaje.as_bytes())?;
    Ok(())
}

pub fn leer(socket: &mut TcpStream) -> Result<String, Error> {
    let mut num_buffer = [0u8; 4];
    let _ = socket.read_exact(&mut num_buffer)?;
    // Una vez que leemos los bytes, los convertimos a un u32
    let size = u32::from_be_bytes(num_buffer);

    // Creamos un buffer para el nombre
    let mut mensaje_buf = vec![0; size as usize];
    let _ = socket.read_exact(&mut mensaje_buf)?;
    // Convierto de bytes a string.
    let mensaje_str = std::str::from_utf8(&mensaje_buf).expect("Error al leer nombre");
    let mensaje = mensaje_str.to_owned();
    Ok(mensaje)
}
