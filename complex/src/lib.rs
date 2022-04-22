use std::ops::Add;
use std::convert::From;
use std::fmt::{{Formatter, Display, Result}};

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct Complex<T> {
    // 实部
    pub re: T,
    // 虚部
    pub im: T
}

impl<T> Complex<T> {
    ///
    ///
    /// # Arguments
    ///
    /// * `re`:
    /// * `im`:
    ///
    /// returns: Complex<T>
    ///
    /// # Examples
    ///
    /// ```
    /// use complex::Complex;
    ///
    /// let a = Complex::new(123, 25);
    /// let b = Complex::default();
    /// let res = a + b;
    /// assert_eq!(res.re, 123);
    /// assert_eq!(res.im, 25);
    /// ```
    pub fn new(re: T, im: T) -> Self { // Self为此类型的别名
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