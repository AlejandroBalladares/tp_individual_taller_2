use std::env::args;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::{io::Read, net::TcpListener, str::FromStr};
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
    let calculadora = Calculator::default();
    let mut handles: Vec<JoinHandle<()>> = vec![];
    let lock = Arc::new(Mutex::new(calculadora));
    let listener = TcpListener::bind(address)?;
    for client_stream in listener.incoming() {
        let calc_clone = Arc::clone(&lock); //EN teoría no debo usar esto 
        let mut cliente = client_stream?;
        let handle = thread::spawn(move || leer_operacion(&mut cliente, calc_clone));
        handles.push(handle);
        let valor = match lock.lock() {
            Ok(mutex) => mutex.value(),
            Err(e) => {
                eprint!("Error: {}", e);
                return Ok(()); //REVISAR!!!!!!!!!!!!!!!!!!!!!!!
            }
        };
        println!("{}", valor)
        //Enviar el resultado al cliente
        //client_stream.write(calculadora.value);
    }
    for handle in handles {
     handle.join().unwrap()
    }
    Ok(())
}

pub fn leer_operacion(stream: &mut dyn Read, calculadora: Arc<Mutex<Calculator>>) {
    loop {
        let mut num_buffer = [0u8; 4];
        match stream.read_exact(&mut num_buffer) {
            Ok(line) => line,
            Err(error) => {
                eprintln!("failed to read line {}", error);
                break;
            }
        }
        // Una vez que leemos los bytes, los convertimos a un u32
        let size = u32::from_be_bytes(num_buffer);

        // Creamos un buffer para el nombre
        let mut mensaje_buf = vec![0; size as usize];
        match stream.read_exact(&mut mensaje_buf) {
            Ok(line) => line,
            Err(error) => {
                eprintln!("failed to read line {}", error);
                break;
            }
        }
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
        let mut calculadora = match calculadora.lock() {
            Ok(calculadora) => calculadora,
            Err(e) => {
                print!("Error: {}", e);
                return;
            }
        };
        calculadora.apply(operation);
    }
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
    Get(),
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the string into tokens separated by whitespace.
        let tokens: Vec<&str> = s.split_whitespace().collect();

        //Agregar un caso para el GET

        // Try to convert the vector into a statically-sized array of 2 elements, failing otherwise.
        let [operation, operand] = tokens.try_into().map_err(|_| "expected 2 arguments")?;

        // Parse the operand into an u8.
        let operand: u8 = operand.parse().map_err(|_| "operand is not an u8")?;

        match operation {
            "+" => Ok(Operation::Add(operand)),
            "-" => Ok(Operation::Sub(operand)),
            "*" => Ok(Operation::Mul(operand)),
            "/" => Ok(Operation::Div(operand)),
            "GET" => Ok(Operation::Get()),
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
            Operation::Get() => () = println!("{}", self.value),
        }
    }
}
