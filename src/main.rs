use std::io::prelude::*;

const HIGHT: usize = 150;
const WIDTH: usize = 150;


struct Perceptron {
    inputs: Vec<Vec<f64>>,
    weights: Vec<Vec<f64>>,
}

impl Perceptron {
    fn new(inputs: Vec<Vec<f64>>, weights: Vec<Vec<f64>>) -> Perceptron {
        Perceptron { inputs, weights }
    }

    fn predict(&self) -> f64 {
        let mut output: f64 = 0.0;
        for input in &self.inputs {
            for i in 0..input.len() {
                output += input[i] * self.weights[0][i];
            }
        }
        output
    }
}

fn fill_rect(input: &mut Vec<Vec<f64>>, x: usize, y: usize, w: usize, h: usize) {

    let mut x1 = x as i32 - w as i32;
    let mut y1 = y as i32 - h as i32;
    let mut x2 = x as i32 + w as i32;
    let mut y2 = y as i32 + h as i32;

    if x1 < 0 { x1 = 0; }
    if y1 < 0 { y1 = 0; }
    if x2 > WIDTH as i32 { x2 = WIDTH as i32; }
    if y2 > HIGHT as i32 { y2 = HIGHT as i32; }
    
    for i in x1..x2 {
        for j in y1..y2 {
            input[i as usize][j as usize] = 1.0;
        }
    }
}

fn fill_circle(input: &mut Vec<Vec<f64>>, x: usize, y: usize, r: usize) {
    let mut x1 = x as i32 - r as i32;
    let mut y1 = y as i32 - r as i32;
    let mut x2 = x as i32 + r as i32;
    let mut y2 = y as i32 + r as i32;

    if x1 < 0 { x1 = 0; }
    if y1 < 0 { y1 = 0; }
    if x2 > WIDTH as i32 { x2 = WIDTH as i32; }
    if y2 > HIGHT as i32 { y2 = HIGHT as i32; }
    
    for i in x1..x2 {
        for j in y1..y2 {
            let distance = ((i - x as i32).pow(2) + (j - y as i32).pow(2)) as f64;
            if distance <= r.pow(2) as f64 {
                input[i as usize][j as usize] = 1.0;
            }
        }
    }
}

fn save_as_ppm(input: &Vec<Vec<f64>>, filename: &str) {
    let mut file = std::fs::File::create(filename).unwrap();
    let header = format!("P3\n{} {}\n255\n", WIDTH, HIGHT);
    file.write_all(header.as_bytes()).unwrap();
    for i in 0..HIGHT {
        for j in 0..WIDTH {
            let color = if input[i][j] == 1.0 { "255 0 0" } else { "255 255 255" };
            let line = format!("{}\n", color);
            file.write_all(line.as_bytes()).unwrap();
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
    let mut input: Vec<Vec<f64>>;

    input = vec![vec![0.0; WIDTH]; HIGHT];
    fill_circle(&mut input, 50, 50, 20);
    save_as_ppm(&input, "circle.ppm");

}
