#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Order {
    Unordered,
    Increasing,
    NotDecreasing,
    Decreasing,
    NotIncreasing,
    Constant,
}

fn sequence_classifier(arr: &[i32]) -> Order {
    if arr.windows(2).all(|w| w[0] == w[1]) {
        return Order::Constant;
    } else if arr.windows(2).all(|w| w[0] < w[1]) {
        return Order::Increasing;
    } else if arr.windows(2).all(|w| w[0] <= w[1]) {
        return Order::NotDecreasing;
    } else if arr.windows(2).all(|w| w[0] > w[1]) {
        return Order::Decreasing;
    } else if arr.windows(2).all(|w| w[0] >= w[1]) {
        return Order::NotIncreasing;
    } else {
        return Order::Unordered;
    }
}

#[cfg(test)]
mod tests {
    use super::{sequence_classifier, Order, Order::*};
    
    #[test]
    fn sample_tests() {
        let cases: [(&[i32], Order); 11] = [
            (&[3,5,8,1,14,3], Unordered),
            (&[3,5,8,9,14,23], Increasing),
            (&[3,5,8,8,14,14], NotDecreasing),
            (&[14,9,8,5,3,1], Decreasing),
            (&[14,14,8,8,5,3], NotIncreasing),
            (&[8,8,8,8,8,8], Constant),
            (&[8,9], Increasing),
            (&[8,8,8,8,8,9], NotDecreasing),
            (&[9,8], Decreasing),
            (&[9,9,9,8,8,8], NotIncreasing),
            (&[3,5,8,1,14,2], Unordered),
        ];
        for (seq, expected) in cases {
            let actual = sequence_classifier(seq);
            assert_eq!(actual, expected, "\nYour result (left) did not match the expected output (right) for the sequence:\n{seq:?}");
        }
    }
}