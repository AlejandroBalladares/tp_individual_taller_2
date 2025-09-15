use std::io::Error;
use std::io::{Read, Write};

///Recibe un mensaje y un socket, realiza las operaciones necesarias para poder enviar el mensaje
pub fn enviar_mensaje(mensaje: &String, socket: &mut impl Write) -> Result<(), Error> {
    let size_be = (mensaje.len() as u32).to_be_bytes();
    let _ = socket.write(&size_be)?;
    let _ = socket.write(mensaje.as_bytes())?;
    Ok(())
}

///Recibe un socket, devuelve un Ok(string) con el mensaje leido
pub fn recibir_mensaje(socket: &mut impl Read) -> Result<String, Error> {
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
    use std::io::BufReader;
    use std::io::Cursor;
    #[test]
    fn enviar_un_mensaje_pasa() {
        let mensaje = "Soy un mensaje".to_string();
        let mut output = Vec::new();
        assert!(enviar_mensaje(&mensaje, &mut output).is_ok());
    }

    #[test]
    fn enviar_un_mensaje_pasa_2() {
        let mensaje = "Soy un mensaje".to_string();
        let mut output = Vec::new();
        enviar_mensaje(&mensaje, &mut output).unwrap();
        let output = output[4..].to_vec(); //ignoro el tamaño
        let respuesta = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(mensaje, respuesta);
    }

    #[test]
    fn leer_un_mensaje_pasa() {
        let contenido = "Soy un mensaje".to_string();
        let tam = (contenido.len() as u32).to_be_bytes();
        let mensaje = contenido.as_bytes();

        let mut resultado_vec = Vec::new();
        resultado_vec.extend_from_slice(&tam);
        resultado_vec.extend_from_slice(mensaje);

        let cursor = Cursor::new(resultado_vec);
        let mut reader = BufReader::new(cursor);
        let respuesta = recibir_mensaje(&mut reader).unwrap();
        assert_eq!(respuesta, contenido);
    }
}
