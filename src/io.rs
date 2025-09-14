use std::io::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

///Recibe un mensaje y un socket, realiza las operaciones necesarias para poder enviar el mensaje
pub fn enviar_mensaje(mensaje: &String, socket: &mut TcpStream) -> Result<(), Error> {
    let size_be = (mensaje.len() as u32).to_be_bytes();
    let _ = socket.write(&size_be)?;
    let _ = socket.write(mensaje.as_bytes())?;
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
        //let valor = resultado.unwrap();
        //assert_eq!(valor, mensaje);
        let mut tupla= listener.accept().unwrap();
        let resultado = recibir_mensaje(&mut tupla.0).unwrap();
        assert_eq!(resultado, mensaje);
    }
    /*
    #[test]
    fn recibir_un_mensaje_pasa() {
        let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
        let mut socket = TcpStream::connect("127.0.0.1:8080").unwrap();
        let mensaje = "Soy un mensaje".to_string();

        let _ = enviar_mensaje(&mensaje, &mut socket);

        let mut tupla= listener.accept().unwrap();
        let resultado = recibir_mensaje(&mut tupla.0).unwrap();
        assert_eq!(resultado, mensaje);
        //let valor = resultado.unwrap();
        //assert_eq!(valor, mensaje);
    } */
}
/*
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Read, Write};

// A mock object implementing Read and Write for testing purposes
struct MockStream {
    read_data: Vec<u8>,
    write_data: Vec<u8>,
}

impl Read for MockStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_to_read = self.read_data.len().min(buf.len());
        buf[..bytes_to_read].copy_from_slice(&self.read_data[..bytes_to_read]);
        self.read_data.drain(..bytes_to_read);
        Ok(bytes_to_read)
    }
}

impl Write for MockStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write_data.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

    #[test]
    fn vector_con_los_datos_correctos_pasa() {
        let mut mock_stream = MockStream {
        read_data: b"hello".to_vec(),
        write_data: Vec::new(),
        };
        let mensaje = "Hello world".to_string();
        let valor = enviar_mensaje(mensaje,mock_stream);
        assert!(valor.is_ok());
    }
}
*/
