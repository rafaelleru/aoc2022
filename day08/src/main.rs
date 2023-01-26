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

fn get_viewing_distance_top(map: &Vec<Vec<u32>>, row: usize, column: usize) -> u32 {
    let tree_size = map[row][column];
    let mut viewing_distance_top = 0;
    let mut i = 0;

    if row == 0 {
        return 0;
    }

    if row > 0 {
        i = row - 1;
    }

    while i > 0 {
        if map[i][column] < tree_size {
            viewing_distance_top += 1;
        } else {
            viewing_distance_top += 1;
            break;
        }

        i -= 1;
    }

    if i == 0 && map[0][column] <= tree_size {
        viewing_distance_top += 1;
    }

    return viewing_distance_top;
}

fn get_viewing_distance_right(map: &Vec<Vec<u32>>, row: usize, column: usize) -> u32 {
    let tree_size = map[row][column];
    let mut viewing_distance_top = 0;

    if column == map.first().unwrap().len() -1 {
        return 0;
    }

    let mut i = column + 1;


    while i < map.first().unwrap().len() {
        if map[row][i] < tree_size {
            viewing_distance_top += 1;
        } else {
            viewing_distance_top += 1;
            break;
        }

        i += 1;
    }

    return viewing_distance_top;
}

fn get_viewing_distance_left(map: &Vec<Vec<u32>>, row: usize, column: usize) -> u32 {
    let tree_size = map[row][column];
    let mut viewing_distance_top = 0;
    let mut i = 0;
    if column == 0 {
        return 0;
    }

    if column > 0 {
        i = column - 1;
    }

    while i > 0 {
        if map[row][i] < tree_size {
            viewing_distance_top += 1;
        } else {
            viewing_distance_top += 1;
            break;
        }

        i -= 1;
    }

    if i == 0 && map[row][0] < tree_size {
        viewing_distance_top += 1;
    }

    return viewing_distance_top;
}

fn get_viewing_distance_bottom(map: &Vec<Vec<u32>>, row: usize, column: usize) -> u32 {
    let tree_size = map[row][column];
    let mut viewing_distance_top = 0;
    if row == map.len() - 1 {
        return 0;
    }

    let mut i = row + 1;

    while i < map.len() {
        if map[i][column] < tree_size {
            viewing_distance_top += 1;
        } else {
            viewing_distance_top += 1;
            break;
        }

        i += 1;
    }

    return viewing_distance_top;
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

    // Part 2
    let mut best_scenic_score = 0;
    for i in 0..map.len() {
        for j in 0..map.first().unwrap().len() {
            let viewing_distance_top = get_viewing_distance_top(&map, i, j);
            let viewing_distance_left = get_viewing_distance_left(&map, i, j);
            let viewing_distance_right = get_viewing_distance_right(&map, i, j);
            let viewing_distance_bottom = get_viewing_distance_bottom(&map, i, j);

            let scenic_score = viewing_distance_top * viewing_distance_right * viewing_distance_left * viewing_distance_bottom;

            println!("Cheking scenic score for {} is {}", map[i][j], scenic_score);
            println!("top: {}", viewing_distance_top);
            println!("bottom: {}", viewing_distance_bottom);
            println!("left: {}", viewing_distance_left);
            println!("right: {}", viewing_distance_right);

            if scenic_score > best_scenic_score {
                best_scenic_score = scenic_score;
            }
        }
    }
    println!("Best scenic score is: {}", best_scenic_score);


}
