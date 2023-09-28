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
 * It stores the current animation, the current frame, and the countdown timer.
 * The countdown timer is used to determine when to move to the next frame.
 *
*/

struct AnimationMachine {
    pub animations: Vec<RandomAccessAnimation>,
    pub current_animation_index: usize,
    pub current_animation_frame: usize,
    pub countdown_timer: u8,
    pub frame_duration: u8,
    pub default_frame_duration: u8,
}

impl AnimationMachine {
    pub fn new(
        animations: Vec<RandomAccessAnimation>,
        default_frame_duration: u8,
    ) -> AnimationMachine {
        AnimationMachine {
            animations: animations,
            current_animation_index: 0,
            current_animation_frame: 0,
            countdown_timer: default_frame_duration,
            frame_duration: default_frame_duration,
            default_frame_duration: default_frame_duration,
        }
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

    pub fn set_animation(&mut self, index: usize) {
        self.current_animation_index = index;
        self.current_animation_frame = 0;
        self.countdown_timer = 0;
        self.frame_duration = 0;
    }

    pub fn get_current_animation(&self) -> &RandomAccessAnimation {
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

fn main() {
    // Create animations
    let anim1 = RandomAccessAnimation {
        positions: vec![(0, 0), (1, 1), (2, 2)],
    };
    let anim2 = RandomAccessAnimation {
        positions: vec![(10, 10), (11, 11), (12, 12)],
    };

    let mut anim_machine = AnimationMachine::new(vec![anim1, anim2], 5);

    // Simulate Animation Steps
    for i in 0..20 {
        println!(
            "Step {}: Current Frame {:?}, Current Position {:?}, Countdown Timer {}",
            i,
            anim_machine.current_animation_frame,
            anim_machine.get_current_frame_position(),
            anim_machine.countdown_timer
        );

        // Step AnimationMachine
        anim_machine.set_speed(1.0);
        anim_machine.reset_speed();
        anim_machine.step();

        // Change animation at step 10
        if i == 9 {
            anim_machine.set_animation(1);
            anim_machine.frame_duration = 3;
            anim_machine.countdown_timer = 3;
        }
    }
}
