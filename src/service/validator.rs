#[derive(Debug)]
pub struct Validator<T> {
  pub value: T,
  pub max_length: Option<i32>
}

impl Validator<String> {

  pub fn new(value: String) -> Self {
    Validator { value: value, max_length: None }
  }

  pub fn max_length(mut self, max_length: i32) -> Self {
    self.max_length = Some(max_length);
    self
  }

}