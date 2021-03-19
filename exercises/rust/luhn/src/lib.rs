/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .collect::<Vec<_>>()
        .iter()
        .map(|ch, index| {
            match index {
                even if index % 2 == 0 => {
                    ch.to_digit(10)
                },
                odd if index % 2 != 0 => {
                    ch.to_digit(10)
                }
            }
        })
}
