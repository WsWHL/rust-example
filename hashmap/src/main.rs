use std::collections::HashMap;

fn main() {
    println!("哈希字典:");
    let mut brands: HashMap<&str, u32> = HashMap::new();
    brands.insert("apple", 8888);
    brands.insert("xiaomi", 6666);
    brands.insert("huawei", 7777);

    // 获取指定键的值
    let amount = brands["apple"];
    println!("apple amount: {}", amount);

    // 判断指定键名是否存在
    if brands.contains_key("huawei") {
       println!("exists huawei brand.")
    }

    // 删除指定键
    brands.remove("huawei");

    // 遍历字典
    for (key, value) in brands {
        println!("key: {}, value: {}", key, value);
    }
}
