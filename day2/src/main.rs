fn main() {
    let guide = include_str!("input.txt").trim();
    let moves : Vec<Vec<&str>> = guide.split("\n")  // Split the content by the newline character
        .collect::<Vec<&str>>()  // Convert the spit struct to a Vector of string
        .iter()  // Create an iterator to the vector
        .map(|l| l.split(" ").collect())  // For each element in the iterator split it by " "
        .collect::<Vec<Vec<&str>>>();  // Convert the map result to a vector of vectors of strings

    let mut points = 0;

    for m in &moves {
        if m[0] == "A" {  // Rock
            if m[1] == "X" {  // Rock
                points += 4;
            }

            if m[1] == "Y" { // Paper
                points += 8;
            }

            if m[1] == "Z" { // Scissors
                points += 3;
            }
        }

        if m[0] == "B" {  // Paper
            if m[1] == "X" {  // Rock
                points += 1;
            }

            if m[1] == "Y" { // Paper
                points += 5;
            }

            if m[1] == "Z" { // Scissors
                points += 9;
            }
        }

        if m[0] == "C" {  // Scissors
            if m[1] == "X" {  // Rock
                points += 7;
            }

            if m[1] == "Y" { // Paper
                points += 2;
            }

            if m[1] == "Z" { // Scissors
                points += 6;
            }
        }
    }

    println!("{}", points);

    points = 0;
    for m in &moves {
        if m[1] == "X" {
            if m[0] == "A" {
                points += 3;
            }

            if m[0] == "B" {
                points += 1;
            }

            if m[0] == "C" {
                points += 2;
            }
        }

        if m[1] == "Y" {
            if m[0] == "A" {
                points += 4;
            }

            if m[0] == "B" {
                points += 5;
            }

            if m[0] == "C" {
                points += 6;
            }
        }

        if m[1] == "Z" {
            if m[0] == "A" {
                points += 8;
            }

            if m[0] == "B" {
                points += 9;
            }

            if m[0] == "C" {
                points += 7;
            }
        }
    }

    println!("{}", points);
}
