/// Check a Luhn checksum.
pub fn is_valid(input: &str) -> bool {
   let mut digits : Vec<u32> = Vec::new();
    for c in input.chars(){
        match c {
            '0'..='9' => digits.push(c.to_digit(10).unwrap()),
            ' ' => {}
            _=> return false,
        }
    }
if digits.len()<=1{
    return false;
}

    let sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(index, &digit)| {
            if index % 2 == 1 {
                let doubled = digit * 2;
                if doubled > 9 {
                    doubled -9
                }
                    else {
                    doubled
                    }
            }else {
                digit
                }
            })
        .sum();
    // A valid Luhn checksum must be divisible by 10
    sum.is_multiple_of(10)
    
}
