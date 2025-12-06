use tracing::{debug, info, instrument};
use crate::day_1::rotation::{Rotation, RotationDirection};

pub enum PasswordMethod {
    Simple,
    Method0x434C49434B
}

pub struct Dial {
    position: i32,
    point_at_zero_counter: u32,
    traversed_zero_counter: u32
}

impl Default for Dial {
    fn default() -> Self {
        Self { position: 50, point_at_zero_counter: 0, traversed_zero_counter: 0}
    }
}

impl Dial {
    fn update_position(&mut self, rotation: &Rotation) -> bool {
        let wrapped_steps = rotation.steps % 100;
        match rotation.direction {
            RotationDirection::Left => self.position -= wrapped_steps as i32,
            RotationDirection::Right => self.position += wrapped_steps as i32,
        }
        let overflow = self.position < 0 || self.position > 100;
        self.position = self.position.rem_euclid(100);
        overflow
    }

    fn update_zero_reached_counter(&mut self) -> u32 {
        let reached_zero = self.position == 0;
        debug!(%reached_zero, "updating zero reached counter");
        if reached_zero {
            self.point_at_zero_counter += 1;
        }
        reached_zero as u32
    }
    fn update_zero_crossings_counter(&mut self, rotation: &Rotation, overflow: bool, last_position: i32) -> u32 {
        let traversed_zero = overflow && last_position != 0;
        let mut full_cycles = rotation.steps / 100;
        if full_cycles > 0 && last_position == 0 &&  self.position == 0 {
            // Special case: we were at zero, we did N full rotations ending up at zero again - we should not consider this as a crossing
            full_cycles -= 1;
        }
        if traversed_zero  {
            full_cycles += 1;
        }
        debug!(%traversed_zero, %full_cycles, %last_position, "updating zero crossings counter");
        self.traversed_zero_counter += full_cycles;
        full_cycles
    }

    #[instrument(skip(self))]
    pub fn rotate(&mut self, rotation: Rotation) {
        let last_position = self.position;
        let overflow = self.update_position(&rotation);
        let new_zero_reached_counts = self.update_zero_reached_counter();
        let new_zero_crossing_counts = self.update_zero_crossings_counter(&rotation, overflow, last_position);
        info!(%new_zero_reached_counts, %new_zero_crossing_counts, %last_position, %self.position, "Dial state after rotation");
    }

    pub fn get_password(&self, password_method: PasswordMethod) -> u32 {
        match password_method {
            PasswordMethod::Simple => self.point_at_zero_counter,
            PasswordMethod::Method0x434C49434B => self.traversed_zero_counter + self.point_at_zero_counter
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_simple_rotations() {
        let mut dial = Dial::default();
        dial.rotate(Rotation{ direction: RotationDirection::Left, steps: 3 });
        assert_eq!(dial.position, 47);
        dial.rotate(Rotation{ direction: RotationDirection::Right, steps: 4 });
        assert_eq!(dial.position, 51);
        // Ensure the counter is incremented correctly
        dial.rotate(Rotation{ direction: RotationDirection::Right, steps: 49 });
        assert_eq!(dial.point_at_zero_counter, 1);
        dial.rotate(Rotation{ direction: RotationDirection::Left, steps: 50 });
        dial.rotate(Rotation{ direction: RotationDirection::Left, steps: 50 });
        assert_eq!(dial.point_at_zero_counter, 2);
        // The password for the simple case should be equal to the number of times the dial has reached zero
        assert_eq!(dial.get_password(PasswordMethod::Simple), 2)
    }

    /// Ensure that the dial doesn't move when rotating by multiples of 100
    #[test]
    fn test_long_rotations() {
        let mut dial = Dial::default();
        dial.rotate(Rotation{ direction: RotationDirection::Left, steps: 300 });
        assert_eq!(dial.position, 50);
        assert_eq!(dial.get_password(PasswordMethod::Simple), 0);
        assert_eq!(dial.get_password(PasswordMethod::Method0x434C49434B), 3);
        dial.rotate(Rotation{ direction: RotationDirection::Right, steps: 500 });
        assert_eq!(dial.position, 50);
        assert_eq!(dial.get_password(PasswordMethod::Simple), 0);
        assert_eq!(dial.get_password(PasswordMethod::Method0x434C49434B), 8);
    }

    #[test]
    fn test_method_0x434c49434b() {
        let mut dial = Dial::default();
        dial.rotate(Rotation{ direction: RotationDirection::Right, steps: 60 });
        assert_eq!(dial.get_password(PasswordMethod::Method0x434C49434B), 1);

        let mut dial = Dial::default();
        dial.rotate(Rotation{ direction: RotationDirection::Left, steps: 50 });
        // We reached zero once, and we never crossed it
        assert_eq!(dial.get_password(PasswordMethod::Method0x434C49434B), 1);
        dial.rotate(Rotation{direction: RotationDirection::Right, steps: 30});
        // We did not cross zero, so the password should stay at 1
        assert_eq!(dial.get_password(PasswordMethod::Method0x434C49434B), 1);
        dial.rotate(Rotation{direction: RotationDirection::Right, steps: 80});
        // We crossed zero now, and we reached zero before, so the total should be 2
        assert_eq!(dial.get_password(PasswordMethod::Method0x434C49434B), 2);
        dial.rotate(Rotation{direction: RotationDirection::Left, steps: 300});
        // We crossed zero three times, so the counter should reflect that
        assert_eq!(dial.get_password(PasswordMethod::Method0x434C49434B), 5);

        let mut dial = Dial{ position: 0, ..Default::default()};
        // Special case: we're at zero, and we need to travel right by 100 steps - the counter should only increment once
        dial.rotate(Rotation{direction: RotationDirection::Right, steps: 100});
        assert_eq!(dial.get_password(PasswordMethod::Method0x434C49434B), 1);


    }

    #[test]
    fn test_method_0x434c49434b_full_cycle_reaching_zero() {
        // Special case: we're not at zero, but we perform N full rotations AND end up at zero again - both counters should increment accordingly
        let mut dial = Dial::default();
        dial.rotate(Rotation{direction: RotationDirection::Right, steps: 250});
        assert_eq!(dial.get_password(PasswordMethod::Method0x434C49434B), 3);

        let mut dial = Dial::default();
        dial.rotate(Rotation{direction: RotationDirection::Left, steps: 250});
        assert_eq!(dial.get_password(PasswordMethod::Method0x434C49434B), 3);
    }
}