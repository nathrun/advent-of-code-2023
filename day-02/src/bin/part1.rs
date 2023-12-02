fn main() {
    println!("Day 2 part 1");
    let input = include_str!("part1.txt");
    let output = part1(input, 12, 13, 14);
    dbg!(output);
}

fn part1(input: &str, red: u8, green: u8, blue: u8) -> String {
    let lines = input.split("\n");
    format!("red: {}, green: {}, blue: {}", red, green, blue);
    let sum = lines.fold(0, |acc, line| {
        let (game_id, contents) = line_split(line);
        let valid = check_valid_game(contents, red, green, blue);
        if !valid {
            println!("{}", line);
        }
        println!("Game {} is valid: {}", game_id, valid);
        if valid {
            acc + game_id as u32
        } else {
            acc
        }
    });
    return sum.to_string();
}

//returns (gameId, contents) where contents is a vector of (setNumber, Vec<(colour, count)>)
fn line_split(line: &str) -> (u8, Vec<(usize, Vec<(&str, u8)>)>) {
    let split = line.split(":").collect::<Vec<&str>>();
    let game_id = split[0].split(" ").nth(1).unwrap().parse::<u8>().unwrap();
    let contents = split[1].split(";").enumerate().map(|(i, set)| {
        let sets = set.split(",").map(|ball| {
            let game_split = ball.trim().split(" ").collect::<Vec<&str>>();
            let colour = game_split[1].trim();
            let count = game_split[0].trim().parse::<u8>().unwrap_or(0);
            (colour, count)
        }).collect::<Vec<(&str, u8)>>();
        (i, sets)
    }).collect::<Vec<(usize, Vec<(&str, u8)>)>>();
    
    (game_id, contents)
}

fn check_valid_game(contents: Vec<(usize, Vec<(&str, u8)>)>, red: u8, green: u8, blue: u8) -> bool {
    for (_, set) in contents {
        for (colour, count) in set {
            match colour {
                "red" => {
                    if red < count {
                        return false
                    }
                },
                "green" => {
                    if green < count {
                        return false
                    }
                },
                "blue" => {
                    if blue < count {
                        return false
                    }
                },
                _ => panic!("Unknown colour")
            }
        }
    }
    return true
}
