fn int32_to_ip(int: u32) -> String {
  let a = (int >> 24) & 0xFF;
  let b = (int >> 16) & 0xFF;
  let c = (int >> 8) & 0xFF;
  let d = int & 0xFF;
  format!("{}.{}.{}.{}", a, b, c, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(int32_to_ip(2154959208), "128.114.17.104");
        assert_eq!(int32_to_ip(2149583361), "128.32.10.1");
        assert_eq!(int32_to_ip(0), "0.0.0.0");
    }
}