use iced::{Sandbox, Length, Rectangle, futures::io::Cursor, Point, Size};

use rand::Rng;
use std::{collections::HashMap, path::Path};


const WIDTH: usize = 200;
const HEIGHT: usize = 200;

fn get_game_of_live_rules() -> HashMap<(u8, u8), u8> {
    HashMap::from([((0, 3), 1), ((1, 3), 1), ((1, 2), 1)])
}

fn generate_gird_random(width: usize, height: usize, probability_of_one: f64) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for i in 0..height {
        grid.push(vec![]);
        for _ in 0..width {
            let is_one = rand::thread_rng().gen_bool(probability_of_one);
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


// TODO make it work
// struct RectangleApp;

// impl Sandbox for RectangleApp {
//     type Message = ();

//     fn new() -> Self {
//         Self
//     }

//     fn title(&self) -> String {
//         "My simple rectangle".into()
//     }

//     fn update(&mut self, _: ()) {}

//     fn view(&mut self) -> iced::Element<'_, Self::Message> {
//         Canvas::new(RectangleProgram)
//             .width(Length::Fill)
//             .height(Length::Fill)
//             .into()
//     }
// }

// struct RectangleProgram;

// impl Program<()> for RectangleProgram {
//     fn draw(&self, bounds: Rectangle, _: Cursor) -> Vec<Geometry> {
//         let mut frame = Frame::new(bounds.size());
//         frame.stroke(
//             &Path::rectangle(
//                 Point {
//                     x: bounds.width / 10.,
//                     y: bounds.height / 10.,
//                 },
//                 Size {
//                     width: 4. * bounds.width / 5.,
//                     height: 4. * bounds.height / 5.,
//                 },
//             ),
//             Stroke::default(),
//         );
//         vec![frame.into_geometry()]
//     }
// }

fn main() {
    //RectangleApp::run(Settings::default());
}
