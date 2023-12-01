use std::env;

fn main() {
    let mut input = "";
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        input = &args[1];
    } else {
        return
    }

    let mut y = None;
    let mut z = None;
    for i in input.chars() {
        if i.is_digit(10){
            y = Some(i);
            break;
        }
        
    }

    for i in input.chars().rev() {
        if i.is_digit(10){
            z = Some(i);
            break;
        }
    }

    match (y, z) {
        (Some(a), Some(b)) => println!("{}{}", a, b),
        _ => println!("No digits found in the argument."),
    }

}
