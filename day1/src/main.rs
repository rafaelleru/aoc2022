fn main() {
    let content = include_str!("input.txt");
    let calories : Vec<&str> = content.trim().split("\n\n").collect();
    let mut sums : Vec<i32> = Vec::new();

    for c in calories {
        let partials : Vec<&str> = c.split("\n").collect();
        sums.push(partials.iter().map(|p| p.parse::<i32>().unwrap()).sum::<i32>());
    }

    println!("{}", sums.iter().max().unwrap());


    // part 2
    sums.sort();

    let mut top3 : Vec<i32> = Vec::new();

    for _i in 0..3 {
        top3.push(sums.pop().unwrap())
    }

    println!("{}", top3.iter().sum::<i32>())
}
