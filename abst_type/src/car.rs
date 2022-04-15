
pub trait Vehicle {
    fn get_price(&self) -> u64;
}

pub trait Car: Vehicle {
    fn model(&self) -> String;
}

pub struct TeslaRoadster {
    pub model: String,
    pub release_date: u16
}

impl TeslaRoadster {
    pub fn new(model: &str, release_date: u16) -> Self {
        Self { model: model.to_string(), release_date }
    }
}

impl Car for TeslaRoadster {
    fn model(&self) -> String {
        "Tesla Roadster I".to_string()
    }
}

impl Vehicle for TeslaRoadster {
    fn get_price(&self) -> u64 {
        270_000 // 创建可读的数字文本
    }
}