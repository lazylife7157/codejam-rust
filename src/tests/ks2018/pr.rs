use super::super::*;

const YEAR: &str = super::YEAR;
const ROUND: &str = PR;

#[test]
fn test_a() {
    test(KS, YEAR, ROUND, A);
}

#[test]
fn test_b() {
    test(KS, YEAR, ROUND, B);
}

#[test]
fn test_c() {
    test(KS, YEAR, ROUND, C);
}

#[test]
fn test_d() {
    test(KS, YEAR, ROUND, D);
}
