use std::io::Error;
use std::io::{Read, Write};
//10 = \n

///Recibe un mensaje y un socket, realiza las operaciones necesarias para poder enviar el mensaje
pub fn enviar_mensaje(mensaje: &String, socket: &mut impl Write) -> Result<(), Error> {
    //let size_be = (mensaje.len() as u32 + 1).to_be_bytes();
    //socket.write_all(&size_be)?;
    socket.write_all(mensaje.as_bytes())?;
    socket.write_all("\n".as_bytes())?;
    socket.flush()?;
    Ok(())
}

///Recibe un socket, devuelve un Ok(string) con el mensaje leido
pub fn recibir_mensaje(socket: &mut impl Read) -> Result<String, Error> {
    //let mut num_buffer = [0u8; 4];
    //socket.read_exact(&mut num_buffer)?;
    //let size = u32::from_be_bytes(num_buffer);
    let mut mensaje_buf = vec![0; 1024 as usize];
    //socket.read_exact(&mut mensaje_buf)?;
    //let mut n = 0;
    
    let mut n = 0;
    loop{
        let bytes_leidos = socket.read(&mut mensaje_buf)?;
        n += bytes_leidos;
        //if bytes_leidos == 0{break;}
        if mensaje_buf[n-1] == 10{break;}
        
        //println!("lo ultimo leido fue {} y el ultimo caracter es {:?}", bytes_leidos,mensaje_buf);
    }
    //println!("mensaje recibido {:?}", mensaje_buf);
    let mensaje_str = match std::str::from_utf8(&mensaje_buf[0..n]) {
        Ok(mensaje_str) => mensaje_str,
        Err(_) => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "\"Error de lectura\"",
            ));
        }
    };
    let mensaje = mensaje_str.to_owned();
    //let mensaje = mensaje + "\n";
    print!("EL MENSAJE QUE RECIBI FUE {}", mensaje);
    Ok(mensaje)
}

//use std::net::TcpStream;
use std::io::BufReader;
use std::io::BufRead;

pub fn recibir_mensaje2(socket: &mut impl Read) -> Result<String, Error> {
    let buf_reader = BufReader::new(socket);
    let message_vec: Vec<_> = buf_reader
        .lines()
        .map(|result| match result {
            Ok(x) => x,
            Err(_) => "".to_string(),
        })
        .take_while(|line| !line.is_empty())
        .collect();
    //message_vec.iter().for_each(|x| println!("{x}"));
    let mensaje = message_vec.join(" ");
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
        assert_eq!(mensaje, respuesta.trim());
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
