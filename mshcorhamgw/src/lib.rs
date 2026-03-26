#![cfg_attr(target_arch = "xtensa", no_std)]

pub fn next_state(counter: u32) -> bool {
    counter.is_multiple_of(2)
}

#[cfg(test)]
mod tests {
    use super::next_state;

    #[test]
    fn even_counter_is_on() {
        assert!(next_state(0));
        assert!(next_state(2));
    }

    #[test]
    fn odd_counter_is_off() {
        assert!(!next_state(1));
        assert!(!next_state(3));
    }
}
