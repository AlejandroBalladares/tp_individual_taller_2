use std::env::args;
use std::net::TcpListener;

use std::{io::Read, str::FromStr};

//use std::sync::{Arc};
//use std::thread::{self, JoinHandle};
//use std::sync::Mutex;

static SERVER_ARGS: usize = 2;

fn main() -> Result<(), ()> {
    let argv = args().collect::<Vec<String>>();
    if argv.len() != SERVER_ARGS {
        println!("Cantidad de argumentos inválido");
        let app_name = &argv[0];
        println!("{:?} <host> <puerto>", app_name);
        return Err(());
    }
    let address = "0.0.0.0:".to_owned() + &argv[1];
    match server_run(&address) {
        Ok(_) => {}
        Err(error) => {
            eprint!("Error: {}", error);
        }
    }
    Ok(())
}

fn server_run(address: &str) -> std::io::Result<()> {
    //let resultado = 0;
    let mut calculadora = Calculator::default();
    //let lock = Arc::new(Mutex::new(calculator)); //Arc::new(RwLock::new(calculator));

    let listener = TcpListener::bind(address)?;
    // accept devuelve una tupla (TcpStream, std::net::SocketAddr)
    for client_stream in listener.incoming() {
        //Acá debería agregar los hilos?
        leer_operacion(&mut client_stream.unwrap(), &mut calculadora)?;
        println!("El valor final es {}", calculadora.value);
    }
    Ok(())
}

// fn server_run(address: &str) -> std::io::Result<()> {
//     let listener = TcpListener::bind(address)?;
//     // accept devuelve una tupla (TcpStream, std::net::SocketAddr)
//     let connection = listener.accept()?;
//     let mut client_stream : TcpStream = connection.0;
//     // TcpStream implementa el trait Read, así que podemos trabajar como si fuera un archivo
//     handle_client(&mut client_stream)?;
//     Ok(())
// }

pub fn leer_operacion(stream: &mut dyn Read, calculadora: &mut Calculator) -> std::io::Result<()> {
    loop {
        let mut num_buffer = [0u8; 4];
        stream.read_exact(&mut num_buffer)?;

        // Una vez que leemos los bytes, los convertimos a un u32
        let size = u32::from_be_bytes(num_buffer);

        // Creamos un buffer para el nombre
        let mut mensaje_buf = vec![0; size as usize];
        stream.read_exact(&mut mensaje_buf)?;

        // Convierto de bytes a string.
        let mensaje_str = std::str::from_utf8(&mensaje_buf).expect("Error al leer nombre");
        let mensaje = mensaje_str.to_owned();
        //println!("el mensaje recibido fue {}", mensaje);
        if mensaje == "Fin del archivo" {
            break;
        }
        let operation = match Operation::from_str(&mensaje) {
            Ok(operation) => operation,
            Err(error) => {
                eprintln!("failed to parse line {}", error);
                continue;
            }
        };
        //let mut calculadora = calculadora.lock().unwrap();
        calculadora.apply(operation);
    }
    Ok(())
}

// A basic wrapping u8 calculator.=
//
// The possible values range from [0;256).
#[derive(Default)]
pub struct Calculator {
    value: u8,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Operation {
    Add(u8),
    Sub(u8),
    Mul(u8),
    Div(u8),
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the string into tokens separated by whitespace.
        let tokens: Vec<&str> = s.split_whitespace().collect();

        // Try to convert the vector into a statically-sized array of 2 elements, failing otherwise.
        let [operation, operand] = tokens.try_into().map_err(|_| "expected 2 arguments")?;

        // Parse the operand into an u8.
        let operand: u8 = operand.parse().map_err(|_| "operand is not an u8")?;

        match operation {
            "+" => Ok(Operation::Add(operand)),
            "-" => Ok(Operation::Sub(operand)),
            "*" => Ok(Operation::Mul(operand)),
            "/" => Ok(Operation::Div(operand)),
            _ => Err("unknown operation"),
        }
    }
}

impl Calculator {
    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn apply(&mut self, op: Operation) {
        match op {
            Operation::Add(operand) => self.value = self.value.wrapping_add(operand),
            Operation::Sub(operand) => self.value = self.value.wrapping_sub(operand),
            Operation::Mul(operand) => self.value = self.value.wrapping_mul(operand),
            Operation::Div(operand) => self.value = self.value.wrapping_div(operand),
        }
    }
}

/*

//! Lee operaciones de una lista de archivos, y las aplica.
//!
//! Una vez procesados todos los archivos, imprime el resultado final.
//!
//! ## Uso
//!
//! Para procesar todos los archivos de la carpeta `data/`, ejecutar:
//! ```bash
//! cargo run -- data/*
//! ```
//! El resultado esperado de una ejecución secuencial es 26.



pub fn main() {
    // `Args` is an iterator over the program arguments.
    let mut inputs = std::env::args();
    let mut handles: Vec<JoinHandle<()>> = vec![];
    // We skip the first argument, as its traditionally the path to the executable.
    inputs.next();

    // We maintain a *global* calculator for the entire program.
    let calculator = Calculator::default();
    let lock = Arc::new(Mutex::new(calculator)); //Arc::new(RwLock::new(calculator));

    // usar mutex , nos permite escribir un ARC .ya que un arc de un mutex se puede escribrir

    for input in inputs {
        // Open the input file.
        //println!("Nombre del archivo = {}", input);
        let file = File::open(input).expect("failed to open input file");

        //EN teoría no debo usar clone
        //EN teoría no debo usar clone
        //EN teoría no debo usar clone
        //EN teoría no debo usar clone
        //EN teoría no debo usar clone
        //EN teoría no debo usar clone
        let calc_clone = Arc::clone(&lock); //EN teoría no debo usar esto
        //EN teoría no debo usar clone
        //EN teoría no debo usar clone
        //EN teoría no debo usar esto
        //EN teoría no debo usar esto
        //EN teoría no debo usar esto
        //EN teoría no debo usar esto
        //EN teoría no debo usar esto

        let handle = thread::spawn(move || funcion_auxiliar(file, calc_clone));
        handles.push(handle);

        // We need to create a BufReader for the file.
        //
        // It can be excessively inefficient to work directly with a reader,
        // as each read results in a system call. A buffered readered performs
        // large, infrequent reads on the underlying reader and maintains an
        // in-memory buffer of the results.
    }

    for handle in handles {
        handle.join().unwrap()
    }
    //let calculadora_read = lock_calculadora.read().unwrap();

    println!("{}", lock.lock().unwrap().value())
}

fn funcion_auxiliar(file: File, calculadora: Arc<Mutex<Calculator>>) {
            //let  lock_calculadora = lock.clone();
            let file_reader = BufReader::new(file);


            // A buffered reader also implements useful methods, like `lines()`
            for line in file_reader.lines() {
                // The underlying reader (file) may fail. In that case, we print the
                // error and skip the current file.
                let line = match line {
                    Ok(line) => line,
                    Err(error) => {
                        eprintln!("failed to read line {}", error);
                        break;
                    }
                };

                // The operation may be invalid. In that case, we print the error
                // and skip the current *line*.
                let operation = match Operation::from_str(&line) {
                    Ok(operation) => operation,
                    Err(error) => {
                        eprintln!("failed to parse line {}", error);
                        continue;
                    }
                };
                let mut calculadora = calculadora.lock().unwrap();
                calculadora.apply(operation);
            }
        }
        */

*/
