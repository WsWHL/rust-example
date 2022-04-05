fn main() {
    println!("不带条件循环:");
    let mut count = 1;
    loop {
        if count > 10 {
            break;
        }
        println!("loop count: {}", count);
        count+=1;
    }

    println!();
    println!("带条件循环:");
    let mut number = 3;
    while number > 0 {
        println!("number: {}", number);
        number-=1;
    }

    println!();
    println!("不包含上限:");
    println!("Normal ranges:");
    for i in 1..10 {
        println!("i: {}", i);
    }

    println!();
    println!("包含上限:");
    println!("Inclusive ranges: ");
    for i in 1..=10 {
        println!("i: {}", i);
    }
}
