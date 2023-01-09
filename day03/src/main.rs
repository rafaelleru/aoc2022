fn char_to_point (c: char) -> u32 {
    let points : u32;
    if c > 'Z' {
        // 'a' is ASCII code 96 but priority 1, so we subtract 96
        points = (c as u32) - 96;
    } else {
        // 'A' is ASCII code 65 but priority 27, so we subtract 38
        points = (c as u32) - 38;
    }

    return points
}

fn main() {

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

    let points : u32 = duplicated.iter()
        .map(|c| char_to_point(*c))
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

    let points : u32 = badges.iter()
        .map(|c| char_to_point(*c))
        .sum();

    println!("{}", points);
}
