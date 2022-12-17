use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::env;

use iced::widget::canvas::{self, Cache, Canvas, Cursor, Frame, Geometry};
use iced::{
    executor, Application, Color, Command, Element, Length, Point, Rectangle, Renderer, Settings,
    Size, Subscription, Theme,
};

const WIDTH: usize = 500;
const HEIGHT: usize = 500;
const TICK_TIME: u64 = 100;

const COLOUR_COUNT: u8 = 4;

fn get_colors() -> HashMap<u8, (f32, f32, f32)> {
    HashMap::from([
        (0, (0.0, 0.0, 1.0)), // blue
        (1, (1.0, 0.0, 0.0)), // red
        (2, (0.0, 1.0, 0.0)), // green
        (3, (1.0, 0.7, 0.0)), // yellow
    ])
}

const RULES_NAMES: [&str; 1] = ["placeholder"];
struct Rules {
    placeholder: HashMap<(u8, u8, u8), u8>,
    // state, neighbourhood, color to count, new state
}

impl Rules {
    fn new() -> Self {
        Self {
            placeholder: HashMap::from([
                ((0, 3, 1), 1),
                ((1, 3, 2), 2),
                ((2, 3, 2), 3),
                ((3, 2, 3), 4),
                ((3, 3, 1), 1),
            ]),
        }
    }
}

fn generate_random_rule_cyclical() -> HashMap<(u8, u8, u8), u8> {
    let mut rules: HashMap<(u8, u8, u8), u8> = HashMap::new();
    for color in 0..COLOUR_COUNT {
        let threshold_of_next_color = thread_rng().gen_range(0..8) as u8;
        let mut next_color = (color + 1) % COLOUR_COUNT;
        next_color = if next_color == 0 { 1 } else { next_color };
        for i in threshold_of_next_color..8 {
            rules.insert((color, i, next_color), next_color);
        }
    }
    rules
}

fn get_rule(rule_name: &str) -> HashMap<(u8, u8, u8), u8> {
    let rules = Rules::new();
    match rule_name {
        "placeholder" => rules.placeholder,
        "random" => generate_random_rule_cyclical(),
        _ => panic!(
            "rule {} doesn't exist, avilable rules: {:?}",
            &rule_name, RULES_NAMES
        ),
    }
}

fn generate_gird_random(width: usize, height: usize) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for i in 0..height {
        grid.push(vec![]);
        for _ in 0..width {
            let cell_type = thread_rng().gen_range(0..COLOUR_COUNT);
            grid[i].push(cell_type);
        }
    }
    grid
}

fn generate_gird_one_cell(width: usize, height: usize) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = vec![vec![0; height]; width];
    grid[width / 2][height / 2] = 1;
    grid
}

#[test]
fn test_generate_gird_one_cell() {
    assert_eq!(
        generate_gird_one_cell(3, 3),
        [[0, 0, 0], [0, 1, 0], [0, 0, 0]]
    );
}

fn count_colored_neighbours(y: usize, x: usize, color_to_count: u8, grid: &Vec<Vec<u8>>) -> u8 {
    let mut count: u8 = 0;
    let x_start = if x as isize - 1 <= 0 {
        0
    } else {
        x as isize - 1
    } as usize;
    let y_start = if y as isize - 1 <= 0 {
        0
    } else {
        y as isize - 1
    } as usize;

    let x_end = if x + 2 >= grid[0].iter().len() {
        grid[0].iter().len()
    } else {
        x + 2
    };
    let y_end = if y + 2 >= grid.iter().len() {
        grid.iter().len()
    } else {
        y + 2
    };

    for i in y_start..y_end {
        for j in x_start..x_end {
            if grid[i][j] == color_to_count && (i, j) != (y, x) {
                count += 1;
            }
        }
    }
    count
}

