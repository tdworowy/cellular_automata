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
fn main() {
    println!("{:?}", generate_gird_random(10, 10, 0.3));
}
