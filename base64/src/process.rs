use crate::{utils::get_content, Base64Format};
use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

pub fn process_encode(input: &str, format: Base64Format) -> Result<String> {
    let content = get_content(input)?;
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&content),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&content),
    };

    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<String> {
    let content = get_content(input)?;

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(&content)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(&content)?,
    };

    Ok(String::from_utf8(decoded)?)
}

#[cfg(test)]
mod tests {
    use anyhow::Error;

    use crate::process_encode;

    #[test]
    fn test_process_encode() -> Result<(), Error> {
        let input = "../Cargo.toml";
        let format = crate::Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
        Ok(())
    }
}
