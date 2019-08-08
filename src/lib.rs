use std::ops::Add;
use rand::Rng;

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
/// let d4_result: u32 = d4.roll();
/// let d6_result: u32 = d6.roll();
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
/// let d4_result: u32 = d4.current_face();
/// let d6_result: u32 = d6.current_face();
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
/// let d4 = Die::new(4);
/// let d6 = Die::new(6);
///
/// let result = d4 + d6;
///
/// assert!(result >= 2);
/// assert!(result <= 10);
/// ```
pub struct Die {
    faces: u32,
    current_value: u32,
}

/// A Handful of dice.
///
/// # Example
///
/// ```
/// use one_d_six::Dice;
///
/// let mut dice: Dice = "3d6".parse();
/// let mut sum: u32 = 0;
///
/// for face in dice.roll_all() {
///     sum += face;
/// }
///
/// assert!(sum >= 3);
/// assert!(sum <= 18);
/// ```
pub struct Dice {
    dice: Vec<Die>,
}

impl Die {
    /// Creates a single die with the specified number of faces.
    ///
    /// # Example
    ///
    /// ```
    /// use one_d_six::Die;
    ///
    /// let coin = Die::new(2);
    /// ```
    pub fn new(faces: u32) -> Self {
        let mut die = Die {
            faces,
            current_value: 1,
        };
        die.roll();
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
    pub fn roll(&mut self) -> u32 {
        let r: u32 = rand::random();
        self.current_value = (r % self.faces) + 1;
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
    pub fn current_face(&self) -> u32 {
        self.current_value
    }
}

impl Add for Die {
    type Output = u32;

    fn add(self, other: Self) -> Self::Output {
        self.current_value + other.current_value
    }
}

impl Dice {
    /// Creates a new set of dice.
    /// Each die in the set has an initial starting value.
    /// Only allows dice of same type. No mixture of d4 and d6.
    ///
    /// # Example
    ///
    /// ```
    /// use one_d_six::Dice;
    ///
    /// // Creates 3d6 dice collection
    /// let dice = Dice::new(3, 6);
    /// ```
    pub fn new(dice: usize, faces: u32) -> Self {
        let dice = {
            let mut v: Vec<Die> = Vec::with_capacity(dice);
            for _ in 0..dice {
                v.push(Die::new(faces));
            }
            v
        };

        Dice {
            dice
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_face() {
        let coin = Die::new(2);

        for _ in 0..100 {
            assert!(coin.current_face() >= 1);
            assert!(coin.current_face() <= 2);
        }
    }

    #[test]
    fn roll() {
        let mut d12 = Die::new(12);

        for _ in 0..100 {
            d12.roll();
            assert!(d12.current_face() >= 1);
            assert!(d12.current_face() <= 12);
        }
    }

    #[test]
    fn add_die() {
        for _ in 0..100 {
            let penny = Die::new(2);
            let quarter = Die::new(2);

            let sum = penny + quarter;

            assert!(sum >= 2);
            assert!(sum <= 4);
        }
    }
}
