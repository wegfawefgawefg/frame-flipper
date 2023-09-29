use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/** RandomAccessAnimation:
 * This animation format stores the position of each frame.
 * This allows for tight packing of the frames in memory.
 * This comes at the cost of having to store the position of each frame.
 */
struct RandomAccessAnimation {
    pub positions: Vec<(i32, i32)>,
}

/** AnimationMachine:
 * This struct is used to manage the animation.
 * It takes in an enum it associates with the current animations.
*/
pub struct AnimationMachine<T: Eq + Hash + Clone + Debug> {
    animation_mapping: HashMap<T, usize>,
    animations: Vec<RandomAccessAnimation>,
    current_animation: Option<T>,
    current_animation_index: usize,
    current_animation_frame: usize,
    countdown_timer: u8,
    frame_duration: u8,
    default_frame_duration: u8,
}

impl<T: Eq + Hash + Clone + Debug> AnimationMachine<T> {
    pub fn new(default_frame_duration: u8) -> AnimationMachine<T> {
        AnimationMachine {
            animation_mapping: HashMap::new(),
            animations: Vec::new(),
            current_animation: Option::None,
            current_animation_index: 0,
            current_animation_frame: 0,
            countdown_timer: default_frame_duration,
            frame_duration: default_frame_duration,
            default_frame_duration: default_frame_duration,
        }
    }

    pub fn add_animation(&mut self, state: T, animation: RandomAccessAnimation) {
        self.animation_mapping
            .insert(state.clone(), self.animations.len());
        self.animations.push(animation);
    }

    pub fn reset_speed(&mut self) {
        self.frame_duration = self.default_frame_duration;
    }

    pub fn set_speed(&mut self, speed: f32) {
        if speed > 0.0 {
            let calculated_duration = (self.default_frame_duration as f32 / speed).round() as u8;
            self.frame_duration = calculated_duration.max(1);
        }
    }

    pub fn set_animation(&mut self, state: T) {
        if self.animations.contains_key(&state) {
            self.current_animation = state.clone();
            self.current_animation_frame = 0;
            self.countdown_timer = self.frame_duration;
        } else {
            panic!(
                "Unknown animation state: {:?} \n Did you forget to add it?",
                state
            );
        }
    }

    fn get_current_animation(&self) -> &RandomAccessAnimation {
        &self.animations[self.current_animation_index]
    }

    pub fn get_current_frame_position(&self) -> (i32, i32) {
        self.get_current_animation().positions[self.current_animation_frame]
    }

    pub fn step(&mut self) {
        self.countdown_timer -= 1;
        if self.countdown_timer == 0 {
            self.current_animation_frame += 1;
            if self.current_animation_frame >= self.get_current_animation().positions.len() {
                self.current_animation_frame = 0;
            }
            self.countdown_timer = self.frame_duration;
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum MyAnimationState {
    Run,
    Hide,
    Jump,
    Shoot,
}

fn main() {
    let mut machine = AnimationMachine::new(MyAnimationState::Run, 5);
    machine.add_animation(
        MyAnimationState::Run,
        RandomAccessAnimation {
            positions: vec![(0, 0), (1, 1)],
        },
    );
    machine.add_animation(
        MyAnimationState::Jump,
        RandomAccessAnimation {
            positions: vec![(2, 2), (3, 3)],
        },
    );
    machine.set_animation(MyAnimationState::Run);
}
