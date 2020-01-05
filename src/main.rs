use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

mod lib;

fn main() -> io::Result<()> {
    let mut input = String::new();
    println!("Please enter a day to see the solution:");

    io::stdin().read_line(&mut input)
            .expect("Failed to read line");
    input.pop();

    let day = input.parse().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        return 0;
    });

    match day {
        1 => {
            let day1input = File::open("input1.txt")?;
            let r = BufReader::new(day1input);
            let mut sum: i32 = 0;
            for line in r.lines() {
                let m = line.unwrap().parse::<u32>().unwrap();

                let req = lib::calculate_fuel_req(m);
                sum += req;
            }
            println!("Solution: {}", sum);
        },
        2 => {
            for i in 0..99 {
                for j in 0..99 {
                    let mut iter = lib::IntProgram {
                        mem: vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,10,19,2,9,19,23,1,9,23,27,2,27,9,31,1,31,5,35,2,35,9,39,1,39,10,43,2,43,13,47,1,47,6,51,2,51,10,55,1,9,55,59,2,6,59,63,1,63,6,67,1,67,10,71,1,71,10,75,2,9,75,79,1,5,79,83,2,9,83,87,1,87,9,91,2,91,13,95,1,95,9,99,1,99,6,103,2,103,6,107,1,107,5,111,1,13,111,115,2,115,6,119,1,119,5,123,1,2,123,127,1,6,127,0,99,2,14,0,0],
                        pointer: 0,
                    };

                    iter.run(i,j);

                    if i == 12 && j == 2 { // Pt1.
                        println!("Solution pt1: {}", iter.mem[0]);
                    }

                    if iter.mem[0] == 19690720 { // Pt2.
                        let solution2 = 100 * i + j;
                        println!("Solution pt2: {}", solution2);
                    }
                }
            }
        },
        3 => {
            let day3input = File::open("input3.txt")?;
            let r = BufReader::new(day3input);
            let mut wires = Vec::new();
            for line in r.lines() {
                let input = line.unwrap().parse::<String>().unwrap();

                let new_wire = lib::Wire {
                    input: input,
                    points: vec![(0,0)]
                };

                let segs = new_wire.thread();
                wires.push(segs);
            }

            let wire_a = &wires[0];
            let wire_b = &wires[1];

            let v1: std::collections::HashSet<(i64,i64)> = wire_a.iter().cloned().collect();
            let v2: std::collections::HashSet<(i64,i64)> = wire_b.iter().cloned().collect();

            let mut ds = Vec::new();
            let mut min = Vec::new();

            for x in v1.intersection(&v2) {
                if *x != (0,0) {
                    min.push(lib::get_distance(wire_a, *x) + lib::get_distance(wire_b, *x));
                    ds.push(x.0.abs() + x.1.abs());
                }
            }

            min.sort();
            ds.sort();
            println!("Solution pt1: {}", ds[0]);
            println!("Solution pt2: {}", min[0]);
        },
        _ => println!("{}", input)
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_req() {
        assert_eq!(calculate_fuel_req(12), 2);
        assert_eq!(calculate_fuel_req(14), 2);
        assert_eq!(calculate_fuel_req(1969), 654);
        assert_eq!(calculate_fuel_req(100756), 33583);
    }
}
