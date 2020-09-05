pub struct HAStore {}

impl HAStore {
  fn get() -> String {
    return String::from("Got data");
  }
}

#[cfg(test)]
mod tests {
  use crate::HAStore;

  #[test]
  fn it_works() {
    assert_eq!(HAStore::get(), "Got data");
  }
}
