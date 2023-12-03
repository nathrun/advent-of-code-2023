fn main() {
    println!("Day 2 part 1");
    let input = include_str!("part1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines = input.split("\n");
    let sum = lines.fold(0, |acc, line| {
        let (game_id, contents) = line_split(line);
        let power = get_game_power(contents);
        dbg!(game_id, power);
        acc + power
    });
    return sum.to_string();
}

//returns (gameId, contents) where contents is a vector of (setNumber, Vec<(colour, count)>)
fn line_split(line: &str) -> (u8, Vec<(usize, Vec<(&str, u32)>)>) {
    let split = line.split(":").collect::<Vec<&str>>();
    let game_id = split[0].split(" ").nth(1).unwrap().parse::<u8>().unwrap();
    let contents = split[1].split(";").enumerate().map(|(i, set)| {
        let sets = set.split(",").map(|ball| {
            let game_split = ball.trim().split(" ").collect::<Vec<&str>>();
            let colour = game_split[1].trim();
            let count = game_split[0].trim().parse::<u32>().unwrap_or(0);
            (colour, count)
        }).collect::<Vec<(&str, u32)>>();
        (i, sets)
    }).collect::<Vec<(usize, Vec<(&str, u32)>)>>();
    
    (game_id, contents)
}

fn get_game_power(contents: Vec<(usize, Vec<(&str, u32)>)>) -> u32 {
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;
    for (_, set) in contents {
        for (colour, count) in set {
            match colour {
                "red" => {
                    if red <= 0 {
                        red = count;
                    } else {
                        red = red.max(count);
                    }
                },
                "green" => {
                    if green <= 0 {
                        green = count;
                    } else {
                        green = green.max(count);
                    }
                },
                "blue" => {
                    if blue <= 0 {
                        blue = count;
                    } else {
                        blue = blue.max(count);
                    }
                },
                _ => panic!("Unknown colour")
            }
        }
    }
    //for each colour, set min to at least 1 before doing multiplication
    red = red.max(1);
    green = green.max(1);
    blue = blue.max(1);

    red * green * blue
}
