use tracing::{debug, info, instrument};

pub struct Bank {
    batteries: Vec<u64>,
    max_enabled_batteries: u32,
}

impl Default for Bank {
    fn default() -> Self {
        Self {
            batteries: vec![],
            max_enabled_batteries: 2,
        }
    }
}

impl From<(&str, u32)> for Bank {
    fn from(value: (&str, u32)) -> Self {
        let bank = Bank {
            batteries: value
                .0
                .chars()
                .map(|x| x.to_digit(10).unwrap() as u64)
                .collect(),
            max_enabled_batteries: value.1,
        };
        if bank.batteries.len() < value.1 as usize {
            panic!(
                "Unexpected bank of less than {} batteries, can't proceed with a solution",
                value.1
            );
        }
        bank
    }
}

impl From<&str> for Bank {
    fn from(value: &str) -> Self {
        Bank::from((value, 2))
    }
}

impl Bank {
    pub fn set_max_enabled_batteries(&mut self, max_enabled_batteries: u32) {
        self.max_enabled_batteries = max_enabled_batteries;
    }
    fn battery_with_max_joltage(&self, from: Option<usize>, to: Option<usize>) -> usize {
        debug!(?from, ?to, "searching for battery with max joltage");
        let from = match from {
            Some(x) => x + 1,
            None => 0,
        };
        let to = match to {
            Some(x) => x,
            None => self.batteries.len(),
        };
        // We can't use max() as it'd return the last maximum element in case of >1 maximum
        let battery = self.batteries[from..to]
            .iter()
            .enumerate()
            .reduce(|acc, x| if x.1 > acc.1 { x } else { acc })
            .unwrap() // The reduce operation is a customized max(), and a bank is guaranteed to have at least 2 batteries at construction time
            .0;
        battery + from
    }

    #[instrument(skip(self))]
    pub fn joltage(&self) -> u64 {
        let mut last_battery_id = None;
        let batteries = (0..self.max_enabled_batteries)
            .rev()
            .map(|x| {
                let battery = self.battery_with_max_joltage(
                    last_battery_id,
                    Some(self.batteries.len() - x as usize),
                );
                last_battery_id = Some(battery);
                battery
            })
            .collect::<Vec<_>>();
        let joltage = (0..self.max_enabled_batteries)
            .rev()
            .zip(batteries.iter())
            .map(|(x, battery)| (10_u64.pow(x)) * self.batteries[*battery])
            .sum::<u64>();

        info!(?batteries, %joltage, "found batteries");
        joltage
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_parse_bank() {
        let bank = Bank::from("123456789");
        assert_eq!(bank.batteries, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_simple_joltage() {
        let bank = Bank::from("123456789");
        assert_eq!(bank.joltage(), 89);
    }

    #[test]
    fn test_example() {
        let bank = Bank::from("987654321111111");
        assert_eq!(bank.joltage(), 98);
        let bank = Bank::from("811111111111119");
        assert_eq!(bank.joltage(), 89);
        let bank = Bank::from("234234234234278");
        assert_eq!(bank.joltage(), 78);
        let bank = Bank::from("818181911112111");
        assert_eq!(bank.joltage(), 92);
    }

    #[test]
    fn test_with_12_batteries() {
        let bank = Bank::from(("987654321111111", 12));
        assert_eq!(bank.joltage(), 987654321111);
        let bank = Bank::from(("811111111111119", 12));
        assert_eq!(bank.joltage(), 811111111119);
        let bank = Bank::from(("234234234234278", 12));
        assert_eq!(bank.joltage(), 434234234278);
        let bank = Bank::from(("818181911112111", 12));
        assert_eq!(bank.joltage(), 888911112111);
    }
}
