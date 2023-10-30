use rand::Rng;

fn main() {
    let mut random_numbers = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        let random_number: u32 = rng.gen();
        random_numbers.push(random_number);
    }

    bubble_sort(&mut random_numbers);

    if is_sorted(&random_numbers) {
        println!("Array is sorted.");
    } else {
        println!("Array is not sorted.");
    }
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