use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut input = "";
    let mut o = 0;
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        input = &args[1];
    } else {
        return
    }
    for l in read_lines(input).unwrap(){
        let mut y = 0;
        let mut z = 0;
        let e = l.unwrap();
        for i in e.chars() {
            if i.is_digit(10){
                y = i.to_digit(10).unwrap();
                break;
            }
            
        }

        for i in e.chars().rev() {
            if i.is_digit(10){
                z = i.to_digit(10).unwrap();
                break;
            }
        }
        o += ((y * 10) + z)
    }

    println!("{}", o);
}
