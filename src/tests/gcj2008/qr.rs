use super::super::*;

const YEAR: &str = super::YEAR;
const ROUND: &str = QR;

#[test]
fn test_a() {
    test(GCJ, YEAR, ROUND, A);
}

#[test]
fn test_b() {
    test(GCJ, YEAR, ROUND, B);
}

#[test]
fn test_c() {
    test(GCJ, YEAR, ROUND, C);
}
