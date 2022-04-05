#[derive(Debug)]
struct Dummy;

#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Player {
    id: u8,
    name: String,
    age: u8,
    sex: i8,
}

// 为结构体绑定方法
impl Player {
    fn with_name(name: &str) -> Player {
        Player {
            name: name.to_string(),
            id: 1,
            age: 20,
            sex: -1
        }
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn incr_age(&mut self, val: u8) {
        self.age += val
    }
}

fn main() {
    println!("空结构体:");
    let value = Dummy;
    println!("{:?}", value);

    println!();
    println!("元组结构体:");
    let color = Color(85, 123, 204);

    let red = color.0;
    let green = color.1;
    let blue = color.2;

    println!("red: {}, green: {}, blue: {}", red, green, blue);
    println!("color: {:?}", color);

    // 元组类型解包
    let Color(r, g, b) = color;
    println!("r: {}, g: {}, b: {}", r, g, b);

    println!();
    println!("实体结构:");
    let mut player = Player{id:1, name:"whl".to_string(), age:28, sex:-1};
    player.age += 1;
    println!("player user: {:?}", player);
    println!("name: {}", player.name);
}
