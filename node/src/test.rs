mod tests {
  use speedy_macro::speedydebug;

  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }

  #[speedydebug]
  fn debugmacro(num: i32, txt: &str) -> i32 {
    println!("123,{},{}", num, txt);
    num
  }

  #[test]
  fn test() {
    std::env::set_var("rsdebug", "info");
    let c = debugmacro(1, "test6666");
    print!("{}", c);
  }
}
