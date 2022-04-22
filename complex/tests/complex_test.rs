use complex::Complex;

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