use tracing::{debug, info};

pub struct Bank {
    batteries: Vec<u32>,
}

impl From<&str> for Bank {
    fn from(value: &str) -> Self {
        let bank = Bank {
            batteries: value.chars().map(|x| x.to_digit(10).unwrap()).collect(),
        };
        if bank.batteries.len() < 2 {
            panic!("Unexpected bank of less than 2 batteries, can't proceed with a solution");
        }
        bank
    }
}

impl Bank {
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
    pub fn joltage(&self) -> u32 {
        let first_battery = self.battery_with_max_joltage(None, Some(self.batteries.len() - 1));
        let second_battery = self.battery_with_max_joltage(Some(first_battery), None);
        info!(?first_battery, ?second_battery, "found batteries");
        (self.batteries[first_battery] * 10) + self.batteries[second_battery]
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
}
