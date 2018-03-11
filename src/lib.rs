use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct Borrow<'a, K: 'a + Hash + Eq, V: 'a> {
	_original: &'a mut HashMap<K, V>,
	borrows: Vec<&'a mut V>,
}

impl<'a, K: 'a + Hash + Eq, V> Borrow<'a, K, V> {
	pub fn with_hashset(original: &'a mut HashMap<K, V>, keys: &HashSet<K>) -> Self {
		let mut borrows = Vec::with_capacity(keys.len());
		
		for key in keys {
			if let Some(value) = original.get_mut(key) {
				unsafe {
					borrows.push(&mut *(value as *mut V));
				}
			} else {
				println!("couldn't find entry");
			}
		}
		
		borrows.shrink_to_fit();
		
		Borrow {
			_original: original,
			borrows: borrows,
		}
	}
	
	pub fn with_hashmap<_T>(original: &'a mut HashMap<K, V>, keys: &HashMap<K, _T>) -> Self {
		let mut borrows = Vec::with_capacity(keys.len());
		
		for key in keys.keys() {
			if let Some(value) = original.get_mut(key) {
				unsafe {
					borrows.push(&mut *(value as *mut V));
				}
			} else {
				println!("couldn't find entry");
			}
		}
		
		borrows.shrink_to_fit();
		
		Borrow {
			_original: original,
			borrows: borrows,
		}
	}
	
	pub fn from_raw_parts(original: &'a mut HashMap<K, V>, borrows: Vec<&'a mut V>) -> Self {
		Borrow {
			_original: original,
			borrows: borrows,
		}
	}
	
	pub fn borrow_vec<'b: 'c, 'c>(&'b mut self) -> &'c mut Vec<&'a mut V> {
		&mut (self.borrows)
	}
	
	pub unsafe fn into_vec(self) -> Vec<&'a mut V> {
		self.borrows
	}
}