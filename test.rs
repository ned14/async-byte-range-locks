// A Rust implementation of byte range locks

/// Module implementing file byte range locks
mod file_byte_range_locks {
  use std::collections::btree_map::BTreeMap;
  use std::default::Default;

  /// A file descriptor
  pub type file_descriptor = i32;

  /// A start offset (inclusive) and end offset (exclusive)
  type byte_range = (u64, u64);

  /// A variant holding the type of lock for a range
  enum lock {
    /// This range is share locked by one or more fds
    shared(Vec<file_descriptor>),
    /// This range is exclusively locked by just this fd
    exclusive(file_descriptor),
  }
  
  /// The range locks associated with some file
  pub struct locks {
    /// The file name
    name: String,
    /// A sorted map of ranges to the file descriptors which hold them
    locked_regions: BTreeMap<byte_range, lock>,
  }

  /// Possible unlock errors
  pub enum unset_lock_error { not_found }

  impl locks {
    fn new(name: String) -> locks {
      locks { name : name, locked_regions : () }
    }
  
    /// Sets a lock for a given byte range, returning false if not possible
    fn set_lock(&mut self, fd: file_descriptor, range: byte_range, exclusive: bool) -> Result<bool, ()> {
      Ok(true)
    }

    /// Unsets a lock for a given byte range
    fn unset_lock(&mut self, fd: file_descriptor, range: byte_range) -> Result<(), unset_lock_error> {
      Ok(())
    }
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn two_write_locks_exclude() {
    use super::file_byte_range_locks::{locks, unset_lock_error};
    let mut f = locks::new("foo");
    assert!(f.set_lock(1, (0, 2), true) == Result::Ok(true));
    assert!(f.set_lock(2, (0, 1), true) == Result::Ok(false));
    assert!(f.set_lock(2, (1, 2), true) == Result::Ok(false));
    // After range locked by fd 1, so must succeed
    assert!(f.set_lock(2, (2, 3), true) == Result::Ok(true));
    // Replace range locked by fd 1
    assert!(f.set_lock(1, (0, 1), true) == Result::Ok(true));
    // Now extend range locked by fd 2
    assert!(f.set_lock(2, (0, 2), true) == Result::Ok(false));
    // Now extend range locked by fd 2
    assert!(f.set_lock(2, (1, 2), true) == Result::Ok(true));
    // Check unlocking an unlocked range is an error
    assert!(f.unset_lock(1, (1, 2)) == Result::Err(unset_lock_error::not_found));
    // Unlock a range
    assert!(f.unset_lock(1, (0, 9999)) == Result::Ok(()));
    assert!(f.unset_lock(2, (0, 9999)) == Result::Ok(()));
    // Check unlocking a range twice is an error
    assert!(f.unset_lock(1, (0, 9999)) == Result::Err(unset_lock_error::not_found));
    assert!(f.unset_lock(2, (0, 9999)) == Result::Err(unset_lock_error::not_found));
  }
}
