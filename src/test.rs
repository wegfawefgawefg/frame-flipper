use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub struct AnimationMachine<T: Eq + Hash + Clone + Debug> {
    pub animations: HashMap<T, RandomAccessAnimation>,
    pub current_animation: T,
    // ... (other fields)
}

impl<T: Eq + Hash + Clone + Debug> AnimationMachine<T> {
    pub fn new(default_animation: T, default_frame_duration: u8) -> AnimationMachine<T> {
        AnimationMachine {
            animations: HashMap::new(),
            current_animation: default_animation,
            // ... (rest of the initialization)
        }
    }

    pub fn add_animation(&mut self, state: T, animation: RandomAccessAnimation) {
        self.animations.insert(state, animation);
    }

    pub fn set_animation(&mut self, state: T) {
        if self.animations.contains_key(&state) {
            self.current_animation = state.clone();
            // ... (reset animation frame and other state variables)
        } else {
            // Handle error: unknown animation state
        }
    }

    pub fn get_current_animation(&self) -> &RandomAccessAnimation {
        &self.animations[&self.current_animation]
    }

    // ... (other methods)
}
