fn search_message(input: &Vec<char>, window_size: usize) -> usize {
    let mut window_index = 0;

    for (i, window) in input.windows(window_size).enumerate() {
        // Use vector and insert as show by ThePrimeagean in the YouTube video explaining this problem.
        // for this size of vector insert and check linearly than using a data type such as a set.
        let mut idx : Vec<char> = Vec::new();

        for c in window {
            if idx.contains(c) {
                break;
            } else {
                idx.push(*c);
            }
        }


        if idx.len() == window_size {
            window_index = i + window_size;
            break;
        }
    }

    return window_index;

}

fn main() {
    let input : Vec<char> = include_str!("../input.txt").trim().chars().collect();

    println!("Solution for part 1 is: {}", search_message(&input, 4));
    println!("Solution for part 2 is: {}", search_message(&input, 14));
}
