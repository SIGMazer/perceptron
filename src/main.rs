use core::f64;
use std::io::prelude::*;
use rand::Rng;
use std::{vec, usize};
use std::fs::File;


const HEIGHT: usize = 20;
const WIDTH: usize = 20;
const BIAS: f64 = 100.0;
const SAMPLE_RATE: usize = 100;
const PPM_SCALER: usize= 25;
const PPM_COLOR_INTENSITY: f64 = 255.0;
const PPM_RANGE: f64 = 10.0;





fn predict(inputs: &Vec<Vec<f64>>, weights:&Vec<Vec<f64>>) -> f64 {
    let mut output: f64 = 0.0;
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            output += inputs[i][j] * weights[i][j];
        }
    }
    output
}

fn add_inputs_from_weights(inputs: &[Vec<f64>], weights:&mut[Vec<f64>]) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            weights[i][j] += inputs[i][j];
        }
    }
}
fn sub_inputs_from_weights(inputs: &Vec<Vec<f64>>, weights:& mut Vec<Vec<f64>>) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            weights[i][j] -= inputs[i][j];
        }
    }
}

static mut COUNT : i32 = 0;

fn train(inputs: &mut Vec<Vec<f64>>, weights:&mut Vec<Vec<f64>>) -> usize {
    let mut output;
    let mut adjust = 0;
    for _ in 0..SAMPLE_RATE {
        fill_random_circle(inputs);
        output = predict(inputs, weights);
        unsafe{
            if output < BIAS {
                add_inputs_from_weights(inputs, weights);
                save_as_ppm(weights, &format!("data/train_{:03}.ppm", COUNT));
                COUNT += 1;
                adjust += 1;
            }
            fill_random_rect(inputs);
            output = predict(inputs, weights);
            if output > BIAS{
                sub_inputs_from_weights(inputs, weights);
                save_as_ppm(&weights, &format!("data/train_{:03}.ppm", COUNT));
                COUNT += 1;
                adjust += 1;
            }
        }
    }
    adjust
}
    

fn clamp(value: usize, min: usize, max: usize) -> usize {
    if value < min { return min; }
    if value > max { return max; }
    value
}

fn fill_rect(input: &mut Vec<Vec<f64>>, x: usize, y: usize, w: usize, h: usize) {

    let x1 = clamp(x, 0, WIDTH);
    let y1 = clamp(y, 0, HEIGHT);
    let x2 = clamp(x + w, 0, WIDTH);
    let y2 = clamp(y + h, 0, HEIGHT);

    for i in x1..x2 {
        for j in y1..y2 {
            input[i as usize][j as usize] = 1.0;
        }
    }
}

fn rnad_range(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

fn fill_random_rect(input: &mut Vec<Vec<f64>>) {

    let x = rnad_range(0, WIDTH as i32) as usize;
    let y = rnad_range(0, HEIGHT as i32) as usize;
    let mut w = WIDTH - x;
    if w < 10 {
        w = 10;
    }
    w = rnad_range(0, w as i32) as usize;

    let mut h = HEIGHT - y;
    if h < 10 {
        h = 10;
    }
    h = rnad_range(0, h as i32) as usize;

    fill_rect(input, x, y, w, h);
}

fn fill_circle(input: &mut Vec<Vec<f64>>, x: usize, y: usize, r: usize) {
    let x1 = clamp(x.saturating_sub(r)  , 0, WIDTH -1);
    let y1 = clamp(y.saturating_sub(r)  , 0, HEIGHT -1);
    let x2 = clamp(x.saturating_add(r+1), 0, WIDTH -1);
    let y2 = clamp(y.saturating_add(r+1), 0, HEIGHT -1);


    for i in x1..x2 {
        for j in y1..y2 {
            let dx = i as i32 - x as i32;
            let dy = j as i32 - y as i32;
            let d = (dx * dx + dy * dy) as f64;
            if d <= (r * r) as f64 {
                input[i as usize][j as usize] = 1.0;
            }
        }
    }
}

fn fill_random_circle(input: &mut Vec<Vec<f64>>) {
    let cx = rnad_range(0, WIDTH as i32) as usize;
    let cy = rnad_range(0, HEIGHT as i32) as usize;
    let mut r = usize::MAX;
    if r > cx{
        r = cx;
    }
    if r > cy{
        r = cy;
    }

    if r > WIDTH - cx{
        r = WIDTH - cx;
    }
    if r > HEIGHT - cy{
        r = HEIGHT - cy;
    }
    if r < 10{
        r = 10;
    }

    r = rnad_range(0, r as i32) as usize;
    fill_circle(input, cx, cy, r);
}

fn save_as_ppm(layer: &Vec<Vec<f64>>, file_path: &str) {
    let mut f = match File::create(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("ERROR: could not open file {}: {}", file_path, e);
            std::process::exit(1);
        }
    };

    write!(f, "P6\n{} {} 255\n", WIDTH * PPM_SCALER, HEIGHT * PPM_SCALER).unwrap();

    for y in 0..HEIGHT * PPM_SCALER {
        for x in 0..WIDTH * PPM_SCALER {
            let s = (layer[y / PPM_SCALER][x / PPM_SCALER] + PPM_RANGE) / (2.0 * PPM_RANGE);
            let pixel = [
                (PPM_COLOR_INTENSITY * (1.0 - s)) as u8,
                (PPM_COLOR_INTENSITY * s) as u8,
                (PPM_COLOR_INTENSITY * (1.0 - s)) as u8,
            ];

            f.write_all(&pixel).unwrap();
        }
    }
}
fn save_as_bin(input: &Vec<Vec<f64>>, filename: &str) {
    let mut file = std::fs::File::create(filename).unwrap();
    for row in input {
        for value in row {
            let bytes = value.to_ne_bytes();
            file.write_all(&bytes).unwrap();
        }
    }
}

fn main() {

    let mut input = vec![vec![0.0; WIDTH]; HEIGHT];

    let mut weights = vec![vec![0.0; WIDTH]; HEIGHT];


    let mut count = 0;
    let mut adjust ;
    for _ in 0..SAMPLE_RATE{
        adjust = train(&mut input, &mut weights);
        save_as_ppm(&weights, "output.ppm");
        count += 1;
        println!("{}: adjust = {}", count, adjust);
        if adjust == 0 {
            break;
        }
    }
}

