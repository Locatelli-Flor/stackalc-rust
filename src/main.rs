use std::io;

fn main() {
    let mut q: Vec<f64> = Vec::new();

    print!("Enter input (q to quit): ");
    loop {

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "q" {
            break;
        }

        let inst = parse(input);
        perform(&mut q, inst);
    }

    println!("Goodbye!");
}

fn parse(input: &str) -> Instructions {
    match input.trim().to_lowercase().as_str() {
        s if s.starts_with("ldc ") => {
            let num_str = &s[4..];
            if let Ok(x) = num_str.trim().parse::<f64>() {
                Instructions::Ldc(x)
            } else {
                Instructions::Unknown
            }
        },
        "add" => Instructions::Add,
        "sub" => Instructions::Sub,
        "mul" => Instructions::Mul,
        "div" => Instructions::Div,
        "neg" => Instructions::Neg,
        "ceq" => Instructions::Ceq,
        "cgt" => Instructions::Cgt,
        "clt" => Instructions::Clt,
        "dup" => Instructions::Dup,
        "pop" => Instructions::Pop,
        _ => Instructions::Unknown
    }
}

fn perform(q: &mut Vec<f64>, instr: Instructions) {
    match instr {
        Instructions::Ldc(x) => {
            q.push(x);
        }
        Instructions::Add | Instructions::Sub | Instructions::Mul | Instructions::Div => {
            if q.len() >= 2 {
            let y = q.pop().unwrap();
            let x = q.pop().unwrap();
            let result = match instr {
                Instructions::Add => x + y,
                Instructions::Sub => x - y,
                Instructions::Mul => x * y,
                Instructions::Div => {
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
        },
        Instructions::Neg => {
            if q.len() >= 1 {
                let x: f64 = q.pop().unwrap();
                q.push(-x);
            } else {
                print_error();
            }
        },
        Instructions::Ceq | Instructions::Cgt | Instructions::Clt => {

        },
        Instructions::Dup => {
            if q.len() >= 1 {
                let x: f64 = q.pop().unwrap();
                q.push(x);
                q.push(x);
            } else {
                print_error();
            }
        },
        Instructions::Pop => {
            if q.len() >= 1 {
                q.pop();
            } else {
                print_error();
            }
        },
        Instructions::Unknown => {
            eprintln!("Unknown operation");
        }
    }
    println!("{:?}", q);
}

enum Instructions {
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
    Unknown
}

fn print_error() {
    eprintln!("Error: Not enough elements in the stack to perform operation."); 
}