use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct WordCounter(HashMap<String, u64>);

impl WordCounter {
    fn new() -> WordCounter {
        WordCounter(HashMap::new())
    }

    fn incrment(&mut self, word: &str) {
        let key = word.to_string();
        // 获取指定键值信息，不存在则设置默认值，借用值并递增
        let count = self.0.entry(key).or_insert(0);
        *count += 1;
    }

    fn display(self) {
        // 循环迭代哈希字典
        for (key, value) in self.0.iter() {
            println!("{}: {}", key, value);
        }
    }
}

fn main() {
    println!("单词计数器:");
    let mut arguments: Vec<String> = env::args().collect();
    let filename = &mut arguments[1];
    println!("Processing file: {}", filename);
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut word_counter = WordCounter::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let words = line.split(" ");
        for word in words {
            if word == "" {
                continue;
            } else {
                word_counter.incrment(word);
            }
        }
    }

    word_counter.display();
}
