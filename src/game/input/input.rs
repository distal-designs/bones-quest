use ggez::event::Keycode;
use queues::IsQueue;
use std::collections::HashSet;

use super::ring_buffer::RingBuffer;

const HISTORY_CAPACITY: usize = 10;

pub struct Input {
    current_input: HashSet<Keycode>,
    input_history: RingBuffer<HashSet<Keycode>>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            current_input: HashSet::new(),
            input_history: RingBuffer::new(HISTORY_CAPACITY),
        }
    }

    pub fn add_input(&mut self, keycode: Keycode) {
        self.current_input.insert(keycode);
    }

    pub fn remove_input(&mut self, keycode: Keycode) {
        self.current_input.remove(&keycode);
    }

    pub fn _get_input_history(&self) -> &Vec<HashSet<Keycode>> {
        &self.input_history._get_queue()
    }

    pub fn finalize(&mut self) {
        let inputs = self.current_input.clone();
        self.enqueue(inputs);
    }

    pub fn _pressed(&self) -> HashSet<Keycode> {
        let history = self._get_input_history();
        match history.len() {
            0 => HashSet::new(),
            1 => history.last().unwrap().clone(),
            len => {
                let current = &history[len - 1];
                let previous = &history[len - 2];
                current.difference(&previous).cloned().collect()
            }
        }
    }

    fn enqueue(&mut self, inputs: HashSet<Keycode>) {
        if let Err(e) = self.input_history.add(inputs) {
            panic!(format!("Could not enqueue inputs: {}", e));
        }
    }
}
