use std::convert::TryInto;
#[derive(Clone, Debug)]
struct Point {
    x: i8,
    y: i8,
    possibilities: Vec<i8>,
}

fn get_row_possibilities(puzzle: & [[i8; 9]; 9], x: i8) -> [i8; 9] {
    let mut row_possiblities: [i8; 9] = [0; 9];
    for i in 0..9 {
        let cell = puzzle[x as usize][i];
        if cell != 0 {
            row_possiblities[cell as usize - 1] = 1;
        }
    }
    return row_possiblities;
}

fn get_col_possibilities(puzzle: & [[i8; 9]; 9], y: i8) -> [i8; 9] {
    let mut col_possiblities: [i8; 9] = [0; 9];
    for i in 0..9 {
        let cell = puzzle[i][y as usize];
        if cell != 0 {
            col_possiblities[cell as usize - 1] = 1;
        }
    }
    return col_possiblities;
}

fn get_square_possibilities(puzzle: & [[i8; 9]; 9], x: i8, y: i8) -> [i8; 9] {
    let x_ind: usize = (x / 3 * 3).try_into().unwrap();
    let y_ind: usize = (y / 3 * 3).try_into().unwrap();
    let mut square_possiblities: [i8; 9] = [0; 9];
    for x in 0..3 {
        for y in 0..3 {
            let cell = puzzle[x_ind + x][y_ind + y];
            if cell != 0 {
                square_possiblities[cell as usize - 1] = 1;
            }
        }
    }
    return square_possiblities;
}

fn get_valid_numbers(puzzle: & [[i8; 9]; 9], x: i8, y: i8) -> Vec<i8> {
    // Loop over the row the point is in
    // if a val is in possibilities, update 
    // the array with the val 1
    let row_possiblities: [i8; 9] = get_row_possibilities(puzzle, x);
    
    let col_possiblities: [i8; 9] = get_col_possibilities(puzzle, y);

    let square_possiblities: [i8; 9] = get_square_possibilities(puzzle, x, y);

    let mut temp;
    let mut output: Vec<i8> = Vec::new();
    for i in 0..9 {
        temp = col_possiblities[i] + row_possiblities[i] + square_possiblities[i];
        if temp == 0 {
            let ii = i+1;
            output.push(ii as i8);
        }
    }
    return output;
}
// 
fn solve(puzzle: &mut [[i8; 9]; 9], unfilled: &mut Vec<Point>) -> bool {
    // Check to see if there are any empty spaces
    // If not, return true. The puzzle is solved!
    // If there are empty spaces, get the best one
    // let mut unfilled = get_unfilled(& puzzle);
    let point: Point;
    match unfilled.pop() {
        None => {
            return true;
        },
        Some(p) => {
            point = p;
        }
    }

    // Iterate over the valid possibilities
    // Plug each one in to the puzzle and recurse on that 
    // new puzzle. 
    // If we get a true return from our recursive call, return true
    // Else, set the possibility back to 0
    for number in point.possibilities.iter() {
        let mut new_unfilled = unfilled.to_vec();
        update_unfilled(&mut new_unfilled, point.x, point.y, *number);
        puzzle[point.x as usize][point.y as usize] = *number;
        if solve(puzzle, &mut new_unfilled){
            return true;
        }
        puzzle[point.x as usize][point.y as usize] = 0;
    }
    // If no valid possiblities exist, return false
    // this will trigger backtracking. 
    return false
}

fn parse_puzzle(content: &str) -> [[i8; 9]; 9] {
    let mut puzzle: [[i8; 9]; 9] = [[0; 9]; 9];

    let radix: u32 = 10;
    let mut count = 0;
    for c in content.chars() {
        let val: u32;
        if c == '.' {
            val = 0;
        } else {
            val = c.to_digit(radix).unwrap();
        }
        let x = count / 9;
        let y = count % 9;
        count += 1;
        puzzle[x][y] = val as i8;
    }
    return puzzle;
}

fn update_unfilled(unfilled: &mut Vec<Point>, x: i8, y: i8, val: i8) {
    let x_ind: i8 = x / 3 * 3;
    let y_ind: i8 = y / 3 * 3;

    for point in unfilled.iter_mut() {
        // if point
        let in_row: bool = point.x == x;
        let in_col: bool = point.y == y;
        let p_x_ind: i8 = point.x / 3 * 3;
        let p_y_ind: i8 = point.y / 3 * 3;
        let in_square: bool = x_ind == p_x_ind && y_ind == p_y_ind;

        if in_row || in_col || in_square {
            // point.possibilities.drain_filter(|p| *p == val);
            let mut i = 0;
            while i != point.possibilities.len() {
                if point.possibilities[i] == val {
                    point.possibilities.remove(i);
                } else {
                    i += 1;
                }
            }
        }
    }
    unfilled.sort_by_cached_key(|p| p.possibilities.len());
    unfilled.reverse();
}


