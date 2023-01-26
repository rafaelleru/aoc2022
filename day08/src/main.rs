fn visible_from_top(map: &Vec<Vec<u32>>, row: usize, column: usize) -> bool {
    let tree_size = map[row][column];
    let mut visible = true;

    for i in 0..row {
        if map[i][column] >= tree_size {
            visible = false;
            break;
        }
    }

    return visible;

}


fn visible_from_bottom(map: &Vec<Vec<u32>>, row: usize, column: usize) -> bool {
    let tree_size = map[row][column];
    let mut visible = true;

    for i in row+1..map.len() {
        if map[i][column] >= tree_size {
            visible = false;
            break;
        }
    }

    return visible;

}

fn visible_from_left(map: &Vec<Vec<u32>>, row: usize, column: usize) -> bool {
    let tree_size = map[row][column];
    let mut visible = true;

    for i in 0..column {
        if map[row][i] >= tree_size {
            visible = false;
            break;
        }
    }

    return visible;

}

fn visible_from_right(map: &Vec<Vec<u32>>, row: usize, column: usize) -> bool {
    let tree_size = map[row][column];
    let mut visible = true;

    for i in column+1..map.first().unwrap().len() {
        if map[row][i] >= tree_size {
            visible = false;
            break;
        }
    }

    return visible;

}

fn main() {
    let map = include_str!("../input.txt")
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    let mut nvisible = map.len()*2 + (map.first().unwrap().len()-2)*2;

    // Part 1
    for (i, row) in map.iter().enumerate().skip(1).take(map.len()-2) {
        for (j, _column) in row.iter().enumerate().skip(1).take(map.first().unwrap().len()-2) {
            let visible = visible_from_top(&map, i, j) 
                || visible_from_left(&map, i, j) 
                || visible_from_right(&map, i, j)
                || visible_from_bottom(&map, i, j);
            if visible {
                nvisible += 1;
            }
        }
    }

    println!("{} visibles", nvisible);
}
