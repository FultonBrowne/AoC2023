use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calculate_points(matches: u32) -> u32 {
    if matches == 0 {
        0
    } else {
        1 << (matches - 1)
    }
}


fn main() {
    let mut input = "";
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        input = &args[1];
    } else {
        return
    }
    let lines = read_lines(input).unwrap();
    let mut o = 0;
    for i in lines {
        let e = i.unwrap();
        let ts: Vec<&str> = e.split(":").collect();
        let t = ts[1];
        let num_sets: Vec<&str> = t.split("|").collect();
        let s1 : HashSet<_> = num_sets[0].split_whitespace().collect();
        let s2 : HashSet<_>= num_sets[1].split_whitespace().collect();
        let count = s1.intersection(&s2).count();
        o += calculate_points(count.try_into().unwrap());
    }
    println!("{}", o);
}
