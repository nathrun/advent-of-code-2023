const W_CARD_NUMBER: usize = 10;
const CARD_NUMBER:usize = 25;

fn main() {
    let input = include_str!("part1.txt");
    let points = part1(input);
    println!("Part 1: {}", points);
}

//calculate the number of matches, points = 2 ^ (matches - 1)
fn part1(input: &str) -> u32 {
    let mut points = 0;
    for line in input.lines() {
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
        dbg!(matches);
        if matches > 0 {
            points += 2u32.pow(matches as u32 - 1);
        }
    }
    points
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