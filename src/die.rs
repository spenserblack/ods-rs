use std::ops::Add;

use crate::Rollable;

/// Represents a single die.
/// Has an initial random value.
///
/// # Examples
/// ## Adding roll values
/// ```
/// use one_d_six::Die;
///
/// let mut d4 = Die::new(4);
/// let mut d6 = Die::new(6);
///
/// let d4_result = d4.roll();
/// let d6_result = d6.roll();
/// let result = d4_result + d6_result;
///
/// assert!(result >= 2);
/// assert!(result <= 10);
/// ```
///
/// ## Adding current values
/// ```
/// use one_d_six::Die;
///
/// let d4 = Die::new(4);
/// let d6 = Die::new(6);
///
/// let d4_result = d4.current_face();
/// let d6_result = d6.current_face();
/// let result = d4_result + d6_result;
///
/// assert!(result >= 2);
/// assert!(result <= 10);
/// ```
///
/// ## Adding dice directly
/// ```
/// use one_d_six::Die;
///
/// let d4: Die = Die::new(4);
/// let d6 = Die::new(6);
///
/// let result = d4 + d6;
///
/// assert!(result >= 2);
/// assert!(result <= 10);
/// ```
pub struct Die<T: Rollable = u32> {
    faces: T,
    current_value: T,
}

impl<T: Rollable> Die<T> {
    /// Creates a single die with the specified number of faces.
    ///
    /// # Example
    ///
    /// ```
    /// use one_d_six::Die;
    ///
    /// let coin = Die::new(2);
    /// ```
    pub fn new(faces: T) -> Self {
        let die = Die {
            faces,
            current_value: T::roll(faces),
        };
        die
    }
    /// Rolls a single die.
    ///
    /// # Example
    ///
    /// ```
    /// use one_d_six::Die;
    ///
    /// let mut d6 = Die::new(6);
    ///
    /// assert!(d6.roll() >= 1);
    /// assert!(d6.current_face() <= 6);
    /// ```
    pub fn roll(&mut self) -> T {
        self.current_value = T::roll(self.faces);
        self.current_value
    }
    /// Gets the current value of the die.
    ///
    /// # Example
    ///
    /// ```
    /// use one_d_six::Die;
    ///
    /// let d4 = Die::new(4);
    ///
    /// assert!(d4.current_face() >= 1);
    /// assert!(d4.current_face() <= 4);
    /// ```
    pub fn current_face(&self) -> T {
        self.current_value
    }
}

impl<T: Rollable> Add for Die<T>
where
    T: Add,
{
    type Output = T::Output;

    fn add(self, other: Self) -> Self::Output {
        self.current_value + other.current_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_face() {
        let coin: Die<u128> = Die::new(2);

        for _ in 0..100 {
            assert!(coin.current_face() >= 1);
            assert!(coin.current_face() <= 2);
        }
    }

    #[test]
    fn roll() {
        let mut d12: Die<u64> = Die::new(12);

        for _ in 0..100 {
            d12.roll();
            assert!(d12.current_face() >= 1);
            assert!(d12.current_face() <= 12);
        }
    }

    #[test]
    fn add_die() {
        for _ in 0..100 {
            let penny: Die<u8> = Die::new(2);
            let quarter: Die<u8> = Die::new(2);

            let sum = penny + quarter;

            assert!(sum >= 2);
            assert!(sum <= 4);
        }
    }
}
