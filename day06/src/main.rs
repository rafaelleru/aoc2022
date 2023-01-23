fn main() {
    let input : Vec<char> = include_str!("../input.txt").trim().chars().collect();

    for (i, window) in input.windows(4).enumerate() {
        // Use vector and insert as show by ThePrimeagean in the Yt video explaining this problem.
        // for this size of vector insert and check linearly than using a data type such as a set.
        let mut idx : Vec<char> = Vec::new();

        for c in window {
            if idx.contains(c) {
                break;
            } else {
                idx.push(*c);
            }
        }

        if idx.len() == 4 {
            println!("Solution for Part 1 is: {}", i+4);
            break;
        }
    }


    // Part 2
    for (i, window) in input.windows(14).enumerate() {
        let mut idx : Vec<char> = Vec::new();

        for c in window {
            if idx.contains(c) {
                break;
            } else {
                idx.push(*c);
            }
        }

        if idx.len() == 14 {
            println!("Solution for part 2 is: {}" , i+14);
            break;
        }
    }

}
