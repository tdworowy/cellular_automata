use rand::Rng;
fn generate_gird_random(width: usize, height: usize, probability_of_one: f64) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for i in 0..width {
        grid.push(vec![]);
        for _ in 0..height {
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

fn count_colord_neighbours(x: usize, y: usize, grid: &Vec<Vec<u8>>) -> u8 {
    let mut count: u8 = 0;
    for i in (x - 1) % grid[0].iter().len()..(x + 2) % grid[0].iter().len() {
        for j in (y - 1) % grid[1].iter().len()..(y + 2) % grid[1].iter().len() {
            if grid[i][j] == 1 && (i, j) != (x, y) {
                count += 1;
            }
        }
    }
    count
}

#[test]
fn test_count_colord_neighbours() {
    assert_eq!(
        count_colord_neighbours(
            1,
            1,
            &vec![vec![1, 1, 0, 0], vec![0, 0, 1, 0], vec![0, 1, 0, 0],]
        ),
        4
    );
    assert_eq!(
        count_colord_neighbours(
            1,
            1,
            &vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 1],]
        ),
        8
    );
}

fn main() {
    let grid = generate_gird_random(5, 5, 1.0);
    println!("{:?}", &grid);
    println!("{:?}", count_colord_neighbours(2, 2, &grid));
}
