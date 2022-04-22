use std::ops::Add;
use std::convert::From;
use std::fmt::{{Formatter, Display, Result}, write};

#[derive(Default, Debug, PartialEq, Copy, Clone)]
struct Complex<T> {
    // 实部
    re: T,
    // 虚部
    im: T
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Self { // Self为此类型的别名
        Complex{ re, im }
    }
}

/*
    - 范型T必须实现Add<T, Output=T>
    - "<T, Output=T>"表示Add特征的实现必须具有相同的输入和输出类型
 */
impl<T: Add<T, Output=T>> Add for Complex<T> {
    type Output = Complex<T>;

    // 使用"+"运算时将调用该方法
    fn add(self, rhs: Complex<T>) -> Self::Output {
        Complex{ re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

impl<T> From<(T, T)> for Complex<T> {
    fn from(value: (T, T)) -> Complex<T> {
        Complex{ re: value.0, im: value.1 }
    }
}

impl<T: Display> Display for Complex<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "re: {}, im: {}", self.re, self.im)
    }
}

#[cfg(test)]
mod tests {
    use crate::Complex;

    #[test]
    fn complex_basic() {
        let first = Complex::new(3, 5);
        let second: Complex<i32> = Complex::default();
        assert_eq!(first.re, 3);
        assert_eq!(first.im, 5);
        assert_eq!(second.re, second.im);
    }

    #[test]
    fn complex_add() {
        let a = Complex::new(1, -2);
        let b = Complex::<i32>::default();
        let res = a + b;
        assert_eq!(res, a);
    }

    #[test]
    fn complex_from() {
        let a = (2345, 567);
        let complex = Complex::from(a);
        assert_eq!(complex.re, 2345);
        assert_eq!(complex.im, 567);
    }

    #[test]
    fn complex_display() {
        let complex = Complex::new(2345, 567);
        println!("{}", complex);
    }
}