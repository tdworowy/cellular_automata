use rand::seq::IteratorRandom;
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
    println!("Rule Details :{:?}", rules);
    rules
}
#[derive(PartialEq)]
enum TotalisticType {
    Sum,
    Average,
}

fn choose(raw: &mut Vec<u8>) -> Option<u8> {
    let i = (0..raw.len()).choose(&mut thread_rng())?;
    Some(raw.swap_remove(i))
}

#[test]
fn test_choose() {
    let mut test_vec: Vec<u8> = (0..4).collect();
    let element = choose(&mut test_vec);

    assert!(test_vec.len() == 3);
    assert!(!test_vec.contains(&element.unwrap()));

}
// sum (or averagre) for 9 cells, new state
fn generate_random_rule_totalistic(totalistic_type: TotalisticType) -> HashMap<u8, u8> {
    let mut rules: HashMap<u8, u8> = HashMap::new();
    let max = match totalistic_type {
        TotalisticType::Sum => 9 * COLOUR_COUNT,
        TotalisticType::Average => COLOUR_COUNT + 1,
    };
    let mut thresholds_of_next_color: Vec<u8> = (0..max).collect();
    for color in 0..COLOUR_COUNT  {
        let threshold_of_next_color = choose(&mut thresholds_of_next_color).unwrap();
        let next_color = color;
        for i in 0..threshold_of_next_color {
            rules.insert(i, next_color);
        }
    }
    println!("Rule Details :{:?}", rules);
    rules
}

fn get_rule_cyclical(rule_name: &str) -> HashMap<(u8, u8, u8), u8> {
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

fn get_rule_totalistic(rule_name: &str, totalistic_type: TotalisticType) -> HashMap<u8, u8> {
    match rule_name {
        "random" => generate_random_rule_totalistic(totalistic_type),
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
    if aggregation == &TotalisticType::Average {
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
            &TotalisticType::Sum,
            &vec![vec![1, 1, 0, 0], vec![3, 2, 1, 0], vec![0, 1, 0, 0],]
        ),
        9
    );
    assert_eq!(
        aggregate_colored_neighbours(
            1,
            1,
            &TotalisticType::Average,
            &vec![vec![2, 2, 2, 0], vec![3, 2, 3, 0], vec![2, 2, 2, 0],]
        ),
        2
    );
}

enum RuleType {
    Cyclical,
    TotalisticSum,
    TotalisticAverage,
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
    rule_type: RuleType,
}

impl Application for CellularAutomata2D {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let (rule_type, rule_name) = read_args();
        let init_grid = generate_gird_random(WIDTH, HEIGHT);

        let cellular_automata_2_d = match rule_type {
            RuleType::Cyclical => CellularAutomata2D {
                cache: Default::default(),
                grid: init_grid,
                rules_cyclical: Some(get_rule_cyclical(&rule_name)),
                rules_totalistic: None,
                rule_type: RuleType::Cyclical,
            },
            RuleType::TotalisticAverage => CellularAutomata2D {
                cache: Default::default(),
                grid: init_grid,
                rules_cyclical: None,
                rules_totalistic: Some(get_rule_totalistic(&rule_name, TotalisticType::Average)),
                rule_type: RuleType::TotalisticAverage,
            },
            RuleType::TotalisticSum => CellularAutomata2D {
                cache: Default::default(),
                grid: init_grid,
                rules_cyclical: None,
                rules_totalistic: Some(get_rule_totalistic(&rule_name, TotalisticType::Sum)),
                rule_type: RuleType::TotalisticSum,
            },
        };
        (cellular_automata_2_d, Command::none())
    }

    fn title(&self) -> String {
        String::from("Cellular Automata 2D")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        let totalistic_type: Option<TotalisticType> = match &self.rule_type {
            RuleType::TotalisticAverage => Some(TotalisticType::Average),
            RuleType::TotalisticSum => Some(TotalisticType::Sum),
            RuleType::Cyclical => None,
        };
        match message {
            Message::Tick(_local_time) => match &self.rules_cyclical {
                None => match &&self.rules_totalistic {
                    Some(rule) => {
                        self.grid =
                            update_grid_totalistic(totalistic_type.unwrap(), &self.grid, &rule);
                        self.cache.clear();
                    }
                    None => {
                        println!("That Should not Happen")
                    }
                },
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

fn read_args() -> (RuleType, String) {
    let args: Vec<String> = env::args().collect();
    let rule_name = &args[1];
    let rule_type = match rule_name.as_ref() {
        "cyclical" => RuleType::Cyclical,
        "totalistic_sum" => RuleType::TotalisticSum,
        "totalistic_average" => RuleType::TotalisticAverage,
        _ => RuleType::Cyclical,
    };
    let rule_name = if args.len() < 3 {
        println!("using default rule: random");
        "random"
    } else {
        println!("Using rule:{}", &args[2]);
        &args[2]
    }
    .to_string();

    (rule_type, rule_name)
}

fn main() -> iced::Result {
    env_logger::builder().format_timestamp(None).init();

    CellularAutomata2D::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
