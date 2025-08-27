use std::collections::{HashSet, HashMap};

pub fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
  let mut hash: HashMap<i64, Vec<&i64>> = HashMap::new();

  for i in l.iter() {
    for y in factorize(i) {
      let v: &mut Vec<&i64> = hash.entry(y).or_insert(Vec::new());
      v.push(i);
    }
  }

  let mut ans: Vec<(i64, i64)> = hash.iter()
    .map(|item | (*item.0, item.1.iter().fold(0 as i64,|acc, e| acc + **e)))
    .collect();
  ans.sort_by(|a, b| a.0.cmp(&b.0));

  ans
}

fn factorize(x: &i64) -> HashSet<i64> {
  let mut set = HashSet::new();
  let mut x = x.clone();

  for i in 2..((x as f64).abs().sqrt() as i64 + 1) {
    while x.abs() % i == 0 {
        set.insert(i);
        x = x.abs() / i as i64;
    }
  }

  if x.abs() != 1 {
    set.insert(x.abs());
  }

  set
}

#[cfg(test)]
mod tests {
  use super::*;

  fn testing(l: Vec<i64>, exp: Vec<(i64, i64)>) -> () {
    assert_eq!(sum_of_divided(l), exp)
  }

  #[test]
  fn basics_sum_of_divided() {
    testing(vec![12, 15], vec![ (2, 12), (3, 27), (5, 15) ]);
    testing(vec![15,21,24,30,45], vec![ (2, 54), (3, 135), (5, 90), (7, 21) ]);
  }
}