use std::io;

fn main() {
    let mut q: Vec<f64> = Vec::new();
    let mut v: [f64; 3] = [0., 0., 0.]; 

    println!("Enter input (q to quit): ");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "q" {
            break;
        }

        let instructions: Vec<Instruction> = parse(input);
        perform(&mut q, &mut v, instructions);
        
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
            s if s.starts_with("ldc:") || s.starts_with("ldv:") || s.starts_with("stv:") => {
                let (prefix, num_str) = s.split_at(4);
                if let Ok(x) = num_str.trim().parse::<f64>() {
                    match prefix {
                        "ldc:" => Instruction::Ldc(x),
                        "ldv:" if x >= 0.0 && x.fract() == 0.0 => Instruction::Ldv(x as usize),
                        "stv:" if x >= 0.0 && x.fract() == 0.0 => Instruction::Stv(x as usize),
                        _ => unreachable!()
                    }
                } else {
                    continue;
                }
            },
            s if s.starts_with("brfalse:") || s.starts_with("brtrue:") || s.starts_with("br:") => {
                let parts: Vec<&str> = s.split(':').collect();
                if parts.len() == 2 {
                    let prefix = parts[0];
                    let num_str = parts[1];
                    if let Ok(x) = num_str.trim().parse::<f64>() {
                        if x >= 0.0 && x.fract() == 0.0 {
                            match prefix {
                                "brfalse" => Instruction::BrFalse(x as usize),
                                "brtrue" => Instruction::BrTrue(x as usize),
                                "br" => Instruction::Br(x as usize),
                                _ => continue
                            }
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            },
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
            _ => continue
        };
        results.push(x);
    }
    return results;
}

fn perform(q: &mut Vec<f64>, v: &mut [f64; 3], instructions: Vec<Instruction>) {
    let mut ip = 0;

    while ip < instructions.len() {
        let instr = &instructions[ip];
        match instr {
            Instruction::Ldc(x) => {
                q.push(*x);
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
                                ip += 1;
                                continue;
                            }
                            x / y
                        }
                        _ => unreachable!(),
                    };
                    q.push(result);
                } else {
                    print_error_not_enough_elements();
                }
            }
            Instruction::Neg => {
                if q.len() >= 1 {
                    let x: f64 = q.pop().unwrap();
                    q.push(-x);
                } else {
                    print_error_not_enough_elements();
                }
            }
            Instruction::Ceq | Instruction::Cgt | Instruction::Clt => {
                if q.len() >= 2 {
                    let x = q.pop().unwrap();
                    let y = q.pop().unwrap();
                    let result = match instr {
                        Instruction::Ceq => (x == y) as i32 as f64,
                        Instruction::Cgt => (y > x) as i32 as f64,
                        Instruction::Clt => (y < x) as i32 as f64,
                        _ => unreachable!(),
                    };
                    q.push(result);
                } else {
                    print_error_not_enough_elements();
                }
            }
            Instruction::Dup => {
                if q.len() >= 1 {
                    let x: f64 = q.pop().unwrap();
                    q.push(x);
                    q.push(x);
                } else {
                    print_error_not_enough_elements();
                }
            }
            Instruction::Pop => {
                if q.len() >= 1 {
                    q.pop();
                } else {
                    print_error_not_enough_elements();
                }
            }
            Instruction::Ldv(x) => {
                if *x < v.len() {
                    q.push(v[*x]);
                } else {
                    print_error_out_of_bounds(*x);
                }
            }
            Instruction::Stv(x) => {
                if *x >= v.len() {
                    print_error_out_of_bounds(*x);
                } else if q.len() >= 1 {
                    v[*x] = q.pop().unwrap();
                } else {
                    print_error_not_enough_elements();
                }
            }
            Instruction::Br(x) => {
                if *x < instructions.len() {
                    ip = *x;
                    continue;
                } else {
                    println!("Program finished due to jump to a larger index.");
                    break;
                }
            }
            Instruction::BrFalse(x) => {
                if q.len() >= 1 {
                    let condition = q.pop().unwrap();
                    if condition == 0.0 {
                        if *x < instructions.len() {
                            ip = *x;
                            continue;
                        } else {
                            println!("Program finished due to jump to a larger index.");
                            break;
                        }
                    }
                } else {
                    print_error_not_enough_elements();
                }
            }
            Instruction::BrTrue(x) => {
                if q.len() >= 1 {
                    let condition = q.pop().unwrap();
                    if condition != 0.0 {
                        if *x < instructions.len() {
                            ip = *x;
                            continue;
                        } else {
                            println!("Program finished due to jump to a larger index.");
                            break;
                        }
                    }
                } else {
                    print_error_not_enough_elements();
                }
            } 
        }
        ip += 1; // Move to the next instruction
    }

    println!("Queue: {:?}, Variables: {:?}", q, v);
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
    Ldv(usize),
    Stv(usize),
    Br(usize),
    BrTrue(usize),
    BrFalse(usize)
}

fn print_error_not_enough_elements() {
    eprintln!("Error: Not enough elements in the stack to perform operation.");
}

fn print_error_out_of_bounds(x: usize) {
    eprintln!("Error: Index out of bounds for index {}", x); 
}
