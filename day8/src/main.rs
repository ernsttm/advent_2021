use std::collections::HashMap;

const _TEST_CONTENTS: &str = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

struct Display {
    digits: Vec<String>,
    output: Vec<String>,
}

fn parse(contents: &str) -> Vec<Display> {
    contents
        .lines()
        .map(|line| {
            let mut segments = line.split("|");
            let digits = segments
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .map(|d| {
                    let mut c = d.trim().chars().collect::<Vec<char>>();
                    c.sort();
                    c.iter().collect::<String>()
                })
                .collect::<Vec<String>>();
            let output = segments
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .map(|d| {
                    let mut c = d.trim().chars().collect::<Vec<char>>();
                    c.sort();
                    c.iter().collect::<String>()
                })
                .collect::<Vec<String>>();

            Display { digits, output }
        })
        .collect::<Vec<Display>>()
}

impl Display {
    fn number_of_unique_digits_in_output(&self) -> usize {
        self.output
            .iter()
            .filter(|&o| {
                let len = o.len();
                len == 2 || len == 3 || len == 4 || len == 7
            })
            .count()
    }

    fn determine_output(&self) -> usize {
        let mut value_to_digit_map: [String; 10] = Default::default();
        let mut digit_map: HashMap<String, usize> = HashMap::new();

        // First determine the digits we know based purely on length.
        for digit in &self.digits {
            let digit = digit.clone();
            let len = digit.len();
            if len == 2 {
                digit_map.insert(digit.clone(), 1);
                value_to_digit_map[1] = digit;
            } else if len == 3 {
                digit_map.insert(digit.clone(), 7);
                value_to_digit_map[7] = digit;
            } else if len == 4 {
                digit_map.insert(digit.clone(), 4);
                value_to_digit_map[4] = digit;
            } else if len == 7 {
                digit_map.insert(digit.clone(), 8);
                value_to_digit_map[8] = digit;
            }
        }

        // Determine 6 length digits. They are 0 and 9
        for digit in self.digits.iter().filter(|&d| d.len() == 6) {
            let digit = digit.to_string();
            if digit_contains(&value_to_digit_map[4], &digit) {
                digit_map.insert(digit.clone(), 9);
                value_to_digit_map[9] = digit;
            } else if digit_contains(&value_to_digit_map[1], &digit) {
                digit_map.insert(digit.clone(), 0);
                value_to_digit_map[0] = digit;
            } else {
                digit_map.insert(digit.clone(), 6);
                value_to_digit_map[6] = digit;
            }
        }

        // Determine 5 length digits. They are
        for digit in self.digits.iter().filter(|&d| d.len() == 5) {
            if digit_contains(&value_to_digit_map[1], &digit) {
                digit_map.insert(digit.to_string(), 3);
            } else if digit_contains(digit, &value_to_digit_map[6]) {
                digit_map.insert(digit.to_string(), 5);
            } else {
                digit_map.insert(digit.to_string(), 2);
            }
        }

        let output = self
            .output
            .iter()
            .rev()
            .enumerate()
            .map(|(p, d)| {
                let digit = d.to_string();
                ((10 as usize).pow(p as u32)) * digit_map.get(&digit).unwrap()
            })
            .sum();
        output
    }
}

fn digit_contains(a: &str, b: &str) -> bool {
    a.chars().all(|c| b.contains(c))
}

fn total_unique_digits(displays: &[Display]) -> usize {
    displays
        .iter()
        .map(|d| d.number_of_unique_digits_in_output())
        .sum()
}

fn display_output(displays: &[Display]) -> usize {
    displays.iter().map(|d| d.determine_output()).sum()
}

fn main() {
    let contents = std::fs::read_to_string("day8/input.txt").unwrap();
    let displays = parse(&contents);
    println!("Total unique digits: {}", total_unique_digits(&displays));
    println!("Display output sum: {}", display_output(&displays));
}
