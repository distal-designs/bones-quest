mod ring_buffer;


use ggez::event::Keycode;
use queues::IsQueue;
use std::collections::HashSet;

use self::ring_buffer::RingBuffer;


const HISTORY_CAPACITY: usize = 10;


pub struct Input {
    pub current_input: HashSet<Keycode>,
    input_history: RingBuffer<HashSet<Keycode>>,
}


impl Input {
    pub fn new() -> Self {
        Self {
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


    pub fn get_input_history(&self) -> &Vec<HashSet<Keycode>> {
        &self.input_history.get_queue()
    }


    pub fn finalize(&mut self) {
        let inputs = self.current_input.clone();
        self.enqueue(inputs);
    }


    pub fn pressed(&self) -> HashSet<Keycode> {
        let history = self.get_input_history();
        match history.as_slice() {
            [] => HashSet::new(),
            [h] => h.clone(),
            all => {
                let len = all.len();
                let current = &all[len - 1];
                let previous = &all[len - 2];
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
