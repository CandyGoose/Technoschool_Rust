fn set_bit(number: i64, bit_index: u32, value: bool) -> i64 {
    if value {
        number | (1 << bit_index)
    } else {
        number & !(1 << bit_index)
    }
}

fn main() {
    let number: i64 = 0b1010; // Исходное число
    let bit_index: u32 = 2;

    let updated_number = set_bit(number, bit_index, true);
    println!("After setting bit {}: {:b}", bit_index, updated_number);

    let updated_number = set_bit(updated_number, bit_index, false);
    println!("After clearing bit {}: {:b}", bit_index, updated_number);
}
