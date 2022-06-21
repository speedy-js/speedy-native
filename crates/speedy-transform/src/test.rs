#[cfg(test)]
pub mod tests {

  #[test]
  fn env() {
    for (key, val) in std::env::vars() {
      println!("{:?} is {:?} ....", key, val);
    }
  }
}
