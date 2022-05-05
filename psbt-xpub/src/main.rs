use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::psbt;
use miniscript::descriptor::{Descriptor, DescriptorPublicKey};
use std::str::FromStr;

mod answer;

pub fn add_xpub_to_psbt(
    descriptor: &Descriptor<DescriptorPublicKey>,
    psbt: &mut psbt::PartiallySignedTransaction,
) -> Result<(), &'static str> {
    // TODO...

    Ok(())
}

#[test]
fn test_psbt_xpub() -> Result<(), Box<dyn std::error::Error>> {
    let secp = Secp256k1::new();

    let (descriptor, _) = Descriptor::parse_descriptor(&secp, "wsh(multi(2,[e133565a/1'/2'/3']tpubDC8K7EQY9egvvqVsM6gyjRxHLfC4zMxguhh1nNneFdMeK3iC3HsdTexhZCK5fEex3MsU2RyYPgRAkTvMfkWhwj6yoZmUzm9z8emn8mruy2M/4/5/*,tpubD6NzVbkrYhZ4WSCJG9szAk2F4rDuJddqgr3ae1AKrrJyrocJrB9t6rXBTynWyYkwgzcqUxY2R3yqEYYE5jT4P3NKjgv4YoJ7e4y4NksPf3n/*))#j76l64gc")?;
    let mut psbt = psbt::PartiallySignedTransaction::from_str("cHNidP8BAH0BAAAAAQ3HyUQHwx1NpPRpUvVwNCmDKWMy6VtmSj038jKqTzd/AAAAAAD/////AgDh9QUAAAAAFgAUAikS79yljFIeTqaqNpfa+USveElL6KQ1AAAAACIAIIDvIjhlHI6r3UvglFyc7pWULLemun0aKEY2V7PX7Rt5AAAAAAABAP0BAQIAAAAAAQEe8/Zof1b266z0FpSNiuN9ltpVE2xjXg0ACEXOe+46HQAAAAAXFgAUUtgATHY5L5QAZdPC1eiruMdtbmn+////AgDKmjsAAAAAIgAgj/3r3R4GUfb78Aw4jdmyb4lXjf35sqgS0+1542vGndDIf/AIAAAAABYAFPMKs8HhI1usGFTUDF8xfCsVc9clAkcwRAIgC5zx/UXWgrxn+WAf2K/CaP1Gwgv61y4vv10+LZnk3QsCIAiGy5i3nuH2A0GE/A3xGCqtGopGVorAq1MLbh5qztWnASECXmE4Y/S1qQHH3662aUjXupIY1r90dPdqVDC2Q+Mq0D8AAAAAAQErAMqaOwAAAAAiACCP/evdHgZR9vvwDDiN2bJviVeN/fmyqBLT7Xnja8ad0AEFR1IhAqS1MCM5jMJWZSK6Oy1pzINVuAf2brtuWX8F8lU42DDUIQOlVeYf1lhDDJz08Qg5OsEhboinmJ8GEf5ylunfmyhRdFKuIgYCpLUwIzmMwlZlIro7LWnMg1W4B/Zuu25ZfwXyVTjYMNQc4TNWWgEAAIACAACAAwAAgAQAAAAFAAAAAwAAACIGA6VV5h/WWEMMnPTxCDk6wSFuiKeYnwYR/nKW6d+bKFF0CNTrB7MDAAAAAAAiAgL/RReN3yMWOnD4coPvE5rZit6ntd39hAlXv/JLXXgyoxzhM1ZaAQAAgAIAAIADAACABAAAAAUAAAAGAAAAIgIDRLyG8o99MgGPWFcON7TRCAF1C7p1fnBGmGetqPl5BmkI1OsHswYAAAAA")?;

    add_xpub_to_psbt(&descriptor, &mut psbt)?;
    answer::validate(&psbt)?;

    Ok(())
}
