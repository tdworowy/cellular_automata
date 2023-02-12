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
const MAX_COLORS: u8 = 4;

fn get_colors() -> HashMap<u8, (f32, f32, f32)> {
    HashMap::from([
        (0, (0.0, 0.0, 1.0)), // blue
        (1, (1.0, 0.0, 0.0)), // red
        (2, (0.0, 1.0, 0.0)), // green
        (3, (1.0, 0.7, 0.0)), // yellow
    ])
}
const ELEMENTARY_RULES_NAMES: [&str; 15] = [
    "game_of_live",
    "ameba",
    "_2x2",
    "_34_live",
    "coagulations",
    "mazectric",
    "_move",
    "walled_cities",
    "epileptic",
    "snowflake_1",
    "snowflake_1_5",
    "snowflake_1_3_5",
    "snowflake_1_3",
    "snowflake_random",
    "random",
];

const CYCLICAL_RULES_NAMES: [&str; 2] = ["blob", "random"];
const TOTALISTIC_RULES_NAMES: [&str; 1] = ["random"];
// state, neighbourhood, color to count, new state
struct RulesCyclical {
    blob: HashMap<(u8, u8, u8), u8>,
}

// state, live neighbourhood, new state
struct RulesElementary {
    game_of_live: HashMap<(u8, u8), u8>,
    ameba: HashMap<(u8, u8), u8>,
    _2x2: HashMap<(u8, u8), u8>,
    _34_live: HashMap<(u8, u8), u8>,
    coagulations: HashMap<(u8, u8), u8>,
    mazectric: HashMap<(u8, u8), u8>,
    _move: HashMap<(u8, u8), u8>,
    walled_cities: HashMap<(u8, u8), u8>,
    epileptic: HashMap<(u8, u8), u8>,
}

impl RulesElementary {
    fn new() -> Self {
        Self {
            game_of_live: HashMap::from([((0, 3), 1), ((1, 3), 1), ((1, 2), 1)]),
            ameba: HashMap::from([
                ((0, 3), 1),
                ((0, 5), 1),
                ((0, 5), 1),
                ((1, 1), 1),
                ((1, 3), 1),
                ((1, 5), 1),
                ((1, 8), 1),
            ]),
            _2x2: HashMap::from([
                ((0, 3), 1),
                ((0, 6), 1),
                ((1, 1), 1),
                ((1, 2), 1),
                ((1, 5), 1),
            ]),
            _34_live: HashMap::from([((0, 3), 1), ((0, 4), 1), ((1, 3), 1), ((1, 4), 1)]),
            coagulations: HashMap::from([
                ((0, 3), 1),
                ((0, 7), 1),
                ((0, 8), 1),
                ((1, 2), 1),
                ((1, 3), 1),
                ((1, 5), 1),
                ((1, 6), 1),
                ((1, 7), 1),
                ((1, 8), 1),
            ]),
            mazectric: HashMap::from([
                ((0, 3), 1),
                ((1, 1), 1),
                ((1, 2), 1),
                ((1, 3), 1),
                ((1, 4), 1),
            ]),
            _move: HashMap::from([
                ((0, 3), 1),
                ((0, 6), 1),
                ((0, 8), 1),
                ((1, 2), 1),
                ((1, 4), 1),
                ((1, 5), 1),
            ]),
            walled_cities: HashMap::from([
                ((0, 4), 1),
                ((0, 5), 1),
                ((0, 6), 1),
                ((0, 7), 1),
                ((0, 8), 1),
                ((1, 2), 1),
                ((1, 3), 1),
                ((1, 4), 1),
                ((1, 5), 1),
            ]),
            epileptic: HashMap::from([((0, 0), 1), ((0, 2), 1)]),
        }
    }
}

