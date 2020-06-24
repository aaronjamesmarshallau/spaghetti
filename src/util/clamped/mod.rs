pub trait Clamped {
    fn clamp(&self, lower_bound: i64, upper_bound: i64) -> Self;
    fn clamp_lower(&self, lower_bound: i64) -> Self;
    fn clamp_upper(&self, upper_bound: i64) -> Self;
}

impl Clamped for i64 {
    fn clamp(&self, lower_bound: i64, upper_bound: i64) -> i64 {
        self.clamp_lower(lower_bound).clamp_upper(upper_bound)
    }

    fn clamp_lower(&self, lower_bound: i64) -> i64 {
        if *self < lower_bound {
            lower_bound
        } else {
            *self
        }
    }

    fn clamp_upper(&self, upper_bound: i64) -> i64 {
        if *self > upper_bound {
            upper_bound
        } else {
            *self
        }
    }
}
