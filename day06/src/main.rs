fn main() {
    let input : Vec<char> = include_str!("../example.txt").trim().chars().collect();

    for window in input.windows(4) {
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
            for c in idx {
                println!("{}" , c);
            }

            break;
        }
    }


    // TODO: return the index of the window. I already solved it by searching the substring in vim
    // and getting the column xD

    // Part 2
    println!("Part 2");
    for window in input.windows(14) {
        let mut idx : Vec<char> = Vec::new();

        for c in window {
            if idx.contains(c) {
                break;
            } else {
                idx.push(*c);
            }
        }

        if idx.len() == 14 {
            for c in idx {
                println!("{}" , c);
            }

            break;
        }
    }
}
