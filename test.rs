// A Rust implementation of byte range locks

/// Module implementing a map as a sorted vector as BTreeMap isn't up to needs
mod vector_map {
  #[derive(Clone)]
  struct VectorMapItem<K, V> {
    key: K,
    value: V,
  }
  
  /// A map implemented as a sorted vector
  #[derive(Clone)]
  pub struct VectorMap<K, V> {
    root: Vec<VectorMapItem<K, V>>
  }
  
  impl<K: Ord, V> VectorMap<K, V> {
    pub fn new() -> VectorMap<K, V> {
      VectorMap<K, V> { root : Vec::<VectorMapItem<K, V>>::new() }
    }
    
    /// Find the nearest key matching
    fn binary_search(&self, key : K) -> Result<usize, usize> {
      let s = &self.root[..];
      s.binary_search_by(|x| x.key.cmp(&key));
    }
    
    /// Clears the map, removing all values
    pub fn clear(&mut self) {
      self.root.clear();
    }
    
    /// Inserts a value into the map, returning any exact match formerly there if any
    pub fn insert(&mut self, key : K, value : V) -> Option<V> {
      match self.binary_search(key) {
        Err(index) =>,
        Ok(index) => let oldvalue = self.root[index].value; self.root[index].value = value; oldvalue,
      }
    }
  }
}

/// Module implementing file byte range locks
mod file_byte_range_locks {
  use std::collections::btree_map::BTreeMap;

  /// A file descriptor
  pub type FileDescriptor = i32;

  /// A start offset (inclusive) and end offset (exclusive)
  type ByteRange = (u64, u64);

  /// A variant holding the type of lock for a range
  enum Lock {
    /// This range is share locked by one or more fds
    Shared(Vec<FileDescriptor>),
    /// This range is exclusively locked by just this fd
    Exclusive(FileDescriptor),
  }
  
  /// The range locks associated with some file
  pub struct Locks {
    /// The file name
    name: String,
    /// A sorted map of ranges to the file descriptors which hold them
    locked_regions: BTreeMap<ByteRange, Lock>,
  }

  /// Possible unlock errors
  #[derive(Debug)]
  pub enum UnsetLockError { NotFound }

  impl Locks {
    pub fn new(name: String) -> Locks {
      Locks { name : name, locked_regions : BTreeMap::<ByteRange, Lock>::new() }
    }
  
    /// Sets a lock for a given byte range, returning false if not possible
    pub fn set_lock(&mut self, fd: FileDescriptor, range: ByteRange, exclusive: bool) -> Result<bool, ()> {
      Ok(true)
    }

    /// Unsets a lock for a given byte range
    pub fn unset_lock(&mut self, fd: FileDescriptor, range: ByteRange) -> Result<(), UnsetLockError> {
      Ok(())
    }
  }
}

#[cfg(test)]
mod test {
  // Quite breathtakingly, Rust 1.0 doesn't allow comparison of enum values :(
  // Taken from https://github.com/SimonSapin/rust-std-candidates/blob/master/matches/lib.rs
  #[macro_export]
  macro_rules! matches {
      ($expression: expr, $($pattern:tt)+) => {
          _tt_as_expr_hack! {
              match $expression {
                  $($pattern)+ => true,
                  _ => false
              }
          }
      }
  }

  /// Work around "error: unexpected token: `an interpolated tt`", whatever that means.
  #[macro_export]
  macro_rules! _tt_as_expr_hack {
      ($value:expr) => ($value)
  }

  #[test]
  fn two_write_locks_exclude() {
    use super::file_byte_range_locks::{Locks, UnsetLockError};
    let mut f = Locks::new("foo".to_string());
    assert_eq!(f.set_lock(1, (0, 2), true).ok(), Some(true));
    assert_eq!(f.set_lock(2, (0, 1), true).ok(), Some(false));
    assert_eq!(f.set_lock(2, (1, 2), true).ok(), Some(false));
    // After range locked by fd 1, so must succeed
    assert_eq!(f.set_lock(2, (2, 3), true).ok(), Some(true));
    // Replace range locked by fd 1
    assert_eq!(f.set_lock(1, (0, 1), true).ok(), Some(true));
    // Now extend range locked by fd 2
    assert_eq!(f.set_lock(2, (0, 2), true).ok(), Some(false));
    // Now extend range locked by fd 2
    assert_eq!(f.set_lock(2, (1, 2), true).ok(), Some(true));
    // Check unlocking an unlocked range is an error
    assert!(matches!(f.unset_lock(1, (1, 2)).err(), Some(UnsetLockError::NotFound)));
    // Unlock a range
    assert_eq!(f.unset_lock(1, (0, 9999)).ok(), Some(()));
    assert_eq!(f.unset_lock(2, (0, 9999)).ok(), Some(()));
    // Check unlocking a range twice is an error
    assert!(matches!(f.unset_lock(1, (0, 9999)).err(), Some(UnsetLockError::NotFound)));
    assert!(matches!(f.unset_lock(2, (0, 9999)).err(), Some(UnsetLockError::NotFound)));
  }
}
