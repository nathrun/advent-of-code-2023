const W_CARD_NUMBER: usize = 10;
const CARD_NUMBER:usize = 25;
// const W_CARD_NUMBER: usize = 5;
// const CARD_NUMBER:usize = 8;

fn main() {
    let input = include_str!("part1.txt");
    let points = part1(input);
    println!("Part 2: {}", points);
}

//calculate the number of matches, points = 2 ^ (matches - 1)
fn part1(input: &str) -> u32 {
    let mut card_points: Vec<u32> = Vec::new();
    let mut line_count = 0;
    for line in input.lines() {
        if card_points.len() <= line_count as usize {
            card_points.push(0);
        }
        let games = line.split(": ").collect::<Vec<&str>>()[1].split(" | ").collect::<Vec<&str>>();
        let mut w_numbers: [u8; W_CARD_NUMBER] = [0; W_CARD_NUMBER];
        let mut cards: [u8; CARD_NUMBER] = [0; CARD_NUMBER];
        for card in games[0].split_whitespace().enumerate() {
            w_numbers[card.0] = card.1.parse::<u8>().unwrap();
        }
        for card in games[1].split_whitespace().enumerate() {
            cards[card.0] = card.1.parse::<u8>().unwrap();
        }

        let matches = calculate_matches(w_numbers, cards);

        card_points[line_count as usize] += 1;
        
        println!("line: {} - matches: {}", line_count + 1 ,matches);
        
        for i in 0..matches {
            let increase_index = (line_count + (i + 1)) as usize;
            if increase_index >= card_points.len() {
                card_points.push(1 * card_points[line_count as usize]);
            } else {
                card_points[(line_count + (i + 1)) as usize] += 1 * card_points[line_count as usize];
            }
        }

        line_count += 1;
    }
    card_points.iter().fold(0, |acc, x| acc + x)
}

fn calculate_matches (w_numbers: [u8; W_CARD_NUMBER], cards: [u8; CARD_NUMBER]) -> u8 {
    let mut matches = 0;
    for i in 0..W_CARD_NUMBER {
        for j in 0..CARD_NUMBER {
            if w_numbers[i] == cards[j] {
                matches += 1;
            }
        }
    }
    return matches;
}