#[derive(Debug)]
enum ColorType {
    RED = 1,
    GREED = 2,
    BLUE = 3
}

#[derive(Debug)]
enum PaymentType {
    AliPay,
    WxPay,
    ApplePay,
    Paypal
}

impl PaymentType {
    fn pay(&self, amount: u64) {
        match self {
            PaymentType::ApplePay => println!("Processing apple payment of {}", amount),
            PaymentType::WxPay => println!("Processing wx payment of {}", amount),
            PaymentType::AliPay => println!("Processing alipay payment of {}", amount),
            PaymentType::Paypal => println!("Processing paypal payment of {}", amount),
            other => println!("other payment of {:?}", other)
        }
    }
}

fn main() {
    println!("普通枚举:");
    let red = ColorType::RED;
    let green = ColorType::GREED;
    let blue = ColorType::BLUE;

    println!("red: {:?}, green: {:?}, blue: {:?}", red, green, blue);

    let color = ColorType::RED;
    match color {
        ColorType::RED => println!("this is red."),
        ColorType::GREED => println!("this is green."),
        ColorType::BLUE => println!("this is blue."),
    }

    println!();
    println!("带方法枚举:");
    let apple = PaymentType::ApplePay;
    apple.pay(100);

    let wx = PaymentType::WxPay;
    wx.pay(200);

    let alipay = PaymentType::AliPay;
    alipay.pay(99);

    let paypal = PaymentType::Paypal;
    paypal.pay(480);
}
