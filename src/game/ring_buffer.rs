extern crate queues;
use queues::IsQueue;

pub struct RingBuffer<T: Clone> {
	queue: Vec<T>,
	capacity: usize,
}

impl<T: Clone> RingBuffer<T> {
	pub fn new(capacity: usize) -> RingBuffer<T> {
		RingBuffer {
			queue: vec![],
			capacity,
		}
	}

	pub fn capacity(&self) -> usize {
		self.capacity
	}

	pub fn cloneAsVec(&self) -> Vec<T> {
		let mut cpy = self.queue.to_vec();
		cpy
	}

	impl<T: Clone> IsQueue<T> for RingBuffer<T> {
		fn add(&mut self, val: T) -> Result<Option<T>, &str> {
			if self.queue.len() < self.capacity {
				self.queue.push(val);
				Ok(None)
			} else {
				self.queue.push(val);
				Ok(Some(self.queue.remove(0usize)))
			}
		}

		fn remove(&mut self) -> Result<T, &str> {
			if self.queue.len() > 0 {
				if let Some(val) = self.default_value.clone() {
					self.queue.push(val);
				};
				Ok(self.queue.remove(0usize))
			} else {
				Err("The Buffer is empty!")
			}
		}

		fn peek(&self) -> Result<T, &str> {
			match self.queue.first() {
				Some(val) => Ok(val.clone()),
				None => Err("The Queue is empty!"),
			}
		}

		fn size(&self) -> usize {
			self.queue.len()
		}
	}
}
