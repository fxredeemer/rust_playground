use std::vec;

pub fn collections() {
    let data = vec![1, 2, 3, 4];
    for value in &data {
        println!("Value: {}", value);
    }

    let filtered_values = data.iter().filter(|v| *v % 2 == 0);
    for value in filtered_values {
        println!("Filtered value: {}", value);
    }

    let mapped_values = data.iter().map(|v| v * 2);
    for value in mapped_values {
        println!("Mapped value: {}", value);
    }

    println!("Is any values 4? {}", data.iter().any(|v| *v == 4));
}
