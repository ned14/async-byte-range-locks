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
    
    /// Returns the length of the map
    pub fn len(&self) -> usize { self.root.len() }
    
    /// Return if the map is empty
    pub fn is_empty(&self) -> bool { self.root.is_empty() }
    
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
    
    /// Returns an iterator
    pub fn iter(&self) -> Iter<K, V> {
      self.root.iter()
    }
    
    /// Returns an iterator over the keys
    pub fn keys<'a>(&'a self) -> Keys<'a, K, V> {
      Keys(self.iter().map(|i|{ i.key }))
    }
    
    /// Returns an iterator over the values
    pub fn values<'a>(&'a self) -> Values<'a, K, V> {
      Values(self.iter().map(|i|{ i.value }))
    }
  }
  
  pub struct Iter<'a, K:'a, V:'a> {
    iter: Vec<VectorMapItem<K, V>>
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
    assert_eq!(v.insert(10, "niall10"), None);
    assert_eq!(v.insert(40, "niall40"), None);
    assert_eq!(v.insert(20, "niall20"), None);
    assert_eq!(v.insert(30, "niall30"), None);
    assert_eq!(v.insert(0, "niall0"), None);
    assert_eq!(v.len(), 5);
    for i in v {
      println!("{}", i);
    }
    // Exact match
    assert!(matches!(v.get(&10).ok(), Some(x) if x.value == "niall0"));
    v.clear();
    assert!(v.is_empty());
  }
}
