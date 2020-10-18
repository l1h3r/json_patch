use crate::error::Result;
use crate::operation::OpAdd;
use crate::operation::OpCopy;
use crate::operation::OpMove;
use crate::operation::OpRemove;
use crate::operation::OpReplace;
use crate::operation::OpTest;
use crate::operation::Operation;

pub trait CanPatch {
  fn patch_add(&mut self, op: OpAdd) -> Result<()>;
  fn patch_copy(&mut self, op: OpCopy) -> Result<()>;
  fn patch_move(&mut self, op: OpMove) -> Result<()>;
  fn patch_remove(&mut self, op: OpRemove) -> Result<()>;
  fn patch_replace(&mut self, op: OpReplace) -> Result<()>;
  fn patch_test(&mut self, op: OpTest) -> Result<()>;
}

impl<T> CanPatch for T
where
  T: FnMut(Operation) -> Result<()>,
{
  #[inline]
  fn patch_add(&mut self, op: OpAdd) -> Result<()> {
    (self)(Operation::Add(op))
  }

  #[inline]
  fn patch_copy(&mut self, op: OpCopy) -> Result<()> {
    (self)(Operation::Copy(op))
  }

  #[inline]
  fn patch_move(&mut self, op: OpMove) -> Result<()> {
    (self)(Operation::Move(op))
  }

  #[inline]
  fn patch_remove(&mut self, op: OpRemove) -> Result<()> {
    (self)(Operation::Remove(op))
  }

  #[inline]
  fn patch_replace(&mut self, op: OpReplace) -> Result<()> {
    (self)(Operation::Replace(op))
  }

  #[inline]
  fn patch_test(&mut self, op: OpTest) -> Result<()> {
    (self)(Operation::Test(op))
  }
}
