use anyhow::Result;
use std::{fs::File, io::Read};

fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    Ok(reader)
}

pub fn get_content(input: &str) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut content = Vec::new();
    reader.read_to_end(&mut content)?;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use anyhow::Error;

    use super::get_content;

    #[test]
    fn test_get_content() -> Result<(), Error> {
        let input = "../Cargo.toml";
        assert!(get_content(input).is_ok());
        Ok(())
    }
}