fn get_unfilled(puzzle: & [[i8; 9]; 9]) -> Vec<Point> {
    let mut output: Vec<Point> = Vec::new();

    for x in 0..9 {
        for y in 0..9 {
            let cell = puzzle[x][y];
            if cell == 0 {
                let possibilities: Vec<i8> = get_valid_numbers(puzzle, x as i8, y as i8);
                let point: Point = Point{x: x as i8, y: y as i8, possibilities};
                output.push(point);
                
            }
        }
    }

    // reorder the vector so that the points 
    // with the least possibilities are popped first
    output.sort_by_cached_key(|p| p.possibilities.len());
    output.reverse();
    // for o in output.iter() {
    //     println!("possibilities for point {}, {} = {:?}", o.x, o.y, o.possibilities);
    // }
    return output;
}

pub fn parse_and_solve(content: &str) -> [[i8; 9]; 9] {
    // parse the string into a two dimentional array
    let mut puzzle: [[i8; 9]; 9] = parse_puzzle(&content);
    let mut unfilled: Vec<Point> = get_unfilled(&puzzle);
    solve(&mut puzzle, &mut unfilled);
    return puzzle
}

pub fn puzzle_to_string(puzzle: &[[i8; 9]; 9]) -> String {
    let mut output: String = String::new();
    for x in 0..9 {
        if x%3 == 0 && x !=0 {
            output.push_str("---------------------\n");
        }
        for y in 0..9{
            let v = puzzle[x][y];
            if y%3 == 0 && y !=0 {
                output.push_str("| ");
            }
            output.push_str(&v.to_string());
            output.push_str(" ");
        }
        output.push_str("\n");
    }
    return output;
}

pub fn print_to_output(content: &str, mut writer: impl std::io::Write) {
    writeln!(writer, "{}", content);
}




#[cfg(test)]
fn test_board() -> [[i8; 9]; 9] {
    [
        [1, 7, 4, 0, 9, 0, 6, 0, 0],
        [0, 0, 0, 0, 3, 8, 1, 5, 7],
        [5, 3, 0, 7, 0, 1, 0, 0, 4],
        [0, 0, 7, 3, 4, 9, 8, 0, 0],
        [8, 4, 0, 5, 0, 0, 3, 6, 0],
        [3, 0, 5, 0, 0, 6, 4, 7, 0],
        [2, 8, 6, 9, 0, 0, 0, 0, 1],
        [0, 0, 0, 6, 2, 7, 0, 3, 8],
        [0, 5, 3, 0, 8, 0, 0, 9, 6],
    ]
}

#[cfg(test)]
fn almost_solved_test_board() -> [[i8; 9]; 9] {
    [
        [1, 7, 4, 0, 9, 5, 6, 8, 3],
        [9, 6, 2, 4, 3, 8, 1, 5, 7],
        [5, 3, 8, 7, 6, 1, 9, 2, 4],
        [6, 2, 7, 3, 4, 9, 8, 1, 5],
        [8, 4, 1, 5, 7, 2, 3, 6, 9],
        [3, 9, 5, 0, 0, 6, 4, 7, 2],
        [2, 8, 6, 0, 0, 3, 7, 4, 1],
        [4, 1, 9, 0, 0, 0, 5, 3, 8],
        [7, 5, 3, 1, 8, 4, 2, 9, 6],
    ]
}
#[cfg(test)]
fn solved_test_board() -> [[i8; 9]; 9] {
    [
        [1, 7, 4, 2, 9, 5, 6, 8, 3],
        [9, 6, 2, 4, 3, 8, 1, 5, 7],
        [5, 3, 8, 7, 6, 1, 9, 2, 4],
        [6, 2, 7, 3, 4, 9, 8, 1, 5],
        [8, 4, 1, 5, 7, 2, 3, 6, 9],
        [3, 9, 5, 8, 1, 6, 4, 7, 2],
        [2, 8, 6, 9, 5, 3, 7, 4, 1],
        [4, 1, 9, 6, 2, 7, 5, 3, 8],
        [7, 5, 3, 1, 8, 4, 2, 9, 6],
    ]
}
#[cfg(test)]
fn test_board_str() -> &'static str {
    "174.9.6......3815753.7.1..4..73498..84.5..36.3.5..647.2869....1...627.38.53.8..96"
}

#[test]
fn test_get_valid_numbers() {
    let mut board = almost_solved_test_board();
    let expected = vec![2];
    let actual = get_valid_numbers(&mut board, 0, 3);
    assert_eq!(expected, actual)
}

#[test]
fn test_get_valid_numbers_2() {
    let mut board = test_board();
    let expected = vec![2, 6, 9];
    let actual = get_valid_numbers(&mut board, 1, 1);
    assert_eq!(expected, actual)
}

#[test]
fn test_solve() {
    let mut board = test_board();
    let mut unfilled = get_unfilled(&board);
    let actual = solve(&mut board, &mut unfilled);
    assert_eq!(true, actual);
    let solved = solved_test_board();
    assert_eq!(solved, board);
}

#[test]
fn test_parse_and_solve() {
    let board: &str = test_board_str();
    let actual: [[i8; 9]; 9] = parse_and_solve(board);
    let expected: [[i8; 9]; 9] = solved_test_board();
    assert_eq!(actual, expected);
}