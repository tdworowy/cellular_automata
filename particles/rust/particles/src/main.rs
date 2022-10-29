use itertools::iproduct;
use rand::seq::SliceRandom;
use std::collections::HashMap;

static WIDTH: u16 = 1280;
static HEIGHT: u16 = 720;

fn get_colors() -> HashMap<u8, (u8, u8, u8)> {
    HashMap::from([
        (1, (0, 0, 255)),   // red
        (2, (255, 0, 0)),   // grean
        (3, (0, 255, 0)),   // blue
        (4, (255, 215, 0)), // yellow
    ])
}

#[derive(Debug)]
struct ParticleInfo {
    color: u8,
    x: u16,
    y: u16,
    vx: u16,
    vy: u16,
}

fn generate_init_particles(
    count: u16,
    color: u8,
    coordinates: Vec<(u16, u16)>,
) -> (Vec<ParticleInfo>, Vec<(u16, u16)>) {
    let mut init_partilces: Vec<ParticleInfo> = Vec::new();

    let coordinates_sample: Vec<(u16, u16)> = coordinates
        .choose_multiple(&mut rand::thread_rng(), count as usize)
        .cloned()
        .collect::<Vec<(u16, u16)>>();

    for i in 0..count {
        let temp_tuple = coordinates_sample[i as usize];
        init_partilces.push(ParticleInfo {
            color: color,
            x: temp_tuple.0,
            y: temp_tuple.1,
            vx: 0,
            vy: 0,
        });
    }
    (init_partilces, coordinates_sample)
}

fn main() {
    let X: Vec<u16> = (0..WIDTH).collect();
    let Y: Vec<u16> = (0..HEIGHT).collect();
    let coordinates: Vec<(u16, u16)> = iproduct!(X, Y).collect();

    let result = generate_init_particles(200, 1, coordinates);
    let init_red_particles = result.0;
   
    let used_coordinates = result.1;  // TODO filter coordinates

    println!("{:?}", init_red_particles)
    //     println!("{:?}", coordinates)
    // for (x,y) in coordinates {
    //     println!("{} {}", x,y);
    // }
    // }
}
