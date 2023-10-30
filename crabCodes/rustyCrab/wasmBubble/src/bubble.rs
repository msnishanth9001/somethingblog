use wasm_bindgen::prelude::*;
use js_sys::Math;
use web_sys::console;

#[wasm_bindgen]
pub fn sort_array() -> Vec<u32> {
    let mut random_numbers = generate_randoms(100, 0.0, 1000.0);
    let random_arrangement = random_numbers.clone();

    console::log_1(&format!("{:?}", random_arrangement).into());

    bubble_sort(&mut random_numbers);

    let _is_sorted = is_sorted(&random_numbers);
    random_numbers
}

fn generate_randoms(count: usize, min: f64, max: f64) -> Vec<u32> {
    let mut random_numbers = Vec::with_capacity(count);
    for _ in 0..count {
        let random_number = (min + (max - min) * Math::random()) as u32;
        random_numbers.push(random_number);
    }
    random_numbers
}

fn bubble_sort(arr: &mut Vec<u32>) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn is_sorted(arr: &Vec<u32>) -> bool {
    for i in 0..arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            return false;
        }
    }
    true
}
