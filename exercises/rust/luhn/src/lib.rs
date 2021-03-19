/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let striped_code = code.chars().filter(|c| !c.is_whitespace()).collect::<Vec<_>>();
    if striped_code.len() < 2 || striped_code.iter().any(|ch|
        match ch.to_digit(10) {
            Some(_digit) => false,
            None => true,
        }
    ) { return false; }

    println!("----{:?}", striped_code);


    let sum = striped_code
        .iter()
        .enumerate()
        .fold(0, |acc, (index, ch)| {
            println!(">>>{} == {}", index, ch);

            let digit = match index {
                even if even % 2 != 0 => {
                    ch.to_digit(10).unwrap()
                },
                odd if odd % 2 == 0 => {
                    let x = ch.to_digit(10).unwrap() * 2;
                    if x > 9 { x - 9 } else { x }
                }
                _ => 0,
            };
            acc + digit
        });

    sum % 10 == 0
}
