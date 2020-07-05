use std::cmp::PartialOrd;

pub trait Clamped<T> {
    fn clamp(self, lower_bound: T, upper_bound: T) -> Self;
    fn clamp_lower(self, lower_bound: T) -> Self;
    fn clamp_upper(self, upper_bound: T) -> Self;
}

impl<T: PartialOrd> Clamped<T> for T {
    fn clamp(self, lower_bound: T, upper_bound: T) -> T {
        self.clamp_lower(lower_bound).clamp_upper(upper_bound)
    }

    fn clamp_lower(self, lower_bound: T) -> T {
        if self < lower_bound {
            lower_bound
        } else {
            self
        }
    }

    fn clamp_upper(self, upper_bound: T) -> T {
        if self > upper_bound {
            upper_bound
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp() {
        assert_eq!(Clamped::clamp(5, 4, 6), 5, "Value between bounds remains unchanged.");
        assert_eq!(Clamped::clamp(3, 4, 6), 4, "Value below lower bound returns lower bound.");
        assert_eq!(Clamped::clamp(7, 4, 6), 6, "Value above upper bound returns upper bound.");
    }

    #[test]
    fn test_clamp_lower() {
        assert_eq!(Clamped::clamp_lower(5, 4), 5, "Value above lower bound remains unchanged.");
        assert_eq!(Clamped::clamp_lower(3, 4), 4, "Value below lower bound returns lower bound.");
    }

    #[test]
    fn test_clamp_upper() {
        assert_eq!(Clamped::clamp_upper(5, 6), 5, "Value below upper bound remains unchanged.");
        assert_eq!(Clamped::clamp_upper(7, 6), 6, "Value above upper bound returns upper bound.");
    }
}
