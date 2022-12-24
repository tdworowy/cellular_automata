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

// TODO handle Totalistic and Cyclical rules, read parameter rule type and totalistic type

fn get_colors() -> HashMap<u8, (f32, f32, f32)> {
    HashMap::from([
        (0, (0.0, 0.0, 1.0)), // blue
        (1, (1.0, 0.0, 0.0)), // red
        (2, (0.0, 1.0, 0.0)), // green
        (3, (1.0, 0.7, 0.0)), // yellow
    ])
}

const RULES_NAMES: [&str; 1] = ["blob"];
struct Rules {
    blob: HashMap<(u8, u8, u8), u8>,
    // state, neighbourhood, color to count, new state
}

impl Rules {
    fn new() -> Self {
        Self {
            blob: HashMap::from([
                ((3, 5, 0), 0),
                ((1, 3, 2), 2),
                ((3, 6, 0), 0),
                ((0, 3, 1), 1),
                ((2, 1, 3), 3),
                ((0, 5, 1), 1),
                ((2, 0, 3), 3),
                ((3, 3, 0), 0),
                ((3, 7, 0), 0),
                ((1, 1, 2), 2),
                ((0, 4, 1), 1),
                ((2, 2, 3), 3),
                ((1, 2, 2), 2),
                ((2, 3, 3), 3),
                ((2, 6, 3), 3),
                ((3, 4, 0), 0),
                ((0, 6, 1), 1),
                ((1, 4, 2), 2),
                ((1, 5, 2), 2),
                ((1, 7, 2), 2),
                ((2, 7, 3), 3),
                ((0, 7, 1), 1),
                ((3, 0, 0), 0),
                ((2, 5, 3), 3),
                ((3, 1, 0), 0),
                ((3, 2, 0), 0),
                ((2, 4, 3), 3),
                ((1, 6, 2), 2),
            ]),
        }
    }
}

fn generate_random_rule_cyclical() -> HashMap<(u8, u8, u8), u8> {
    let mut rules: HashMap<(u8, u8, u8), u8> = HashMap::new();
    for color in 0..COLOUR_COUNT {
        let threshold_of_next_color = thread_rng().gen_range(0..8) as u8;
        let next_color = (color + 1) % COLOUR_COUNT;
        for i in threshold_of_next_color..8 {
            rules.insert((color, i, next_color), next_color);
        }
    }
    rules
}
#[derive(PartialEq)]
enum TotalisticType {
    sum,
    average,
}
// sum (or averagre) for 9 cells, new state
fn generate_random_rule_totalistic(totalistic_type: TotalisticType) -> HashMap<u8, u8> {
    let mut rules: HashMap<u8, u8> = HashMap::new();
    let max = match totalistic_type {
        TotalisticType::sum => 9 * COLOUR_COUNT,
        TotalisticType::average => COLOUR_COUNT - 1,
    };
    for color in 0..COLOUR_COUNT {
        let threshold_of_next_color = thread_rng().gen_range(0..max) as u8;
        let next_color = (color + 1) % COLOUR_COUNT;
        for i in threshold_of_next_color..8 {
            rules.insert(i, next_color);
        }
    }
    rules
}

fn get_rule(rule_name: &str) -> HashMap<(u8, u8, u8), u8> {
    let rules = Rules::new();
    match rule_name {
        "blob" => rules.blob,
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

fn get_range(y: usize, x: usize, grid: &Vec<Vec<u8>>) -> (usize, usize, usize, usize) {
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
    (x_start, y_start, x_end, y_end)
}

fn count_colored_neighbours(y: usize, x: usize, color_to_count: u8, grid: &Vec<Vec<u8>>) -> u8 {
    let mut count: u8 = 0;
    let (x_start, y_start, x_end, y_end) = get_range(y, x, grid);

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

fn aggregate_colored_neighbours(
    y: usize,
    x: usize,
    aggregation: &TotalisticType,
    grid: &Vec<Vec<u8>>,
) -> u8 {
    let mut resoult = 0;
    let (x_start, y_start, x_end, y_end) = get_range(y, x, grid);
    for i in y_start..y_end {
        for j in x_start..x_end {
            resoult += grid[i][j];
        }
    }
    if aggregation == &TotalisticType::average {
        resoult = resoult / 9
    }
    resoult
}

#[test]
fn test_aggregate_colored_neighbours() {
    assert_eq!(
        aggregate_colored_neighbours(
            1,
            1,
            &TotalisticType::sum,
            &vec![vec![1, 1, 0, 0], vec![3, 2, 1, 0], vec![0, 1, 0, 0],]
        ),
        9
    );
    assert_eq!(
        aggregate_colored_neighbours(
            1,
            1,
            &TotalisticType::average,
            &vec![vec![2, 2, 2, 0], vec![3, 2, 3, 0], vec![2, 2, 2, 0],]
        ),
        2
    );
}

enum RuleType {
    cyclical,
    totalistic,
}

fn update_grid_cyclical(grid: &Vec<Vec<u8>>, rules: &HashMap<(u8, u8, u8), u8>) -> Vec<Vec<u8>> {
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
                    .unwrap_or(&state),
            );
        }
        new_grid.push(new_row);
    }
    new_grid
}

fn update_grid_totalistic(
    aggregation: TotalisticType,
    grid: &Vec<Vec<u8>>,
    rules: &HashMap<u8, u8>,
) -> Vec<Vec<u8>> {
    let mut new_grid: Vec<Vec<u8>> = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        let mut new_row: Vec<u8> = Vec::new();
        for (j, cell) in row.iter().enumerate() {
            let state = *cell;
            let aggregated_value = aggregate_colored_neighbours(i, j, &aggregation, &grid);

            new_row.push(*rules.get(&aggregated_value).unwrap_or(&state));
        }
        new_grid.push(new_row);
    }
    new_grid
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(time::OffsetDateTime),
}

struct CellularAutomata2D {
    cache: Cache,
    grid: Vec<Vec<u8>>,
    rules_cyclical: Option<HashMap<(u8, u8, u8), u8>>,
    rules_totalistic: Option<HashMap<u8, u8>>,
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
                rules_cyclical: Some(rule),
                rules_totalistic: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Cellular Automata 2D")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(_local_time) => match &self.rules_cyclical {
                None => {
                    println!("That shoulnd not happen")
                }
                Some(rule) => {
                    self.grid = update_grid_cyclical(&self.grid, &rule);
                    self.cache.clear();
                }
            },
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
