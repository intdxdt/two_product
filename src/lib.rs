const SPLITTER: f64 = 134217729.0;//math.Pow(2, 27) + 1.0

///Computes the product of two floating point numbers as a
/// 2-term nonoverlapping increasing sequence
pub fn two_product(a: f64, b: f64) -> (f64, f64) {
    let x = a * b;
    let c = SPLITTER * a;
    let abig = c - a;
    let ahi = c - abig;
    let alo = a - ahi;

    let d = SPLITTER * b;
    let bbig = d - b;
    let bhi = d - bbig;
    let blo = b - bhi;

    let err1 = x - (ahi * bhi);
    let err2 = err1 - (alo * bhi);
    let err3 = err2 - (ahi * blo);

    let y = alo * blo - err3;

    (y, x)
}


#[cfg(test)]
mod two_prod_test {
    use super::two_product;

    extern crate rand;

    use std::f64;

    fn gen() -> f64 {
        rand::random::<f64>()
    }

    #[test]
    fn test_two_product() {
        let mut test_values = vec![
            0.0, 1.0, 2f64.powi(-52), 2f64.powi(-53),
            1.0 - 2f64.powi(-53), 1.0 + 2f64.powi(-52),
            2f64.powi(-500), 2f64.powi(500),
            2.0, 0.5, 3.0, 1.5, 0.1, 0.3, 0.7,
        ];
        for _ in 0..20 {
            test_values.push(gen());
            test_values.push(gen() * (2f64.powf(1000.0 * gen() - 500.0)));
        }
        let n = test_values.len();
        for i in (1..n).rev() {
            let x = test_values[i];
            test_values.push(-x)
        }
        assert!(
            two_product(1.0 + 2f64.powi(-52), 1.0 + 2f64.powi(-52)) ==
                (2f64.powi(-104), 1.0 + 2f64.powi(-51))
        );
        for j in 0..test_values.len() {
            let a = test_values[j];
            assert!(a * a < f64::INFINITY);
            assert!(two_product(0f64, a)  == (0.0, 0.0));
            assert!(two_product(1f64, a)  == (0.0,   a));
            assert!(two_product(-1f64, a) == (0.0,  -a));
        }
    }
}
