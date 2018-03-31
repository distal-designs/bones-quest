use ggez::event::Keycode;
use std::collections::HashSet;

use input_queue::InputQueue;
use ring_buffer::RingBuffer;

const HISTORY_CAPACITY: usize = 10;

pub struct Input {
    current_input: HashSet<Keycode>,
    input_history: RingBuffer<Input>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            self.current_input = HashSet::new();
            self.input_history = RingBuffer::new(HISTORY_CAPACITY);
        }
    }

    pub fn add_input(&mut self, keycode: Keycode, frame_index: usize) {
        self.current_input.insert(keycode); 
    }

    pub fn remove_input(&mut self, keycode: Keycode, frame_index: usize) {
        self.current_input.remove(keycode);
    }

    pub fn get_input_history(&mut self) -> Vec<Input> {
        self.processed_input.cloneAsVec();
    }

    pub fn finalize(&mut self) {
        enqueue(self.current_input.clone());
    }

    fn enqueue(&mut self, input: &Input) {
        self.input_history.add(input);
    }
}