impl RulesCyclical {
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

fn generate_snowflake_rule(neighbours_numbers: Vec<u8>) -> HashMap<(u8, u8), u8> {
    let mut rules: HashMap<(u8, u8), u8> = HashMap::new();
    for number in neighbours_numbers {
        rules.insert((0 as u8, number), 1 as u8);
        rules.insert((1 as u8, number), 1 as u8);
    }
    println!("Rule Details :{:?}", rules);
    rules
}

fn generate_snowflake_rule_random() -> HashMap<(u8, u8), u8> {
    let rule_lenght = thread_rng().gen_range(1..=8);
    let neighbours_numbers: Vec<u8> = (1..rule_lenght)
        .map(|_| thread_rng().gen_range(1..rule_lenght))
        .collect();
    generate_snowflake_rule(neighbours_numbers)
}

#[test]
fn test_generate_snowflake_rule() {
    assert_eq!(
        generate_snowflake_rule(vec![1, 3, 5]),
        HashMap::from([
            ((0, 1), 1),
            ((1, 1), 1),
            ((0, 3), 1),
            ((1, 3), 1),
            ((0, 5), 1),
            ((1, 5), 1)
        ])
    );
}
fn generate_random_rule_elementary(min_lenght: usize, max_lenght: usize) -> HashMap<(u8, u8), u8> {
    let rule_lenght = thread_rng().gen_range(min_lenght..max_lenght);
    let mut rules: HashMap<(u8, u8), u8> = HashMap::new();
    for _ in 0..rule_lenght {
        let first = thread_rng().gen_range(0..2) as u8;
        let second = thread_rng().gen_range(0..8) as u8;
        rules.insert((first, second), 1 as u8);
    }
    println!("Rule Details :{:?}", rules);
    rules
}

fn generate_random_rule_cyclical(colour_count: u8) -> HashMap<(u8, u8, u8), u8> {
    let mut rules: HashMap<(u8, u8, u8), u8> = HashMap::new();
    for color in 0..colour_count {
        let threshold_of_next_color = thread_rng().gen_range(0..8) as u8;
        let next_color = (color + 1) % colour_count;
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
fn generate_random_rule_totalistic(
    totalistic_type: TotalisticType,
    colour_count: u8,
) -> HashMap<u8, u8> {
    let mut rules: HashMap<u8, u8> = HashMap::new();
    let max = match totalistic_type {
        TotalisticType::Sum => 9 * colour_count,
        TotalisticType::Average => colour_count,
    };
    let mut thresholds_of_next_color: Vec<u8> = (0..=max).collect();
    for color in 0..colour_count {
        let threshold_of_next_color = choose(&mut thresholds_of_next_color).unwrap();
        let next_color = color;
        for i in 0..=threshold_of_next_color {
            rules.insert(i, next_color);
        }
    }
    println!("Rule Details :{:?}", rules);
    rules
}

fn get_rule_elementary(rule_name: &str) -> HashMap<(u8, u8), u8> {
    let rules = RulesElementary::new();
    match rule_name {
        "game_of_live" => rules.game_of_live,
        "ameba" => rules.ameba,
        "_2x2" => rules._2x2,
        "_34_live" => rules._34_live,
        "coagulations" => rules.coagulations,
        "mazectric" => rules.mazectric,
        "_move" => rules._move,
        "walled_cities" => rules.walled_cities,
        "epileptic" => rules.epileptic,
        "snowflake_1" => generate_snowflake_rule(vec![1]),
        "snowflake_1_5" => generate_snowflake_rule(vec![1, 5]),
        "snowflake_1_3_5" => generate_snowflake_rule(vec![1, 3, 5]),
        "snowflake_1_3" => generate_snowflake_rule(vec![1, 3]),
        "snowflake_random" => generate_snowflake_rule_random(),
        "random" => generate_random_rule_elementary(5, 15),
        _ => panic!(
            "rule {} doesn't exist, avilable rules: {:?}",
            &rule_name, ELEMENTARY_RULES_NAMES
        ),
    }
}

fn get_rule_cyclical(rule_name: &str, colour_count: u8) -> HashMap<(u8, u8, u8), u8> {
    let rules = RulesCyclical::new();
    match rule_name {
        "blob" => rules.blob,
        "random" => generate_random_rule_cyclical(colour_count),
        _ => panic!(
            "rule {} doesn't exist, avilable rules: {:?}",
            &rule_name, CYCLICAL_RULES_NAMES
        ),
    }
}

fn get_rule_totalistic(
    rule_name: &str,
    totalistic_type: TotalisticType,
    colour_count: u8,
) -> HashMap<u8, u8> {
    match rule_name {
        "random" => generate_random_rule_totalistic(totalistic_type, colour_count),
        _ => panic!(
            "rule {} doesn't exist, avilable rules: {:?}",
            &rule_name, TOTALISTIC_RULES_NAMES
        ),
    }
}

// fn generate_gird_random(width: usize, height: usize, colour_count: u8) -> Vec<Vec<u8>> {
//     let mut grid: Vec<Vec<u8>> = Vec::new();
//     for i in 0..height {
//         grid.push(vec![]);
//         for _ in 0..width {
//             let cell_type = thread_rng().gen_range(0..colour_count);
//             grid[i].push(cell_type);
//         }
//     }
//     grid
// }

fn generate_gird_random(width: usize, height: usize, colour_count: u8) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    (0..height)
        .map(|_| (0..width).map(|_| rng.gen_range(0..colour_count)).collect())
        .collect()
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

fn get_range(y: usize, x: usize, grid: &[Vec<u8>]) -> (usize, usize, usize, usize) {
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
        resoult /= 9
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
#[derive(Debug, PartialEq)]
enum RuleType {
    Elementary,
    ElementarySonwflake,
    Cyclical,
    TotalisticSum,
    TotalisticAverage,
}

enum ElementaryType {
    Elementary,
    Sonwflake,
}

fn update_grid_elementary(
    rule_type: ElementaryType,
    grid: &Vec<Vec<u8>>,
    rules: &HashMap<(u8, u8), u8>,
) -> Vec<Vec<u8>> {
    let mut new_grid: Vec<Vec<u8>> = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        let mut new_row: Vec<u8> = Vec::new();
        for (j, cell) in row.iter().enumerate() {
            let live_neighbours = count_colored_neighbours(i, j, 1, grid);
            let state = *cell;
            match rule_type {
                ElementaryType::Elementary => {
                    new_row.push(*rules.get(&(state, live_neighbours)).unwrap_or(&0))
                }
                ElementaryType::Sonwflake => {
                    new_row.push(*rules.get(&(state, live_neighbours)).unwrap_or(&state))
                }
            }
        }

        new_grid.push(new_row);
    }
    new_grid
}

fn update_grid_cyclical(
    grid: &Vec<Vec<u8>>,
    rules: &HashMap<(u8, u8, u8), u8>,
    colour_count: u8,
) -> Vec<Vec<u8>> {
    let mut new_grid: Vec<Vec<u8>> = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        let mut new_row: Vec<u8> = Vec::new();
        for (j, cell) in row.iter().enumerate() {
            let state = *cell;
            let next_color = (state + 1) % colour_count;
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
            let aggregated_value = aggregate_colored_neighbours(i, j, &aggregation, grid);

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
    rules_elementary: Option<HashMap<(u8, u8), u8>>,
    rules_cyclical: Option<HashMap<(u8, u8, u8), u8>>,
    rules_totalistic: Option<HashMap<u8, u8>>,
    rule_type: RuleType,
    colour_count: u8,
}

impl Application for CellularAutomata2D {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let (rule_type, rule_name, colour_count) = read_args();

        let init_grid = match rule_type {
            RuleType::ElementarySonwflake => generate_gird_one_cell(WIDTH, HEIGHT),
            _ => generate_gird_random(WIDTH, HEIGHT, colour_count),
        };

        let cellular_automata_2_d = match rule_type {
            RuleType::Elementary => CellularAutomata2D {
                cache: Default::default(),
                grid: init_grid,
                rules_elementary: Some(get_rule_elementary(&rule_name)),
                rules_cyclical: None,
                rules_totalistic: None,
                rule_type: RuleType::Elementary,
                colour_count,
            },
            RuleType::ElementarySonwflake => CellularAutomata2D {
                cache: Default::default(),
                grid: init_grid,
                rules_elementary: Some(get_rule_elementary(&rule_name)),
                rules_cyclical: None,
                rules_totalistic: None,
                rule_type: RuleType::ElementarySonwflake,
                colour_count,
            },
            RuleType::Cyclical => CellularAutomata2D {
                cache: Default::default(),
                grid: init_grid,
                rules_elementary: None,
                rules_cyclical: Some(get_rule_cyclical(&rule_name, colour_count)),
                rules_totalistic: None,
                rule_type: RuleType::Cyclical,
                colour_count,
            },
            RuleType::TotalisticAverage => CellularAutomata2D {
                cache: Default::default(),
                grid: init_grid,
                rules_elementary: None,
                rules_cyclical: None,
                rules_totalistic: Some(get_rule_totalistic(
                    &rule_name,
                    TotalisticType::Average,
                    colour_count,
                )),
                rule_type: RuleType::TotalisticAverage,
                colour_count,
            },
            RuleType::TotalisticSum => CellularAutomata2D {
                cache: Default::default(),
                grid: init_grid,
                rules_elementary: None,
                rules_cyclical: None,
                rules_totalistic: Some(get_rule_totalistic(
                    &rule_name,
                    TotalisticType::Sum,
                    colour_count,
                )),
                rule_type: RuleType::TotalisticSum,
                colour_count,
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
            RuleType::Elementary | RuleType::ElementarySonwflake => None,
            RuleType::Cyclical => None,
        };

        match message {
            Message::Tick(_local_time) => match &self.rule_type {
                RuleType::Elementary => {
                    self.grid = update_grid_elementary(
                        ElementaryType::Elementary,
                        &self.grid,
                        self.rules_elementary.as_ref().unwrap(),
                    );
                    self.cache.clear();
                }
                RuleType::ElementarySonwflake => {
                    self.grid = update_grid_elementary(
                        ElementaryType::Sonwflake,
                        &self.grid,
                        self.rules_elementary.as_ref().unwrap(),
                    );
                    self.cache.clear();
                }
                RuleType::Cyclical => {
                    self.grid = update_grid_cyclical(
                        &self.grid,
                        self.rules_cyclical.as_ref().unwrap(),
                        self.colour_count,
                    );
                    self.cache.clear();
                }
                RuleType::TotalisticAverage | RuleType::TotalisticSum => {
                    self.grid = update_grid_totalistic(
                        totalistic_type.unwrap(),
                        &self.grid,
                        self.rules_totalistic.as_ref().unwrap(),
                    );
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

fn read_args() -> (RuleType, String, u8) {
    let args: Vec<String> = env::args().collect();
    let rule_name = if args.len() > 1 { &args[1] } else { "default" };
    let rule_type = match rule_name {
        "elementary" => RuleType::Elementary,
        "elementary_sonwflake" => RuleType::ElementarySonwflake,
        "cyclical" => RuleType::Cyclical,
        "totalistic_sum" => RuleType::TotalisticSum,
        "totalistic_average" => RuleType::TotalisticAverage,
        _ => RuleType::Elementary,
    };

    println!("using rule type: {:?}", rule_type);
    let rule_name = if args.len() < 3 {
        println!("using default rule: random");
        "random"
    } else {
        println!("Using rule: {}", &args[2]);
        &args[2]
    }
    .to_string();

    let colour_count: u8 =
        if rule_type == RuleType::Elementary || rule_type == RuleType::ElementarySonwflake {
            2
        } else if args.len() < 3 {
            MAX_COLORS
        } else {
            let temp = &args[3].parse::<u8>();
            let result = match temp {
                Ok(n) => n,
                Err(e) => panic!("{}", e),
            };
            *result
        };
    println!("Color Count:{} max colors:{}", colour_count, MAX_COLORS);
    (rule_type, rule_name, colour_count)
}

fn main() -> iced::Result {
    env_logger::builder().format_timestamp(None).init();

    CellularAutomata2D::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
