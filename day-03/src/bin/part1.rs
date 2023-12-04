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

fn process_schematic(schematic: Vec<Vec<char>>) -> u32 {
    let mut total: u32 = 0;
    for (y, row) in schematic.iter().enumerate() {
        print!("{}:  ", y);
        let mut num_check = false;
        let mut number = String::new();
        let mut include_number = false;
        for (x, c) in row.iter().enumerate() {
            if c.is_digit(10) {
                num_check = true;
                number.push(*c);
                if !include_number && check_adjacent_cells(schematic.clone(), x, y) {
                    include_number = true;
                }
            } else if num_check && !c.is_digit(10) {
                if include_number {
                    print!("{} | ", number);
                    let i = number.parse::<u32>().unwrap_or(0);
                    total += i;
                    include_number = false;
                }
                num_check = false;
                number.clear();
            }
        }

        if number.len() != 0 && include_number {
            print!("{} | ", number);
            let i = number.parse::<u32>().unwrap_or(0);
            total += i;
        }

        println!();
    }
    total
}

fn check_adjacent_cells(schematic: Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let mut adjacent_cells: Vec<char> = Vec::new();
    if x > 0 {
        adjacent_cells.push(schematic[y][x - 1]);
    }
    if x < schematic[y].len() - 1 {
        adjacent_cells.push(schematic[y][x + 1]);
    }
    if y > 0 {
        adjacent_cells.push(schematic[y - 1][x]);
    }
    if y < schematic.len() - 1 {
        adjacent_cells.push(schematic[y + 1][x]);
    }
    if x > 0 && y > 0 {
        adjacent_cells.push(schematic[y - 1][x - 1]);
    }
    if x < schematic[y].len() - 1 && y < schematic.len() - 1 {
        adjacent_cells.push(schematic[y + 1][x + 1]);
    }
    if x > 0 && y < schematic.len() - 1 {
        adjacent_cells.push(schematic[y + 1][x - 1]);
    }
    if x < schematic[y].len() - 1 && y > 0 {
        adjacent_cells.push(schematic[y - 1][x + 1]);
    }
    //check for symbols (not digits or .)
    for c in adjacent_cells {
        if !char::from(c).is_digit(10) && c != '.'  {
            return true;
        }
    }
    false
}
