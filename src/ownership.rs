struct MyDataType {
    data: i32,
}

pub fn ownership() {
    let value = MyDataType { data: 12 };
    take_ownership(value);
    //take_ownership(value);

    let value = MyDataType { data: 12 };

    take_reference(&value);
    take_reference(&value);
}

fn take_ownership(data: MyDataType) {
    print!("My Data {}", data.data);
}

fn take_reference(data: &MyDataType) {
    print!("My Data {}", data.data);
}
