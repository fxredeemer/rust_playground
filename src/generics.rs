use std::fmt::Display;

struct Data<T> {
    value: T,
    index: i32,
}

pub fn generics() {
    let value = Data {
        value: "32",
        index: 1,
    };

    use_generic(value);
}

fn use_generic<T>(data: Data<T>)
where
    T: Display,
{
    print!("index {}", &data.index);
    print!("index {}", &data.value);
}
