use std::env;
use std::fs::File;
use std::io::Write;
use std::io::{self, Read};

fn adembz(x: &str) {
    const MEMORY_SIZE: usize = 10_000;
    let mut memory = vec![0u8; MEMORY_SIZE];
    let mut pointer = 0;
    let mut inst = 0;
    let mut factor = 1;

    while inst < x.len() {
        match x.chars().nth(inst).unwrap() {
            // pointer operator
            'l' => {
                if pointer == MEMORY_SIZE - 1 {
                    pointer = 0;
                } else {
                    pointer += 1;
                }
            }
            'r' => {
                if pointer == 0 {
                    pointer = MEMORY_SIZE - 1;
                } else {
                    pointer -= 1;
                }
            }
            // memory operator
            'a' => {
                memory[pointer] = memory[pointer].wrapping_add(factor);
            }
            's' => {
                memory[pointer] = memory[pointer].wrapping_sub(factor);
            }
            'd' => match memory[pointer].checked_div(factor) {
                Some(result) => {
                    memory[pointer] = result;
                }
                None => {
                    memory[pointer] = 0;
                }
            },
            'm' => {
                memory[pointer] = memory[pointer].wrapping_mul(factor);
            }
            '0' => {
                memory[pointer] = 0u8;
            }
            // factor operator
            'A' => {
                factor = factor.wrapping_add(memory[pointer]);
            }
            'S' => {
                factor = factor.wrapping_sub(memory[pointer]);
            }
            'D' => match factor.checked_div(memory[pointer]) {
                Some(result) => {
                    factor = result;
                }
                None => {
                    factor = 0;
                }
            },
            'M' => {
                factor = factor.wrapping_mul(memory[pointer]);
            }
            'Z' => {
                factor = 0u8;
            }
            // IO operator
            'p' => {
                print!("{}", memory[pointer] as char);
                io::stdout().flush().unwrap();
            }
            'P' => {
                print!("{}", memory[pointer]);
                io::stdout().flush().unwrap();
            }
            'i' => {
                let mut inp = [0; 1];
                io::stdin().read_exact(&mut inp).unwrap();
                memory[pointer] = inp[0];
            }
            // other
            'q' => {
                print!("{}", x);
                io::stdout().flush().unwrap();
            }
            // loops
            '(' => {
                if memory[pointer] == 0 {
                    let mut brackets = 1;
                    while brackets > 0 {
                        inst += 1;
                        if x.chars().nth(inst).unwrap() == '[' {
                            brackets += 1;
                        } else if x.chars().nth(inst).unwrap() == ']' {
                            brackets -= 1;
                        }
                    }
                }
            }
            ')' => {
                if memory[pointer] != 0 {
                    let mut brackets = 1;
                    while brackets > 0 {
                        inst -= 1;
                        if x.chars().nth(inst).unwrap() == ']' {
                            brackets += 1;
                        } else if x.chars().nth(inst).unwrap() == '[' {
                            brackets -= 1;
                        }
                    }
                }
            }
            _ => {}
        }
        inst += 1;
    }
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut code = String::new();
    file.read_to_string(&mut code)?;
    Ok(code)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} [input | interpret <file>]", args[0]);
        return;
    }

    match args[1].as_str() {
        "input" => {
            let mut input = String::new();
            println!("code (Enter and then CTRL-D when done): ");
            io::stdin().read_to_string(&mut input).unwrap();
            adembz(&input);
        }
        "interpret" => {
            if args.len() < 3 {
                println!("Usage: {} interpret <file>", args[0]);
                return;
            }

            let filename = &args[2];
            match read_file(filename) {
                Ok(code) => {
                    adembz(&code);
                }
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                }
            }
        }
        _ => {
            println!("Usage: {} [input | interpter <file>]", args[0]);
        }
    }
}
