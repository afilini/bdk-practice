use std::str::FromStr;

use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::{bip32, psbt};

use miniscript::descriptor::{Descriptor, DescriptorPublicKey, DescriptorXKey, WshInner};
use miniscript::miniscript::iter::PkPkh;
use miniscript::ForEachKey;

// This is part of `XKeyUtils`, an internal BDK API
fn root_fingerprint(key: &DescriptorXKey<bip32::ExtendedPubKey>) -> bip32::Fingerprint {
    match key.origin {
        Some((fingerprint, _)) => fingerprint,
        None => key.xkey.fingerprint(),
    }
}

// Quickest option, but we can't really return an error from within the closure
#[allow(dead_code)]
pub fn add_xpub_to_psbt(
    descriptor: &Descriptor<DescriptorPublicKey>,
    psbt: &mut psbt::PartiallySignedTransaction,
) -> Result<(), &'static str> {
    let secp = Secp256k1::new();
    descriptor.for_each_key(|pk| {
        if let DescriptorPublicKey::XPub(xpub) = pk.as_key() {
            let origin = match &xpub.origin {
                Some(origin) => origin.clone(),
                None if xpub.xkey.depth == 0 => (root_fingerprint(&xpub), vec![].into()),
                _ => panic!("Missing key origin"),
            };
            // Check if this key matches with the PBBT-input's bip32 keymap.
            // We need this to ensure we are not adding unrelated keys that might
            // be present in the descriptor, into the PSBT global keymap.
            psbt.inputs.iter().for_each(|input| {
                input.bip32_derivation.iter().for_each(|(_, keysource)| {
                    if xpub.matches(keysource, &secp).is_some() {
                        psbt.xpub.insert(xpub.xkey, origin.clone());
                    }
                })
            })
        }
        true
    });

    Ok(())
}

// Alternative option, avoids the closure but only works for `wsh(multi())` in this case (it would have to be
// expanded manually to handle all the script types)
#[allow(dead_code)]
pub fn add_xpub_to_psbt_alt(
    descriptor: &Descriptor<DescriptorPublicKey>,
    psbt: &mut psbt::PartiallySignedTransaction,
) -> Result<(), &'static str> {
    if let Descriptor::Wsh(wsh) = descriptor {
        if let WshInner::Ms(ms) = wsh.as_inner() {
            for pk in ms.iter_pk_pkh().into_iter() {
                // DescriptorPublicKey uses the same type for both `pk` and `pkh`.
                // `miniscript::ForEach` has a convenient ".as_key()" method that does exactly this
                // internally (see line 24). Unfortunately here we have to do this manually.
                // I have a PR to fix this: https://github.com/rust-bitcoin/rust-miniscript/pull/390
                let pk = match pk {
                    PkPkh::PlainPubkey(key) => key,
                    PkPkh::HashedPubkey(key) => key,
                };

                if let DescriptorPublicKey::XPub(xpub) = pk {
                    let origin = match xpub.origin {
                        Some(origin) => origin,
                        None if xpub.xkey.depth == 0 => (root_fingerprint(&xpub), vec![].into()),
                        _ => return Err("Missing key origin"),
                    };

                    psbt.xpub.insert(xpub.xkey, origin);
                }
            }
        }
    }

    Ok(())
}

pub fn validate(psbt: &psbt::PartiallySignedTransaction) -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(psbt.xpub.len(), 2);

    let mut iter = (&psbt.xpub).into_iter();

    assert_eq!(iter.next(), Some((&bip32::ExtendedPubKey::from_str("tpubD6NzVbkrYhZ4WSCJG9szAk2F4rDuJddqgr3ae1AKrrJyrocJrB9t6rXBTynWyYkwgzcqUxY2R3yqEYYE5jT4P3NKjgv4YoJ7e4y4NksPf3n")?, &(bip32::Fingerprint::from_str("d4eb07b3")?, bip32::DerivationPath::from_str("m")?))));
    assert_eq!(iter.next(), Some((&bip32::ExtendedPubKey::from_str("tpubDC8K7EQY9egvvqVsM6gyjRxHLfC4zMxguhh1nNneFdMeK3iC3HsdTexhZCK5fEex3MsU2RyYPgRAkTvMfkWhwj6yoZmUzm9z8emn8mruy2M")?, &(bip32::Fingerprint::from_str("e133565a")?, bip32::DerivationPath::from_str("m/1'/2'/3'")?))));

    Ok(())
}
