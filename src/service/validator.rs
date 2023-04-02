#[derive(Debug)]
pub struct Validator<T> {
  pub value: T,
  pub is_valid: bool,
  pub validation_errors: Vec<String>,
  pub max_length: Option<usize>,
  pub max_length_error: Option<String>,
  pub min_length: Option<usize>,
  pub min_length_error: Option<String>,
  pub allow_lowercase_letters: bool,
  pub allow_uppercase_letters: bool,
  pub allow_numbers: bool,
  pub allowed_symbols: Option<String>,
  pub whitelist_error: Option<String>,
}

impl Validator<String> {
  pub fn new(value: String) -> Self {
    Validator {
      value: value,
      is_valid: true,
      validation_errors: Vec::new(),
      max_length: None,
      max_length_error: None,
      min_length: None,
      min_length_error: None,
      allow_lowercase_letters: true,
      allow_uppercase_letters: true,
      allow_numbers: true,
      allowed_symbols: None,
      whitelist_error: None,
    }
  }

  pub fn run_validation(mut self) -> Self {
    if self.max_length != None {
      if self.value.len() > self.max_length.unwrap() {
        self.is_valid = false;
        self
          .validation_errors
          .push(self.max_length_error.clone().unwrap());
      }
    }

    if self.min_length != None {
      if self.value.len() < self.min_length.unwrap() {
        self.is_valid = false;
        self
          .validation_errors
          .push(self.min_length_error.clone().unwrap());
      }
    }

    if self.allowed_symbols != None {
      for value_char in self.value.chars() {
        if value_char.is_ascii_uppercase() {
          if !self.allow_uppercase_letters {
            self.is_valid = false;
            self
              .validation_errors
              .push(self.whitelist_error.clone().unwrap());
            break;
          }
          continue;
        }
        if value_char.is_ascii_lowercase() {
          if !self.allow_lowercase_letters {
            self.is_valid = false;
            self
              .validation_errors
              .push(self.whitelist_error.clone().unwrap());
            break;
          }
          continue;
        }
        if value_char.is_ascii_digit() {
          if !self.allow_numbers {
            self.is_valid = false;
            self
              .validation_errors
              .push(self.whitelist_error.clone().unwrap());
            break;
          }
          continue;
        }
        let mut found_match: bool = false;
        for symbol_char in self.allowed_symbols.clone().unwrap().chars() {
          if value_char == symbol_char {
            found_match = true;
          }
        }
        if !found_match {
          self.is_valid = false;
          self
            .validation_errors
            .push(self.whitelist_error.clone().unwrap());
          break;
        }
      }
    }

    self
  }

  pub fn max_length(mut self, max_length: usize, error_message: String) -> Self {
    self.max_length = Some(max_length);
    self.max_length_error = Some(error_message);
    self
  }

  pub fn min_length(mut self, min_length: usize, error_message: String) -> Self {
    self.min_length = Some(min_length);
    self.min_length_error = Some(error_message);
    self
  }

  pub fn whitelist(mut self, allowed_symbols: String, error_message: String) -> Self {
    self.allowed_symbols = Some(allowed_symbols);
    self.whitelist_error = Some(error_message);
    self
  }

  pub fn get_error_message(self) -> String {
    self.validation_errors.last().unwrap().to_string()
  }
}
