use regex::Regex;

fn main() {
    let input = include_str!("part1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    //split input into lines
    let lines = input.split("\n");
    //regex to catch first digit
    let re = Regex::new(r"\d").unwrap();
    let mut total = 0;
    //iterate over lines
    for line in lines {
        let numbers = re.find_iter(line).map(|x| x.as_str()).collect::<Vec<&str>>();
        let number = format!("{}{}", numbers[0], numbers[numbers.len() - 1]).parse::<u32>();
        total += number.unwrap();
    }
    return total.to_string();
}