use std::default::Default;

pub use transform::*;

/// An entity is anything that has spatial properties (i.e. a `Transform`).
#[derive(Debug)]
pub struct Entity<T> {
  pub object: T,
  pub transform: Transform
}

impl<T> Entity<T> {
  pub fn new(object: T, transform: Transform) -> Self {
    Entity {
      object: object,
      transform: transform
    }
  }
}

impl<T> Default for Entity<T> where T: Default {
  fn default() -> Self {
    Self::new(T::default(), Transform::default())
  }
}
