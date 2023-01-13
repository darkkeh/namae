use js_sys::Math::*;
use wasm_bindgen::prelude::*;


use crate::animals::ANIMALS;
use crate::adjectives::ADEJCTIVES;

const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[wasm_bindgen]
pub fn generate_name() -> String {
    let idx = floor(random() * CHARS.len() as f64) as usize;
    let char = CHARS[idx] as char;

    let adjectives = fill_vec(char, ADEJCTIVES);
    let animals = fill_vec(char, ANIMALS);

    let idx = floor(random() * animals.len() as f64) as usize;
    let animal = animals[idx].to_string();

    let idx = floor(random() * adjectives.len() as f64) as usize;
    let adjective = adjectives[idx].to_string();

    let name = format!("{} {}", adjective, animal);
    name
}

pub fn fill_vec(char: char, data: &[&'static str]) -> Vec<String> {
    let mut vec = vec![];
    for item in data {
        if item.chars().next().unwrap() == char {
            vec.push(item.to_string());
        }
    }
    vec
}

