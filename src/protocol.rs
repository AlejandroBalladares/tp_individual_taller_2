use std::io::Error;
use std::io::{Read, Write};
static FIN_DE_LINEA: u8 = 10;
static BUFFER_TAM: usize = 1024;

//10 = \n

///Recibe un mensaje y un socket, realiza las operaciones necesarias para poder enviar el mensaje
pub fn enviar_mensaje(mensaje: &String, socket: &mut impl Write) -> Result<(), Error> {
    socket.write_all(mensaje.as_bytes())?;
    //socket.write_all("\n".as_bytes())?;
    socket.flush()?;
    Ok(())
}

///Recibe un socket, devuelve un Ok(string) con el mensaje leido
pub fn recibir_mensaje(socket: &mut impl Read) -> Result<String, Error> {
    let mut mensaje_buf = vec![0; BUFFER_TAM];
    let mut n = 0;
    loop {
        let bytes_leidos = socket.read(&mut mensaje_buf)?;
        n += bytes_leidos;
        if mensaje_buf[n - 1] == FIN_DE_LINEA {
            break;
        }
    }
    let mensaje_str = match std::str::from_utf8(&mensaje_buf[0..n]) {
        Ok(mensaje_str) => mensaje_str,
        Err(_) => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "\"Error de lectura\"",
            ));
        }
    };
    let mensaje = mensaje_str.to_string();
    Ok(mensaje)
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let output = output.to_vec();
        let respuesta = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(mensaje, respuesta.trim());
    }
}
