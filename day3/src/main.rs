use std::collections::HashMap;

fn main() {
    let priorities : HashMap<char, i32> = vec![
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ].into_iter().collect();

    let input = include_str!("../input.txt");
    let rucksacks : Vec<&str> = input.trim().split("\n").collect::<Vec<&str>>();  // Convert the spit struct to a Vector of string
    let mut duplicated : Vec<char> = Vec::new();

    for rucksack in &rucksacks {
        //println!("Checking {}", rucksack);
        let rucksack_elements = rucksack.chars().collect::<Vec<char>>();
        let half = rucksack_elements.len() / 2; 

        for i in 0..half {
            if rucksack_elements[half..].contains(&rucksack_elements[i]) {
                //println!("Duplicated found {}", rucksack_elements[i]);
                duplicated.push(rucksack_elements[i]);
                break;
            }
        }

    }

    let points : i32 = duplicated.iter()
        .map(|c| priorities[&c])
        .sum();

    println!("{}", points);

    // Part 2
    let mut badges : Vec<char> = Vec::new();
    let mut i = 0;

    while i < rucksacks.len() {
        for c in rucksacks[i].chars() {
            if rucksacks[i+1].contains(c) && rucksacks[i+2].contains(c) {
                badges.push(c);
                break;
            }
        }
        i += 3
    }

    let points : i32 = badges.iter()
        .map(|c| priorities[&c])
        .sum();

    println!("{}", points);
}
