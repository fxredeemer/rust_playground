enum WorkLocation {
    Jungle,
    NorthPole,
    OtherPlace(String),
    SecretPlace(i32, i32),
}

pub fn enums() {
    print_place_related_stuff(WorkLocation::Jungle);
    print_place_related_stuff(WorkLocation::NorthPole);
    print_place_related_stuff(WorkLocation::OtherPlace("Honolulu".to_owned()));
    print_place_related_stuff(WorkLocation::SecretPlace(425, 555));
}

fn print_place_related_stuff(location: WorkLocation) {
    match location {
        WorkLocation::Jungle => println!("Welcome to the jungle"),
        WorkLocation::NorthPole => println!("❄❄❄"),
        WorkLocation::OtherPlace(location) => println!("Working from {}", location),
        WorkLocation::SecretPlace(latitude, longitude) => {
            println!("I'll find you... Coordinates: {}|{}", latitude, longitude)
        }
    }
}
