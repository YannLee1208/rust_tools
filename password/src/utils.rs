use anyhow::{anyhow, Error, Result};
use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

// generate password of given conditions. First choose one char from the given conditions. Then generate the left length from all of them.
// length - length of the password
// upper_case - if the password contains the upper chars
// lower_case - if the password contains the lower chars
// number - if the password contains numbers
// symbol - if the password contains symbols in "!@#$%^&*_"

pub fn process_password(
    length: u8,
    upper_case: bool,
    lower_case: bool,
    number: bool,
    symbol: bool,
) -> Result<String, Error> {
    if length <= 0 {
        return Err(anyhow!("length should be greater than 0"));
    }

    let mut rng = rand::thread_rng();
    let mut charset = Vec::new();
    let mut password = Vec::new();

    if upper_case {
        charset.extend_from_slice(UPPER);
        password.push(
            *UPPER
                .choose(&mut rng)
                .expect("choose from upper won't be none"),
        );
    }

    if lower_case {
        charset.extend_from_slice(LOWER);
        password.push(
            *LOWER
                .choose(&mut rng)
                .expect("choose from lower won't be none"),
        );
    }

    if number {
        validate_password_length(&password, length)?;
        charset.extend_from_slice(NUMBER);
        password.push(
            *NUMBER
                .choose(&mut rng)
                .expect("choose from number won't be none"),
        );
    }

    if symbol {
        validate_password_length(&password, length)?;
        charset.extend_from_slice(SYMBOL);
        password.push(
            *SYMBOL
                .choose(&mut rng)
                .expect("choose from number won't be none"),
        );
    }

    for _ in 0..(length - password.len() as u8) {
        let choose_char = charset.choose(&mut rng).expect("choose won't be none");
        password.push(*choose_char);
    }

    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;
    Ok(password)
}

fn validate_password_length(password: &[u8], length: u8) -> Result<(), Error> {
    if password.len() as u8 >= length {
        return Err(anyhow!(
            "password length too smaller, fail to satisfy the condition"
        ));
    }
    Ok(())
}
