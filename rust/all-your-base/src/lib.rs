#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    validate_number(number, from_base, to_base)?;

    let result = from_dec(to_dec(number, from_base), to_base);

    return if result.is_empty() { Ok(vec![0]) } else { Ok(result) };
}


fn validate_number(number: &[u32], from_base: u32, to_base: u32) -> Result<(), Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    number.iter()
        .try_for_each(|&d|
            if d >= from_base { Err(Error::InvalidDigit(d)) } else { Ok(()) })?;

    return Ok(());
}
fn to_dec(number: &[u32], from_base: u32) -> u32 {
    number.iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| d * from_base.pow(i as u32))
        .sum()
}

fn from_dec(number: u32, to_base: u32) -> Vec<u32> {
    return (0..)
        .map(|i| number / to_base.pow(i as u32))
        .take_while(|&n| n > 0)
        .map(|n| n % to_base)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
}