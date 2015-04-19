/// Module implementing a map as a sorted vector as BTreeMap isn't up to needs
mod vector_map {
  use std::mem;
  
  #[derive(Clone)]
  struct VectorMapItem<K, V> {
    pub key: K,
    pub value: V,
  }
  
  /// A map implemented as a sorted vector
  #[derive(Clone)]
  pub struct VectorMap<K, V> {
    root: Vec<VectorMapItem<K, V>>
  }
  
  impl<K: Ord, V> VectorMap<K, V> {
    pub fn new() -> VectorMap<K, V> {
      VectorMap { root : Vec::<VectorMapItem<K, V>>::new() }
    }
    
    /// Find the nearest key matching
    fn binary_search(&self, key : &K) -> Result<usize, usize> {
      let s = &self.root[..];
      s.binary_search_by(|x| x.key.cmp(key))
    }
    
    /// Clears the map, removing all values
    pub fn clear(&mut self) {
      self.root.clear();
    }
    
    // FIXME TODO Allow K to be not a reference
    /// Returns a reference to an exact match (Ok) or nearest match (Err) for the specified key
    pub fn get(&self, key: &K) -> Result<&VectorMapItem<K, V>, &VectorMapItem<K, V>> {
      match self.binary_search(key) {
        Err(index) => Err(&self.root[index]),
        Ok(index) => Ok(&self.root[index]),
      }
    }
    
    /// Inserts a value into the map, returning any exact match formerly there if any
    pub fn insert(&mut self, key : K, value : V) -> Option<V> {
      match self.binary_search(&key) {
        // Insert value
        Err(index) => {
          self.root.insert(index, VectorMapItem { key : key, value : value});
          None
        },
        // Replace value
        Ok(index) => {
          let mut v = value;
          {
            let x = &mut v;
            let y = &mut self.root[index].value;
            mem::swap(x, y);
          }
          Some(v)
        },
      }
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
  fn insert_works() {
    use super::vector_map::VectorMap;
    let mut v = VectorMap::new();
    assert_eq!(v.insert(1, "niall1"), None);
    assert_eq!(v.insert(4, "niall4"), None);
    assert_eq!(v.insert(2, "niall2"), None);
    assert_eq!(v.insert(3, "niall3"), None);
    assert_eq!(v.insert(0, "niall0"), None);
    assert!(matches!(v.get(&1).ok(), Some(x) if x.value == "niall1"));
  }
}
