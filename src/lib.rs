//! Provides utilities for rolling dice.
//!
//! # Examples
//!
//! ## Simple Usage
//!
//! ```
//! use one_d_six::quickroll;
//!
//! if quickroll::<u16>("1d2") == 1 {
//!     println!("Heads!");
//! } else {
//!     println!("Tails!");
//! }
//! ```
//!
//! ## Adding Sets of Dice Together
//!
//! ```
//! use one_d_six::Dice;
//!
//! // 3d6
//! let set_1 = Dice::new(3, 6);
//! // 2d4
//! let set_2: Dice = "2d4".parse().unwrap();
//!
//! // 3d6 + 2d4
//! let dice = set_1 + set_2;
//!
//! // Each set of dice starts pre-rolled
//! let roll = dice.total();
//!
//! println!("Result of 3d6 + 2d4 roll: {}", roll);
//! ```
//!
//! ## Getting Dice as String
//! ### Simple String
//!
//! ```
//! use one_d_six::Dice;
//!
//!
//! let dice: Dice = Dice::new(3, 6);
//! println!("3d6: {}", dice);
//! ```
//!
//! ### Complex String
//!
//! ```
//! use one_d_six::Dice;
//!
//!
//! // Will look like "1 2 3"
//! let dice: Dice = Dice::new(3, 6);
//! println!("3d6: {:?}", dice);
//! ```
use std::fmt;
use std::ops::Add;
use std::str::FromStr;

pub use dice::*;
pub use die::*;
pub use rollable::*;

mod dice;
mod die;
mod rollable;

/// Allows `one_d_six::Dice::total` to be used.
pub trait DiceTotal<T: Rollable> {
    fn dice_total(dice_faces: Vec<T>) -> T;
}

impl DiceTotal<u8> for u8 {
    fn dice_total(dice_faces: Vec<u8>) -> u8 {
        dice_faces.iter().sum()
    }
}
impl DiceTotal<u16> for u16 {
    fn dice_total(dice_faces: Vec<u16>) -> u16 {
        dice_faces.iter().sum()
    }
}
impl DiceTotal<u32> for u32 {
    fn dice_total(dice_faces: Vec<u32>) -> u32 {
        dice_faces.iter().sum()
    }
}
impl DiceTotal<u64> for u64 {
    fn dice_total(dice_faces: Vec<u64>) -> u64 {
        dice_faces.iter().sum()
    }
}
impl DiceTotal<u128> for u128 {
    fn dice_total(dice_faces: Vec<u128>) -> u128 {
        dice_faces.iter().sum()
    }
}
impl DiceTotal<usize> for usize {
    fn dice_total(dice_faces: Vec<usize>) -> usize {
        dice_faces.iter().sum()
    }
}

/// Attempts to roll dice based on a *1d6* style string.
///
/// # Example
///
/// ```
/// use one_d_six::try_quickroll;
///
/// if let Ok(roll) = try_quickroll::<u32>("1d6") {
///     assert!(roll >= 1);
///     assert!(roll <= 6);
/// } else {
///     unreachable!();
/// }
/// ```
pub fn try_quickroll<T: Rollable>(dice_format: &str) -> Result<T, String>
where
    T: DiceTotal<T>,
    T: FromStr,
{
    let dice: Dice<T> = dice_format.parse()?;
    Ok(dice.total())
}

/// Rolls dice based on a *1d6* style string.
///
/// # Example
///
/// ```
/// use one_d_six::quickroll;
///
/// let coin_flip: u8 = quickroll("1d2");
///
/// assert!(coin_flip == 1 || coin_flip == 2);
/// ```
///
/// # Panics
///
/// Panics if `dice_format` is in an improper format.
pub fn quickroll<T: Rollable>(dice_format: &str) -> T
where
    T: DiceTotal<T>,
    T: FromStr,
{
    let dice: Dice<T> = dice_format.parse().unwrap();
    dice.total()
}
