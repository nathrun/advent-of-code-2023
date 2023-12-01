use std::vec;

use regex::Regex;
#[derive(Debug)]
struct Match<'a> {
    start: usize,
    value: &'a str,
}

fn main() {
    let input = include_str!("part2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    //split input into lines
    let lines = input.split("\n");
    //regex in two groups to account for overlaps
    let group1 = Regex::new(r"\d|(two)|(four)|(six)|(seven)").unwrap();
    let group2 = Regex::new(r"(eight)|(nine)").unwrap();
    let group3 =Regex::new(r"(one)|(three)|(five)").unwrap();
    let mut total = 0;
    //iterate over lines
    for line in lines {
        let mut numbers: Vec<Match> = vec![];
        dbg!(line);
        for cap in group1.captures_iter(line) {
            cap.iter().for_each(|x| {
                if let Some(x) = x {
                    let value = convert_to_number(x.as_str());
                    numbers.push(Match {start: x.start(), value})
                }
            })
        }
        for cap in group2.captures_iter(line) {
            cap.iter().for_each(|x| {
                if let Some(x) = x {
                    let value = convert_to_number(x.as_str());
                    numbers.push(Match {start: x.start(), value});
                }
            })
        }
        for cap in group3.captures_iter(line) {
            cap.iter().for_each(|x| {
                if let Some(x) = x {
                    let value = convert_to_number(x.as_str());
                    numbers.push(Match {start: x.start(), value});
                }
            })
        }
        numbers.sort_by(|a, b| a.start.cmp(&b.start));
        let lineTotal = format!("{}{}", numbers[0].value, numbers[numbers.len() -1].value).parse::<i32>().unwrap();
        dbg!(lineTotal);
        total += lineTotal;
    }
    return total.to_string();
}

fn convert_to_number(input: &str) -> &str {
    match input {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => input,
    }
}