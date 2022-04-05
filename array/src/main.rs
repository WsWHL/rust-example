fn main() {
    println!("数组:");
    let numbers: [i8; 5] = [9, 10, 25, -4, 99];
    let floats = [2.444, 888.99, 10.33];

    println!("number: {}", numbers[3]);
    println!("float: {}", floats[2]);

    println!("元组:");
    let num_and_str: (u8, &str) = (20, "this is string");
    println!("{:?}", num_and_str);
    let (num, string) = num_and_str;
    println!("num: {}, string: {}", num, string);

    println!("列表:");
    let mut numbers_vec: Vec<u8> = Vec::new();
    numbers_vec.push(10);
    numbers_vec.push(50);
    numbers_vec.push(99);
    println!("numbers: {:?}", numbers_vec);

    let mut vec_with_macro = vec![1, 2, 5]; // 自动推断列表中元素类型
    vec_with_macro.push(255);
    vec_with_macro.push(128);
    let _ = vec_with_macro.pop(); // 忽略空格

    let message = if numbers_vec == vec_with_macro {
        "They are equal"
    } else {
        "Nah! They look different to me"
    };

    println!("{} {:?} {:?}", message, numbers_vec, vec_with_macro);

    println!("列表切片:");
    let mut names: [&str; 5] = ["hello", "test", "tomcat", "lisi", "hi"];
    {
        let all = &names[..];
        println!("all of them: {:?}", all);
    }

    {
        let first_two: &mut [&str] = &mut names[0..2];
        first_two[0] = "first";
        first_two[1] = "two";
    }

    println!("names: {:?}", names);
}