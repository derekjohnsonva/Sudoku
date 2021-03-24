use std::collections::HashMap;
use std::convert::TryInto;

struct Point {
    x: i8,
    y: i8,
}
fn row_col_is_valid(line: &[usize; 9]) -> bool {
    let mut vals: [i8; 9] = [0; 9];
    
    for i in 0..9 {
        let cell:usize = line[i];
        if cell != 0 {
            if !(vals[cell-1] == 0){
                return false
            }
            else {
                vals[cell-1] = 1;
            }
        }
    }
    println!("Hello World");
    println!("{:?}",vals);
    return true;
}
// 1 | 2  | 3  
// ----------
// 4 | 5  | 6
// ----------
// 7 | 8  | 9
// fn square_is_valid(puzzle: &[[usize; 9]; 9], square: u8)
fn get_row_possibilities(puzzle: &mut [[i8; 9]; 9], p: &Point) -> [i8; 9] {
    let mut row_possiblities: [i8; 9] = [0; 9];
    for i in 0..9 {
        let cell = puzzle[p.x as usize][i];
        if cell != 0 {
            row_possiblities[cell as usize - 1] = 1;
        }
    }
    return row_possiblities;
}

fn get_col_possibilities(puzzle: &mut [[i8; 9]; 9], p: &Point) -> [i8; 9] {
    let mut col_possiblities: [i8; 9] = [0; 9];
    for i in 0..9 {
        let cell = puzzle[i][p.y as usize];
        if cell != 0 {
            col_possiblities[cell as usize - 1] = 1;
        }
    }
    return col_possiblities;
}

fn get_square_possibilities(puzzle: &mut [[i8; 9]; 9], p: &Point) -> [i8; 9] {
    let x_ind: usize = (p.x / 3 * 3).try_into().unwrap();
    let y_ind: usize = (p.y / 3 * 3).try_into().unwrap();
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

fn get_valid_numbers(puzzle: &mut [[i8; 9]; 9], p: &Point) -> Vec<i8> {
    // Loop over the row the point is in
    // if a val is in possibilities, update 
    // the array with the val 1
    let row_possiblities: [i8; 9] = get_row_possibilities(puzzle, p);
    
    let col_possiblities: [i8; 9] = get_col_possibilities(puzzle, p);

    let square_possiblities: [i8; 9] = get_square_possibilities(puzzle, p);

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

fn solve(puzzle: &mut [[i8; 9]; 9], unfilled: &mut Vec<Point>) -> bool {
    // Check to see if there are any empty spaces
    // If not, return true. The puzzle is solved!
    // If there are empty spaces, get one of them.
    let point: Point;
    match unfilled.pop() {
        None => return true,
        Some(p) => {
            point = p;
        }
    }
    // Determine which numbers are valid possiblities 
    // for filling in that space
    let valid: Vec<i8> = get_valid_numbers(puzzle, &point);
    // Iterate over the valid possibilites
    // Plug each one in to the puzzle and recurse on that 
    // new puzzle. 
    // If we get a true return from our recursive call, return true
    // Else, set the possibility back to 0
    for number in valid.iter() {
        puzzle[point.x as usize][point.y as usize] = *number;
        if solve(puzzle, unfilled){
            return true;
        }
        puzzle[point.x as usize][point.y as usize] = 0;
    }
    unfilled.push(point);
    // If no valid possiblities exist, return false
    // this will trigger backtracking. 
    return false
}
fn get_unfilled(puzzle: &mut [[i8; 9]; 9]) -> Vec<Point> {
    let mut output: Vec<Point> = Vec::new();
    for x in 0..9 {
        for y in 0..9 {
            let cell = puzzle[x][y];
            if cell == 0 {
                let point: Point = Point{x: x as i8, y: y as i8};
                output.push(point);
            }
        }
    }
    return output;
}

pub fn parse_and_solve(content: &str) -> [[i8; 9]; 9] {
    // parse the string into a two dimentional array
    let mut puzzle: [[i8; 9]; 9] = [[0; 9]; 9];
    let mut unfilled: Vec<Point> = Vec::new();

    let RADIX: u32 = 10;
    let mut count = 0;
    for c in content.chars() {
        let val: u32;
        if c == '.' {
            val = 0;
        } else {
            val = c.to_digit(RADIX).unwrap();
        }
        let x = count / 9;
        let y = count % 9;
        count += 1;
        puzzle[x][y] = val as i8;
        if val == 0 {
            let newPoint: Point = Point{x: x as i8, y: y as i8};
            unfilled.push(newPoint);
        }
    }

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
    output.push_str("\n");
    return output;
}

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
fn test_board_str() -> &'static str {
    "174.9.6......3815753.7.1..4..73498..84.5..36.3.5..647.2869....1...627.38.53.8..96"
}
#[test]
fn test_row_col_is_valid() {
    let arr1:[usize; 9] = [0,0,0,0,0,0,0,0,0];
    let arr2:[usize; 9] = [1,2,3,0,0,0,0,0,0];
    let arr3:[usize; 9] = [1,2,3,4,5,6,7,8,9];
    let out1 = row_col_is_valid(&arr1);
    let out2 = row_col_is_valid(&arr2);
    let out3 = row_col_is_valid(&arr3);
    assert_eq!(true, out1);
    assert_eq!(true, out2);
    assert_eq!(true, out3);
}
#[test]
fn test_row_col_is_invalid() {
    let arr4:[usize; 9] = [1,1,0,0,0,0,0,0,0];
    let out4 = row_col_is_valid(&arr4);
    assert_eq!(false, out4);
}

#[test]
fn test_get_valid_numbers() {
    let mut board = almost_solved_test_board();
    let p = Point{x: 0, y: 3};
    let expected = vec![2];
    let actual = get_valid_numbers(&mut board, &p);
    assert_eq!(expected, actual)
}

#[test]
fn test_get_valid_numbers_2() {
    let mut board = test_board();
    let p = Point{x: 1, y: 1};
    let expected = vec![2, 6, 9];
    let actual = get_valid_numbers(&mut board, &p);
    assert_eq!(expected, actual)
}

#[test]
fn test_solve() {
    let mut board = test_board();
    let mut unfilled = get_unfilled(&mut board);
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