#[test]
fn test_count_colored_neighbours() {
    assert_eq!(
        count_colored_neighbours(
            1,
            1,
            1,
            &vec![vec![1, 1, 0, 0], vec![0, 0, 1, 0], vec![0, 1, 0, 0],]
        ),
        4
    );
    assert_eq!(
        count_colored_neighbours(
            1,
            1,
            2,
            &vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 0, 0, 0],]
        ),
        0
    );
    assert_eq!(
        count_colored_neighbours(
            0,
            0,
            3,
            &vec![vec![1, 3, 0, 0], vec![3, 3, 0, 1], vec![0, 0, 1, 0]],
        ),
        3
    );
    assert_eq!(
        count_colored_neighbours(1, 1, 1, &vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]],),
        8
    );
    assert_eq!(
        count_colored_neighbours(
            1,
            1,
            1,
            &vec![vec![1, 1, 1, 0], vec![1, 0, 1, 0], vec![1, 1, 1, 0]],
        ),
        8
    );
    assert_eq!(
        count_colored_neighbours(
            1,
            1,
            2,
            &vec![vec![2, 2, 2, 0], vec![2, 0, 2, 1], vec![2, 2, 2, 0]],
        ),
        8
    );
    assert_eq!(
        count_colored_neighbours(
            0,
            3,
            1,
            &vec![vec![1, 1, 0, 0], vec![1, 0, 0, 1], vec![0, 0, 1, 0]],
        ),
        1
    );
    assert_eq!(
        count_colored_neighbours(
            2,
            3,
            1,
            &vec![vec![1, 1, 0, 0], vec![1, 0, 0, 1], vec![0, 0, 1, 0]],
        ),
        2
    );
}

fn update_grid(grid: &Vec<Vec<u8>>, rules: &HashMap<(u8, u8, u8), u8>) -> Vec<Vec<u8>> {
    let mut new_grid: Vec<Vec<u8>> = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        let mut new_row: Vec<u8> = Vec::new();
        for (j, cell) in row.iter().enumerate() {
            let state = *cell;
            let next_color = (state + 1) % COLOUR_COUNT;
            let live_neighbours = count_colored_neighbours(i, j, next_color, &grid);

            new_row.push(
                *rules
                    .get(&(state, live_neighbours, next_color))
                    .unwrap_or(&0),
            );
        }
        new_grid.push(new_row);
    }
    new_grid
}

#[test]
fn test_update_grid() {
    assert_eq!(
        update_grid(
            &vec![vec![1, 2, 0, 0], vec![2, 2, 0, 1], vec![0, 0, 1, 1]],
            &get_rule("placeholder")
        ),
        [[2, 0, 0, 0], [0, 0, 1, 0], [0, 0, 0, 0]]
    );
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(time::OffsetDateTime),
}

struct CellularAutomata2D {
    cache: Cache,
    grid: Vec<Vec<u8>>,
    rules: HashMap<(u8, u8, u8), u8>,
}

impl Application for CellularAutomata2D {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let rule = read_rule();
        let init_grid = generate_gird_random(WIDTH, HEIGHT);
        (
            CellularAutomata2D {
                cache: Default::default(),
                grid: init_grid,
                rules: rule,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Cellular Automata 2D")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(_local_time) => {
                self.grid = update_grid(&self.grid, &self.rules);
                self.cache.clear();
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        Canvas::new(self)
            .width(Length::Units(WIDTH as u16 * 2))
            .height(Length::Units(HEIGHT as u16 * 2))
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(TICK_TIME)).map(|_| {
            Message::Tick(
                time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
            )
        })
    }
}

impl<Message> canvas::Program<Message> for CellularAutomata2D {
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
            for row in &self.grid {
                for cell in row {
                    let color = get_colors()[cell];
                    generate_box(frame, x, y, Color::from_rgb(color.0, color.1, color.2));
                    x += 2.0;
                }
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

fn read_rule() -> HashMap<(u8, u8, u8), u8> {
    let args: Vec<String> = env::args().collect();
    let mut rule: HashMap<(u8, u8, u8), u8> = get_rule("random");
    if args.len() != 2 {
        println!("using default rule: random");
        println!("avilable rules: {:?}", RULES_NAMES);
    } else {
        rule = get_rule(&args[1]);
        println!("Using rule:{}", &args[1]);
    }
    println!("Rule Details:{:?}", rule);
    rule
}

fn main() -> iced::Result {
    env_logger::builder().format_timestamp(None).init();

    CellularAutomata2D::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
