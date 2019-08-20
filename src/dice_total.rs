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
