use std::collections::HashMap;
use regex::Regex;

fn main() {
    let re : Regex = Regex::new(r"[a-zA-Z]").unwrap();
    let input : &str = include_str!("../input.txt");
    let mut stacks : HashMap<usize, Vec<&str>> = HashMap::new();
    let move_regex : Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    // Build the stacks 
    for l in input.lines() {
        if l == "" {
            break;
        }

        for letter in re.find_iter(l) {
            let destination_stack : usize = (&letter.start() - 1) / 4;
            let stack = stacks.entry(destination_stack).or_insert(vec![]);
            stack.insert(0, letter.as_str());
        }
    }

    let mut stacks2 : HashMap<usize, Vec<&str>> = stacks.clone();

    for line in input.lines() {
        if line.starts_with("move") {
            let captures = move_regex.captures(line).unwrap();
            let moves = &captures[1].parse::<i32>().unwrap();

            let mut to_be_moved : Vec<&str> = Vec::new();

            for _i in 0..*moves {
                let source_stack = stacks.get_mut(&(&captures[2].parse::<usize>().unwrap() - 1)).unwrap();
                to_be_moved.push(source_stack.pop().unwrap());
            }

            // here I can get another mut ref to the hash map because source_stack is not in the
            // scope anymore
            let stack_idx = &captures[3].parse::<usize>().unwrap() - 1;
            let destination_stack = stacks.get_mut(&stack_idx).unwrap();
            for c in to_be_moved.iter() {
                destination_stack.push(c);
            }
        }
    }

    // I use this for to print the string shorted, if we iterate over stacks output is not printed
    // in the rigth order
    println!("Solution for part 1:");
    for i in 0..stacks.len() {
        print!("{}", stacks[&i].last().unwrap());
    }

    // Part 2
    for line in input.lines() {
        if line.starts_with("move") {
            let captures = move_regex.captures(line).unwrap();
            let moves = &captures[1].parse::<i32>().unwrap();

            let mut to_be_moved : Vec<&str> = Vec::new();

            for _i in 0..*moves {
                let source_stack = stacks2.get_mut(&(&captures[2].parse::<usize>().unwrap() - 1)).unwrap();
                to_be_moved.insert(0, source_stack.pop().unwrap());
            }

            // here I can get another mut ref to the hash map because source_stack is not in the
            // scope anymore
            let stack_idx = &captures[3].parse::<usize>().unwrap() - 1;
            let destination_stack = stacks2.get_mut(&stack_idx).unwrap();

            for c in to_be_moved.iter() {
                destination_stack.push(c);
            }
        }

    }

    println!("\nSolution for part 2:");
    for i in 0..stacks2.len() {
        print!("{}", stacks2[&i].last().unwrap());
    }

}
