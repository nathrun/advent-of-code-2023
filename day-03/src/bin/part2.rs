
fn main() {
    let input = include_str!("part1.txt");
    let schematic = parse_input(input);
    let result = process_schematic(schematic);
    println!("Result: {}", result);
    //536993 too low
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut schematic: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        schematic.push(row);
    }
    schematic.clone()
}

fn process_schematic(schematic: Vec<Vec<char>>) -> i32 {
    let mut total: i32 = 0;
    for (y, row) in schematic.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c.eq(&'*') {
               total += check_gear_ratio(&schematic, x as i32, y as i32);
            }
        }
    }
    total
}

//check if exactly two adjacent cells that or numbers and multiply them
fn check_gear_ratio(schematic: &Vec<Vec<char>>, x: i32, y: i32) -> i32 {
    let mut adjacent_cells: Vec<(i16, i8, i8)> = Vec::new();
    for y_check in y-1..y+2 {
        let mut num_length = 0;
        for x_check in x-1..x+2 {
            if num_length > 0 {
                num_length -= 1;
                continue;
            }
            if x_check == x && y_check == y {
                continue;
            }
            if x_check < 0 || y_check < 0 {
                continue;
            }
            if x_check >= schematic[0].len() as i32 || y_check >= schematic.len() as i32 {
                continue;
            }
            if schematic[y_check as usize][x_check as usize].is_digit(10) {
                let (num, length) = get_number(&schematic, x_check, y_check);
                num_length = length;
                adjacent_cells.push((num, x_check as i8, y_check as i8));
            }
        }
    }

    if adjacent_cells.len() == 2 {
        let mut total: i32 = 1;
        for (num, _, _) in adjacent_cells {
            total *= num as i32;
        }
        return total;
    }
    //return 0 if not exactly two adjacent cells that are numbers
    0
}

fn get_number(schematic: &Vec<Vec<char>>, x: i32, y: i32) -> (i16, i8) {
    //check for symbols (not digits or .)
    let mut number = String::new();
    let mut x_check = x;
    while x_check >= 0 && schematic[y as usize][x_check as usize].is_digit(10) {
        let c = schematic[y as usize][x_check as usize];
        number = format!("{}{}", c, number);
        x_check -= 1;
    }
    let mut x_check = x+1;
    while x_check < schematic[0].len() as i32 && schematic[y as usize][x_check as usize].is_digit(10) {
        let c = schematic[y as usize][x_check as usize];
        number.push(c);
        x_check += 1;
    }
    x_check -= 1;
    (number.parse::<i16>().unwrap_or(0), (x_check - x) as i8)
}
