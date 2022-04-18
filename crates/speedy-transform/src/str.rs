pub trait StringExtend {
  fn compare_handle(&self) -> std::string::String;
}

impl StringExtend for String {
  fn compare_handle(&self) -> std::string::String {
    let mut new_str = self.replace(' ', "");
    new_str = new_str.trim().replace('\n', "");
    new_str
  }
}
