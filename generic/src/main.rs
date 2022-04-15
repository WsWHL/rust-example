

// 范型方法
fn give_me<T>(value: T) {
    let _ = value;
}

// 范型结构体
#[derive(Debug)]
struct GenericStruct<T>(T);

#[derive(Debug)]
struct Container<T> {
    item: T
}

impl<T> Container<T> {
    fn new(item: T) -> Self {
        Container{item}
    }
}

impl Container<u32> {
    fn sum(item: u32) -> Self {
        Container{item}
    }
}

// 范型枚举
#[derive(Debug)]
enum Transmission<T> {
    Signal(T),
    NoSignal
}

fn main() {
    let name = "generics";
    let number = 100;
    give_me(name);
    give_me(number);

    let a = GenericStruct(1);
    print!("a: {:?}\n", a);

    let tran = Transmission::Signal("hello");
    println!("tran: {:?}", tran);
    let tran_a = Transmission::Signal(100);
    println!("tran_a: {:?}", tran_a);

    let container = Container::new("hello");
    print!("item: {}\n", container.item);
    assert_eq!(container.item, "hello");
    let c_num = Container::new(99);
    print!("item: {}\n", c_num.item);

    let c_sum = Container::sum(80);
    print!("item: {}\n", c_sum.item);

    // 范型列表
    let v1: Vec<u8> = Vec::new();
    let mut v2 = Vec::new();
    v2.push("test");
    let v3 = Vec::<u8>::new();
    print!("v1: {:?}, v2: {:?}, v3: {:?}\n", v1, v2, v3);

    // 字符转换
    let num = str::parse::<u8>("34").unwrap();
    print!("Parsed number: {}", num);
}
