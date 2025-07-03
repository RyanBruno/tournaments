
pub type EntityId = String;

pub trait Patch<T> {
  fn apply_to(self, target: &mut T);
}