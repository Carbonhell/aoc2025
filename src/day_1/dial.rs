use tracing::{debug, info, instrument};
use crate::day_1::rotation::{Rotation, RotationDirection};

pub struct Dial {
    position: i32,
    pub point_at_zero_counter: u32
}

impl Default for Dial {
    fn default() -> Self {
        Self { position: 50, point_at_zero_counter: 0 }
    }
}

impl Dial {
    #[instrument(skip(self))]
    pub fn rotate(&mut self, rotation: Rotation) {
        debug!(%rotation, "Rotating dial");
        // Constrain the range of steps to [0:100] to make the cast to i32 safe
        let wrapped_steps = rotation.steps % 100;
        match rotation.direction {
            RotationDirection::Left => self.position -= wrapped_steps as i32,
            RotationDirection::Right => self.position += wrapped_steps as i32,
        }
        self.position = self.position.rem_euclid(100);
        info!(%self.position, "New position");
        if self.position == 0 {
            info!("Dial reached zero position");
            self.point_at_zero_counter += 1;
        }
    }
}
