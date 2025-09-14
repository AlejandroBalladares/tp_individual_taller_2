use std::io::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

///Recibe un mensaje y un socket, realiza las operaciones necesarias para poder enviar el mensaje
pub fn enviar_mensaje(mensaje: &String, socket: &mut TcpStream) -> Result<(), Error> {
    let size_be = (mensaje.len() as u32 + 1).to_be_bytes();
    let _ = socket.write(&size_be)?;
    let _ = socket.write(mensaje.as_bytes())?;
    let _ = socket.write("\n".as_bytes())?;

    Ok(())
}

///Recibe un socket, devuelve un Ok(string) con el mensaje leido
pub fn recibir_mensaje(socket: &mut TcpStream) -> Result<String, Error> {
    let mut num_buffer = [0u8; 4];
    socket.read_exact(&mut num_buffer)?;
    let size = u32::from_be_bytes(num_buffer);
    let mut mensaje_buf = vec![0; size as usize];
    socket.read_exact(&mut mensaje_buf)?;
    let mensaje_str = match std::str::from_utf8(&mensaje_buf) {
        Ok(mensaje_str) => mensaje_str,
        Err(_) => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "\"Error de lectura\"",
            ));
        }
    };
    let mensaje = mensaje_str.to_owned();
    Ok(mensaje)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{TcpListener, TcpStream};

    #[test]
    fn enviar_un_mensaje_pasa() {
        let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
        let mut socket = TcpStream::connect("127.0.0.1:8080").unwrap();
        let mensaje = "Soy un mensaje".to_string();

        let resultado = enviar_mensaje(&mensaje, &mut socket);
        assert!(resultado.is_ok());

        let mut tupla = listener.accept().unwrap();
        let resultado = recibir_mensaje(&mut tupla.0).unwrap();
        assert_eq!(resultado, mensaje);
    }
}
