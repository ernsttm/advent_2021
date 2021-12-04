use crate::Bit::{ONE, ZERO};

const _TEST_CONTENTS: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

#[derive(PartialEq, Copy, Clone)]
enum Bit {
    ONE,
    ZERO,
}

#[derive(Clone)]
struct BitField {
    bits: Vec<Bit>,
}

impl BitField {
    fn to_binary(&self) -> usize {
        let mut result = 0;

        for (bit_index, binary_index) in (0..self.bits.len()).rev().enumerate() {
            if self.bits[bit_index] == ONE {
                result |= 1 << binary_index
            }
        }

        result
    }
}

fn parse(contents: &str) -> Vec<BitField> {
    contents
        .lines()
        .map(|l| {
            let bits = l
                .chars()
                .map(|c| if c == '1' { ONE } else { ZERO })
                .collect::<Vec<Bit>>();

            BitField { bits }
        })
        .collect()
}

fn power_consumption(diagnostics: &[BitField]) -> usize {
    let bit_count = diagnostics[0].bits.len();
    let mut gamma = 0;
    let mut epsilon = 0;
    for (field_index, gamma_index) in (0..bit_count).rev().enumerate() {
        let gamma_count = diagnostics
            .iter()
            .filter(|&b| b.bits[field_index] == ONE)
            .count();

        if gamma_count > diagnostics.len() / 2 {
            gamma |= 1 << gamma_index;
        } else {
            epsilon |= 1 << gamma_index;
        }
    }

    gamma * epsilon
}

fn life_support(diagnostics: &[BitField]) -> usize {
    let bit_count = diagnostics[0].bits.len();

    let mut t = diagnostics
        .iter()
        .map(|f| f.clone())
        .collect::<Vec<BitField>>();

    let mut oxygen = BitField { bits: Vec::new() };
    for i in 0..bit_count {
        let (ones, zeros): (Vec<&BitField>, Vec<&BitField>) =
            t.iter().partition(|&f| f.bits[i] == ONE);

        if ones.len() * 2 >= t.len() {
            t = ones.iter().map(|&f| f.clone()).collect::<Vec<BitField>>();
        } else {
            t = zeros.iter().map(|&f| f.clone()).collect::<Vec<BitField>>();
        }

        if t.len() == 1 {
            oxygen = t[0].clone();
            break;
        }
    }

    t = diagnostics
        .iter()
        .map(|f| f.clone())
        .collect::<Vec<BitField>>();
    let mut carbon = BitField { bits: Vec::new() };
    for i in 0..bit_count {
        let (ones, zeros): (Vec<&BitField>, Vec<&BitField>) =
            t.iter().partition(|&f| f.bits[i] == ONE);

        if ones.len() * 2 < t.len() {
            t = ones.iter().map(|&f| f.clone()).collect::<Vec<BitField>>();
        } else {
            t = zeros.iter().map(|&f| f.clone()).collect::<Vec<BitField>>();
        }

        println!("{}", t.len());
        if t.len() == 1 {
            carbon = t[0].clone();
            break;
        }
    }

    println!("{} | {}", oxygen.to_binary(), carbon.to_binary());
    oxygen.to_binary() * carbon.to_binary()
}

fn main() {
    let contents = std::fs::read_to_string("day3/input.txt").unwrap();
    let diagnostics = parse(&contents);
    println!("Power Consumption {}", power_consumption(&diagnostics));
    println!("Life Support {}", life_support(&diagnostics));
}
