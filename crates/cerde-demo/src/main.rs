use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized: {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized: {:?}", deserialized);

    let data = json!({
        "id": 1,
        "name": "test",
        "age": 30,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });
    println!(
        "id: {}, name: {}, phone1: {}, phone2: {}",
        data["id"], data["name"], data["phones"][0], data["phones"][1]
    );

    println!("{}", data.to_string());
}
