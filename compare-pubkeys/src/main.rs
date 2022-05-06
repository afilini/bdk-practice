use bitcoin::secp256k1::{All, Secp256k1, SecretKey};
use miniscript::descriptor::DescriptorPublicKey;

mod answer;

pub fn find_key<'h>(
    _needle: &SecretKey,
    _haystack: &'h [DescriptorPublicKey],
    _secp: &Secp256k1<All>,
) -> Result<&'h DescriptorPublicKey, &'static str> {
    Err("TODO")
}

#[test]
fn test_compare_pubkeys() -> Result<(), Box<dyn std::error::Error>> {
    answer::validate(find_key)?;

    Ok(())
}
