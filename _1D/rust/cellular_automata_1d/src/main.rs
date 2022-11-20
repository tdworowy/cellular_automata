use itertools::{Itertools, MultiProduct};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct RuleSegment {
    neighborhood: Vec<u32>,
    cell_type: u32,
}

fn get_colors() -> HashMap<u16, (f32, f32, f32)> {
    HashMap::from([
        (0, (0.0, 1.0, 0.0)), // blue
        (1, (0.0, 0.0, 1.0)), // red
        (2, (1.0, 0.0, 0.0)), // grean
        (3, (1.0, 0.7, 0.0)), // yellow
    ])
}

pub fn product_repeat<I>(it: I, repeat: usize) -> MultiProduct<I>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    std::iter::repeat(it).take(repeat).multi_cartesian_product()
}
pub trait ProductRepeat: Iterator + Clone
where
    Self::Item: Clone,
{
    fn product_repeat(self, repeat: usize) -> MultiProduct<Self> {
        std::iter::repeat(self)
            .take(repeat)
            .multi_cartesian_product()
    }
}

impl<T: Iterator + Clone> ProductRepeat for T where T::Item: Clone {}

fn n_nary(mut number: u32, n: u32) -> Vec<u32> {
    let mut result = Vec::new();
    if number == 0 {
        result.push(0 as u32);
    } else {
        while number > 0 {
            let temp = (number / n, number % n);
            number = temp.0;
            result.push(temp.1);
        }
    }
    result.reverse();
    result
}

#[test]
fn test_n_ary() {
    assert_eq!(n_nary(110, 2), [1, 1, 0, 1, 1, 1, 0]);
    assert_eq!(n_nary(0, 2), [0]);
    assert_eq!(n_nary(10, 3), [1, 0, 1]);
}

fn wolfram_number_to_bin(
    wolfram_number: u32,
    possible_states: u32,
    colours_count: u32,
) -> Vec<u32> {
    let mut wolfram_number_n_ary = n_nary(wolfram_number, colours_count);
    let mut wolfram_number_bin = vec![0; possible_states as usize - wolfram_number_n_ary.len()];
    wolfram_number_bin.append(&mut wolfram_number_n_ary);
    wolfram_number_bin.reverse();

    wolfram_number_bin
}

#[test]
fn test_wolfram_number_to_bin() {
    assert_eq!(wolfram_number_to_bin(110, 8, 2), [0, 1, 1, 1, 0, 1, 1, 0]);
}

fn generate_rule(
    wolfram_number: u32,
    neighborhood_size: u32,
    colours_count: u32,
) -> Vec<RuleSegment> {
    let mut rule: Vec<RuleSegment> = Vec::new();
    let possible_states = colours_count.pow(neighborhood_size);
    let wolfram_number = wolfram_number_to_bin(wolfram_number, possible_states, colours_count);

    for (i, neighborhood) in
        product_repeat(0..colours_count, neighborhood_size as usize).enumerate()
    {
        let cell_type = wolfram_number[i];
        rule.push(RuleSegment {
            neighborhood,
            cell_type,
        });
    }
    rule
}
#[test]
fn test_generate_rule() {
    assert_eq!(
        generate_rule(110, 3, 2),
        [
            RuleSegment {
                neighborhood: [0, 0, 0].to_vec(),
                cell_type: 0
            },
            RuleSegment {
                neighborhood: [0, 0, 1].to_vec(),
                cell_type: 1
            },
            RuleSegment {
                neighborhood: [0, 1, 0].to_vec(),
                cell_type: 1
            },
            RuleSegment {
                neighborhood: [0, 1, 1].to_vec(),
                cell_type: 1
            },
            RuleSegment {
                neighborhood: [1, 0, 0].to_vec(),
                cell_type: 0
            },
            RuleSegment {
                neighborhood: [1, 0, 1].to_vec(),
                cell_type: 1
            },
            RuleSegment {
                neighborhood: [1, 1, 0].to_vec(),
                cell_type: 1
            },
            RuleSegment {
                neighborhood: [1, 1, 1].to_vec(),
                cell_type: 0
            }
        ]
    );
}

fn get_neighborhood(input: &Vec<u32>, i: usize, neighborhood_center: usize) -> Vec<u32> {
    let mut current_neighborhood: Vec<u32> = Vec::new();
    let input_lenght: isize = input.len() as isize;
    let neighborhood_centeri = neighborhood_center as isize;

    for j in i as isize - neighborhood_centeri..i as isize + neighborhood_centeri + 1 as isize {
        let index = match j {
            x if x < 0 => input_lenght + x,
            x if x >= input_lenght => x - input_lenght,
            _ => j,
        };
        current_neighborhood.push(input[index as usize]);
    }
    current_neighborhood
}

#[test]
fn test_get_neighborhood() {
    assert_eq!(get_neighborhood(&[0, 1, 0, 1, 0].to_vec(), 2, 1), [1, 0, 1]);
    assert_eq!(get_neighborhood(&[0, 1, 0, 1, 0].to_vec(), 0, 1), [0, 0, 1]);
    assert_eq!(get_neighborhood(&[0, 1, 0, 1, 0].to_vec(), 4, 1), [1, 0, 0]);
}
fn step(input: &Vec<u32>, rules: Vec<RuleSegment>) -> Vec<u32> {
    let input_lenght = input.len();
    let mut output: Vec<u32> = vec![0; input_lenght];
    let neighborhood_size: usize = rules[0].neighborhood.len();
    let neighborhood_center = (neighborhood_size - 1) / 2;
    for i in 0..input_lenght {
        for rule in &rules {
            let current_neighborhood = get_neighborhood(input, i, neighborhood_center);
            if current_neighborhood == rule.neighborhood {
                output.push(rule.cell_type);
            }
        }
    }
    output
}

fn main() {
    println!("{:?}", generate_rule(103, 3, 2));
}
