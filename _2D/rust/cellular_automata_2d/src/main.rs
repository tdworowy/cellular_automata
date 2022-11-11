use rand::{thread_rng, Rng};
use std::collections::HashMap;

use iced::widget::canvas::{self, Cache, Canvas, Cursor, Frame, Geometry};
use iced::{
    executor, Application, Color, Command, Element, Length, Point, Rectangle, Renderer, Settings,
    Size, Subscription, Theme,
};

const WIDTH: usize = 500;
const HEIGHT: usize = 500;
const TICK_TIME: u64 = 200;

fn get_game_of_live_rules() -> HashMap<(u8, u8), u8> {
    HashMap::from([((0, 3), 1), ((1, 3), 1), ((1, 2), 1)])
}

fn generate_gird_random(width: usize, height: usize, probability_of_one: f64) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for i in 0..height {
        grid.push(vec![]);
        for _ in 0..width {
            let is_one = thread_rng().gen_bool(probability_of_one);
            let cell_type = if is_one { 1 } else { 0 };
            grid[i].push(cell_type as u8);
        }
    }
    grid
}

#[test]
fn test_generate_gird_random() {
    assert_eq!(generate_gird_random(2, 2, 0.0), [[0, 0], [0, 0]]);
    assert_eq!(generate_gird_random(2, 2, 1.0), [[1, 1], [1, 1]]);
    assert_eq!(generate_gird_random(3, 2, 1.0), [[1, 1, 1], [1, 1, 1]]);
    assert_eq!(generate_gird_random(2, 3, 1.0), [[1, 1], [1, 1], [1, 1]]);
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

fn count_colored_neighbours(y: usize, x: usize, grid: &Vec<Vec<u8>>) -> u8 {
    let mut count: u8 = 0;
    let x_start = if x as isize - 1 <= 0 {
        0
    } else {
        x as isize - 1
    };
    let y_start = if y as isize - 1 <= 0 {
        0
    } else {
        y as isize - 1
    };

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

    for i in y_start as usize..y_end {
        for j in x_start as usize..x_end {
            if grid[i][j] == 1 && (i, j) != (y, x) {
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
            &vec![vec![1, 1, 0, 0], vec![0, 0, 1, 0], vec![0, 1, 0, 0],]
        ),
        4
    );
    assert_eq!(
        count_colored_neighbours(
            1,
            1,
            &vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 1],]
        ),
        8
    );
    assert_eq!(
        count_colored_neighbours(
            0,
            0,
            &vec![vec![1, 1, 0, 0], vec![1, 0, 0, 1], vec![0, 0, 1, 0]],
        ),
        2
    );
    assert_eq!(
        count_colored_neighbours(
            1,
            2,
            &vec![vec![1, 1, 0, 0], vec![1, 0, 0, 1], vec![0, 0, 1, 0]],
        ),
        3
    );
    assert_eq!(
        count_colored_neighbours(
            0,
            3,
            &vec![vec![1, 1, 0, 0], vec![1, 0, 0, 1], vec![0, 0, 1, 0]],
        ),
        1
    );
    assert_eq!(
        count_colored_neighbours(
            2,
            3,
            &vec![vec![1, 1, 0, 0], vec![1, 0, 0, 1], vec![0, 0, 1, 0]],
        ),
        2
    );
}

fn update_grid(grid: &Vec<Vec<u8>>, rules: &HashMap<(u8, u8), u8>) -> Vec<Vec<u8>> {
    let mut new_grid = grid.clone();
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let live_neighbours = count_colored_neighbours(i, j, &grid);
            let state = *cell;
            new_grid[i][j] = *rules.get(&(state, live_neighbours)).clone().unwrap_or(&0);
        }
    }
    new_grid
}

#[test]
fn test_update_grid() {
    assert_eq!(
        update_grid(
            &vec![vec![1, 1, 0, 0], vec![1, 0, 0, 1], vec![0, 0, 1, 0]],
            &get_game_of_live_rules()
        ),
        [[1, 1, 0, 0], [1, 0, 1, 0], [0, 0, 0, 0]]
    );
}

fn test_console() {
    let mut grid = generate_gird_random(WIDTH, HEIGHT, 0.4);
    let rules = get_game_of_live_rules();
    for i in 0..100 {
        grid = update_grid(&grid, &rules);
        println!("Step {}", i);
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(time::OffsetDateTime),
}
struct CellularAutomata2D {
    cache: Cache,
    grid: Vec<Vec<u8>>,
    rules: HashMap<(u8, u8), u8>,
}

impl Application for CellularAutomata2D {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            CellularAutomata2D {
                cache: Default::default(),
                grid: generate_gird_random(WIDTH, HEIGHT, 0.4),
                rules: get_game_of_live_rules(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Cellular Automata 2D")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(local_time) => {
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
                    let color = if cell == &1 {
                        Color::from_rgb(1.0, 0.0, 0.0)
                    } else {
                        Color::from_rgb(0.0, 0.0, 1.0)
                    };
                    generate_box(frame, x, y, color);
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
fn main() -> iced::Result {
    env_logger::builder().format_timestamp(None).init();

    CellularAutomata2D::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
