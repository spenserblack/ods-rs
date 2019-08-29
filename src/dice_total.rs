use crate::Rollable;

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
impl DiceTotal<i8> for i8 {
    fn dice_total(dice_faces: Vec<i8>) -> i8 {
        dice_faces.iter().sum()
    }
}
impl DiceTotal<i16> for i16 {
    fn dice_total(dice_faces: Vec<i16>) -> i16 {
        dice_faces.iter().sum()
    }
}
impl DiceTotal<i32> for i32 {
    fn dice_total(dice_faces: Vec<i32>) -> i32 {
        dice_faces.iter().sum()
    }
}
impl DiceTotal<i64> for i64 {
    fn dice_total(dice_faces: Vec<i64>) -> i64 {
        dice_faces.iter().sum()
    }
}
impl DiceTotal<i128> for i128 {
    fn dice_total(dice_faces: Vec<i128>) -> i128 {
        dice_faces.iter().sum()
    }
}
impl DiceTotal<isize> for isize {
    fn dice_total(dice_faces: Vec<isize>) -> isize {
        dice_faces.iter().sum()
    }
}
