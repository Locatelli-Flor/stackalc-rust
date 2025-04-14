use std::io;

fn main() {
    let mut q: Vec<f64> = Vec::new();

    println!("Enter input (q to quit): ");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "q" {
            break;
        }

        let instructions: Vec<Instruction> = parse(input);
        for instr in instructions {
            perform(&mut q, instr);
        }
    }

    println!("Goodbye!");
}

fn parse(input: &str) -> Vec<Instruction> {
    let lis: Vec<&str> = input.split(' ').collect();
    let mut results = Vec::new();

    for instr in lis {
        if instr.is_empty() {
            continue;
        }
        let x = match instr.trim().to_lowercase().as_str() {
            s if s.starts_with("ldc:") => {
                let num_str = &s[4..];
                if let Ok(x) = num_str.trim().parse::<f64>() {
                    Instruction::Ldc(x)
                } else {
                    Instruction::Unknown
                }
            }
            "add" => Instruction::Add,
            "sub" => Instruction::Sub,
            "mul" => Instruction::Mul,
            "div" => Instruction::Div,
            "neg" => Instruction::Neg,
            "ceq" => Instruction::Ceq,
            "cgt" => Instruction::Cgt,
            "clt" => Instruction::Clt,
            "dup" => Instruction::Dup,
            "pop" => Instruction::Pop,
            _ => Instruction::Unknown,
        };
        results.push(x);
    }
    return results;
}

fn perform(q: &mut Vec<f64>, instr: Instruction) {
    match instr {
        Instruction::Ldc(x) => {
            q.push(x);
        }
        Instruction::Add | Instruction::Sub | Instruction::Mul | Instruction::Div => {
            if q.len() >= 2 {
                let y = q.pop().unwrap();
                let x = q.pop().unwrap();
                let result = match instr {
                    Instruction::Add => x + y,
                    Instruction::Sub => x - y,
                    Instruction::Mul => x * y,
                    Instruction::Div => {
                        if y == 0.0 {
                            eprintln!("Error: Division by zero.");
                            q.push(x);
                            q.push(y);
                            return;
                        }
                        x / y
                    }
                    _ => unreachable!(),
                };
                q.push(result);
            } else {
                print_error();
            }
        }
        Instruction::Neg => {
            if q.len() >= 1 {
                let x: f64 = q.pop().unwrap();
                q.push(-x);
            } else {
                print_error();
            }
        }
        Instruction::Ceq | Instruction::Cgt | Instruction::Clt => {
            if q.len() >= 2 {
                let x = q.pop().unwrap();
                let y = q.pop().unwrap();
                let result = match instr {
                    Instruction::Ceq => {
                        if x == y {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    Instruction::Cgt => {
                        if y > x {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    Instruction::Clt => {
                        if y < x {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    _ => unreachable!(),
                };
                q.push(result);
            }
        }
        Instruction::Dup => {
            if q.len() >= 1 {
                let x: f64 = q.pop().unwrap();
                q.push(x);
                q.push(x);
            } else {
                print_error();
            }
        }
        Instruction::Pop => {
            if q.len() >= 1 {
                q.pop();
            } else {
                print_error();
            }
        }
        Instruction::Unknown => {
            eprintln!("Unknown operation");
        }
    }
    println!("{:?}", q);
}

enum Instruction {
    Ldc(f64),
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Ceq,
    Cgt,
    Clt,
    Dup,
    Pop,
    Unknown,
}

fn print_error() {
    eprintln!("Error: Not enough elements in the stack to perform operation.");
}
