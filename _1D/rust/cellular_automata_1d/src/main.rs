use itertools::{Itertools, MultiProduct};
use std::collections::HashMap;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

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

fn generate_row_random(width: usize, probability_of_one: f64) -> Vec<u8> {
    let mut row: Vec<u8> = Vec::new();
    for i in 0..width {
        let is_one = thread_rng().gen_bool(probability_of_one);
        let cell_type = if is_one { 1 } else { 0 };
        row[i].push(cell_type as u8);
    }

    row
}

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

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(time::OffsetDateTime),
}

struct CellularAutomata1D {
    cache: Cache,
    rule: HashMap<(u8, u8), u8>,
}

impl Application for CellularAutomata1D {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let rule = generate_rule(110, 3, 2);
        (
            CellularAutomata1D {
                cache: Default::default(),
                rule: rule,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Cellular Automata 1D")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        Canvas::new(self)
            .width(Length::Units(WIDTH as u16 * 2))
            .height(Length::Units(HEIGHT as u16 * 2))
            .into()
    }
}

impl<Message> canvas::Program<Message> for CellularAutomata1D {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(bounds.size(), |frame| {
            let mut x: f32 = 0.0;
            let mut y: f32 = 0.0;
            let mut row = generate_row_random(WIDTH, 0.3);
            for _ in 0..HEIGHT {
                for cell in row {
                    let color = get_colors()[cell];
                    generate_box(frame, x, y, Color::from_rgb(color.0, color.1, color.2));
                    x += 2.0;
                }
                row = step(row, rule);
                x = 0.0;
                y += 2.0;
            }
        });

        vec![geometry]
    }
}

fn generate_box(frame: &mut Frame, x: f32, y: f32, color: Color) {
    let top_left = Point::new(x, y);
    let size = Size::new(2.0, 2.0);
    frame.fill_rectangle(top_left, size, color);
}

fn main() -> iced::Result {
    env_logger::builder().format_timestamp(None).init();

    CellularAutomata2D::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
