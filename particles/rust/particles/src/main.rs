use itertools::iproduct;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

use flo_canvas::*;
use flo_draw::*;

const WIDTH: u16 = 2560;
const HEIGHT: u16 = 1440;
const TIME_SCALE: f32 = 1.0;
const VELOCITY: f32 = 0.7;
const ITERATION_DISTANCE: u16 = 3500;

#[derive(Debug, Clone)]
struct ParticleInfo {
    id: u16,
    color: u16,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

fn get_colors() -> HashMap<u16, (f32, f32, f32)> {
    HashMap::from([
        (1, (0.0, 0.0, 1.0)), // red
        (2, (1.0, 0.0, 0.0)), // grean
        (3, (0.0, 1.0, 0.0)), // blue
        (4, (1.0, 0.7, 0.0)), // yellow
    ])
}

fn generate_random_rule(color_count: u16, rule_range: (f32, f32)) -> HashMap<(u16, u16), f32> {
    let mut rules = HashMap::new();
    let colour_pairs = iproduct!(1..color_count + 1, 1..color_count + 1);
    for pair in colour_pairs {
        rules.insert(
            pair,
            rand::thread_rng().gen_range(rule_range.0..rule_range.1 + 1.0),
        );
    }
    rules
}

fn apply_rules(
    rules: &HashMap<(u16, u16), f32>,
    particles_sub_list: Vec<ParticleInfo>,
    all_particles: &Vec<ParticleInfo>,
) -> Vec<ParticleInfo> {
    let mut result: Vec<ParticleInfo> = Vec::new();

    for particle1 in particles_sub_list {
        let mut fx: f32 = 0.0;
        let mut fy: f32 = 0.0;

        for particle2 in all_particles {
            if particle1.id != particle2.id {
                let g = rules[&(particle1.color, particle2.color)];
                let dx = particle1.x - particle2.x;
                let dy = particle1.y - particle2.y;
                if dx != 0.0 || dy != 0.0 {
                    let distance = dx * dx + dy * dy;
                    if distance < ITERATION_DISTANCE as f32 {
                        let F = g / (distance as f32).sqrt();
                        fx += F * dx as f32;
                        fy += F * dy as f32;
                    }
                }
            }
        }
        let vmix = 1.0 - VELOCITY;
        let mut vx = particle1.vx * vmix + fx * TIME_SCALE;
        let mut vy = particle1.vy * vmix + fy * TIME_SCALE;
        let mut x = particle1.x + particle1.vx;
        let mut y = particle1.y + particle1.vy;

        if x < 0.0 || x >= WIDTH as f32 {
            vx *= -1.0;
            x = if x < 0.0 { 0.0 } else { WIDTH as f32 - 1.0 }
        }

        if y < 0.0 || y >= HEIGHT as f32 {
            vy *= -1.0;
            y = if y < 0.0 { 0.0 } else { HEIGHT as f32 - 1.0 }
        }
        result.push(ParticleInfo {
            id: particle1.id,
            color: particle1.color,
            x,
            y,
            vx,
            vy,
        });
    }
    result
}

fn generate_init_particles(
    count: u16,
    color_count: u16,
    coordinates: Vec<(u16, u16)>,
) -> Vec<ParticleInfo> {
    let count_per_color = count / color_count;
    let mut j = 0;
    let mut init_particles: Vec<ParticleInfo> = Vec::new();
    let mut color = 1;

    let coordinates_sample: Vec<(u16, u16)> = coordinates
        .choose_multiple(&mut rand::thread_rng(), count as usize)
        .cloned()
        .collect::<Vec<(u16, u16)>>();

    for i in 0..count {
        let temp_tuple = coordinates_sample[i as usize];
        init_particles.push(ParticleInfo {
            id: i,
            color: color,
            x: temp_tuple.0 as f32,
            y: temp_tuple.1 as f32,
            vx: 0.0,
            vy: 0.0,
        });

        j += 1;
        if j >= count_per_color {
            color += 1;
            j = 0;
        }
    }
    init_particles
}

fn main() {
    with_2d_graphics(|| {
        let canvas = create_drawing_window("Particles");
        let color_count: u16 = 4;

        let X: Vec<u16> = (0..WIDTH).collect();
        let Y: Vec<u16> = (0..HEIGHT).collect();
        let coordinates: Vec<(u16, u16)> = iproduct!(X, Y).collect();

        let mut particles = generate_init_particles(3200, color_count, coordinates);
        let rules = generate_random_rule(color_count, (-2.0, 2.0));

        for _ in 0.. {
            canvas.draw(|gc| {
                gc.clear_canvas(Color::Rgba(0.0, 0.0, 0.0, 1.0));
                gc.canvas_height(HEIGHT as f32);
                gc.center_region(0.0, 0.0, WIDTH as f32, HEIGHT as f32);

                particles = apply_rules(&rules, particles.clone(), &particles);
                for particle in &particles {
                    gc.new_path();
                    gc.circle(particle.x, particle.y, 2.0);

                    let color = get_colors()[&particle.color];
                    gc.fill_color(Color::Rgba(color.0, color.1, color.2, 1.0));
                    gc.fill();
                    gc.stroke();
                }
            });
        }
    });
}
