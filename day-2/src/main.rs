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
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        input = &args[1];
    } else {
        return
    }
    let lines = read_lines(input).unwrap();
    
    let mut id_sum = 0;
    let mut id_pos = 1;
    for i in lines {
        let e = i.unwrap();
        let t: Vec<&str> = e.split(':').collect();
        let cube_data = t[1].trim();
        let parts: Vec<&str> = cube_data.split(';').collect();

        let mut highest_red = 0;
        let mut highest_green = 0;
        let mut highest_blue = 0;
    
        for part in parts {
            let items = part.split(',');
    
            for item in items {
                let color_data: Vec<&str> = item.trim().split_whitespace().collect();
    
                if color_data.len() == 2 {
                    let count = color_data[0].parse::<i32>().unwrap_or(0);
                    match color_data[1] {
                        "red" => highest_red = highest_red.max(count),
                        "green" => highest_green = highest_green.max(count),
                        "blue" => highest_blue = highest_blue.max(count),
                        _ => {}
                    }
                }
            }
        }

        if highest_red <= 12 && highest_green <= 13 && highest_blue <= 14 {
            println!("{}: {} {} {}", id_pos, highest_red, highest_green, highest_blue);
            id_sum = id_sum + id_pos;
        }
        id_pos+=1;

    }
    println!("{}", id_sum);
}
