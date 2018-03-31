extern crate queues;
use queues::Queue;

use input::Input;
use ring_buffer::RingBuffer;

const QUEUE_CAPACITY: usize = 5;
const HISTORY_CAPACITY: usize = 10;

pub trait InputHistoryQueue {
	fn enqueue(&mut self, input: &Input);
	fn dequeue(&mut self) -> Result<Input, &str>;
	fn getInputHistory(&mut self) -> Vec<Input>;
}

pub struct InputQueue {
	recent_input: Queue<Input>,
	processed_input: RingBuffer<Input>
}

impl InputQueue {
	pub fn new() -> InputQueue {
		InputQueue {
			self.recent_input = Queue::new();
			self.processed_input = RingBuffer::new(HISTORY_CAPACITY);
		}
	}

	pub fn enqueue(&mut self, input: &Input) {
		if self.recent_input.size() >= QUEUE_CAPACITY {
			return;
		}
		self.recent_input.add(input);
	}

	pub fn dequeue(&mut self) -> Result<Input, &str> {
		let deq_input = match self.recent_input.remove() {
			Err(e) => return Err(e),
			Ok(in) => in,
		}

		self.processed_input.add(deq_input);
		Ok(deq_input);
	}

	pub fn getInputHistory(&mut self) -> Vec<Input> {
		self.processed_input.cloneAsVec()
	}
}
