use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug)]
pub struct Borrow<'a, K: 'a + Hash + Eq, V: 'a> {
	_original: &'a mut HashMap<K, V>,
	borrows: Vec<&'a mut V>,
}

impl<'a, K: 'a + Eq + Hash, V> Borrow<'a, K, V> {
	pub fn with_hashset(original: &'a mut HashMap<K, V>, keys: &HashSet<K>) -> Self {
		let mut borrows = Vec::with_capacity(keys.len());
		
		for k in keys {
			if let Some(value) = original.get_mut(k) {
				unsafe { borrows.push(&mut *(value as *mut V)) }
			}
		}
		
		borrows.shrink_to_fit();
		
		Self {
			_original: original,
			borrows,
		}
	}
	
	pub fn with_hashmap<_T>(original: &'a mut HashMap<K, V>, keys: &HashMap<K, _T>) -> Self {
		let mut borrows = Vec::with_capacity(keys.len());
		
		for k in keys.keys() {
			if let Some(value) = original.get_mut(k) {
				unsafe { borrows.push(&mut *(value as *mut V)) }
			}
		}
		
		borrows.shrink_to_fit();
		
		Self {
			_original: original,
			borrows,
		}
	}
	
	pub fn with_deduped_vec(original: &'a mut HashMap<K, V>, keys: &mut Vec<K>) -> Self
	where K: Ord
	{
		keys.sort_unstable();
		keys.dedup();
		unsafe { Self::with_slice_unchecked(original, keys) }
	}
	
	pub fn with_slice(original: &'a mut HashMap<K, V>, keys: &[K]) -> Self {
		if keys.is_empty() {
			Self {
				_original: original,
				borrows: Vec::new(),
			}
		} else {
			for i in 0..(keys.len()-1) {
				for j in (i+1)..keys.len() {
					if keys[i] == keys[j] {
						panic!("tried to mutably borrow multiple keys from same hashmap");
					}
				}
			}
			unsafe { Self::with_slice_unchecked(original, keys) }
		}
	}
	
	pub unsafe fn with_slice_unchecked(original: &'a mut HashMap<K, V>, keys: &[K]) -> Self {
		let mut borrows = Vec::with_capacity(keys.len());
		
		for k in keys.iter() {
			if let Some(value) = original.get_mut(k) {
				borrows.push(&mut *(value as *mut V));
			}
		}
		
		borrows.shrink_to_fit();
		
		Self {
			_original: original,
			borrows,
		}
	}
	
	pub fn from_raw_parts(original: &'a mut HashMap<K, V>, borrows: Vec<&'a mut V>) -> Self {
		Self {
			_original: original,
			borrows,
		}
	}
	
	pub unsafe fn into_vec(self) -> Vec<* mut V> {
		std::mem::transmute(self.borrows)
	}
}

impl<'a, K: 'a + Eq + Hash, V: 'a> std::ops::Deref for Borrow<'a, K, V> {
	type Target = Vec<&'a mut V>;
	fn deref(&self) -> &Self::Target {
		&self.borrows
	}
}

impl<'a, K: 'a + Eq + Hash, V: 'a> std::ops::DerefMut for Borrow<'a, K, V> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.borrows
	}
}
