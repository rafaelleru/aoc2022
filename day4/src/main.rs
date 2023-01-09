fn main() {
    let lines : Vec<&str> = include_str!("../input.txt")
        .trim()
        .split("\n")
        .collect::<Vec<&str>>();

    let assignements : Vec<Vec<i32>> = lines.iter()
        .map(|l| l.replace(",", "-"))
        .collect::<Vec<String>>()
        .iter()
        .map(|p| p.split("-").collect::<Vec<&str>>())
        .map(|s| {
            s.iter().map(|d| d.parse::<i32>().unwrap()).collect::<Vec<i32>>()
        }).collect();

    let mut counter = 0;

    for a in &assignements {
        if a[0] >= a[2] && a[1] <= a[3] {
                counter += 1;
        }
        else if a[2] >= a[0] && a[3] <= a[1] {
                counter += 1;
        }
    }

    println!("{}", counter);

    // Part 2
    let mut overlapings = 0;

    for i in assignements {
        if i[0] <= i[3] && i[1] >= i[2] {
            overlapings += 1
        }
    }

    println!("{}", overlapings);
}
