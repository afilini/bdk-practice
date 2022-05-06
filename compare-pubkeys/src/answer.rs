use bitcoin::secp256k1::{self, All, Secp256k1, SecretKey};
use bitcoin::{PrivateKey, PublicKey, XOnlyPublicKey};
use miniscript::descriptor::{DescriptorPublicKey, DescriptorSinglePub, SinglePubKey};
use std::str::FromStr;

#[allow(dead_code)]
pub fn find_key<'h>(
    needle: &SecretKey,
    haystack: &'h [DescriptorPublicKey],
    secp: &Secp256k1<All>,
) -> Result<&'h DescriptorPublicKey, &'static str> {
    // A "raw" secp256k1::PublicKey can be "converted" into a `bitcoin::PublicKey` or
    // `bitcoin::XOnlyPublicKey`, so that we can properly perform the comparison.
    //
    // When deriving xpubs we also get "raw" pks, which allows us to use extended keys for both
    // taproot (x_only) and non-taproot scripts.
    let raw_pk = secp256k1::PublicKey::from_secret_key(secp, &needle);

    haystack
        .into_iter()
        .find(|pk| match pk {
            DescriptorPublicKey::SinglePub(DescriptorSinglePub {
                key: SinglePubKey::FullKey(fk),
                ..
            }) => &PublicKey::new(raw_pk) == fk,
            DescriptorPublicKey::SinglePub(DescriptorSinglePub {
                key: SinglePubKey::XOnly(xonly),
                ..
            }) => &XOnlyPublicKey::from(raw_pk) == xonly,
            DescriptorPublicKey::XPub(xkey) => {
                let derived = xkey.xkey.derive_pub(secp, &xkey.derivation_path).unwrap();

                raw_pk == derived.public_key
            }
        })
        .ok_or("Key not found")
}

pub fn validate<
    F: for<'h> Fn(
        &SecretKey,
        &'h [DescriptorPublicKey],
        &Secp256k1<All>,
    ) -> Result<&'h DescriptorPublicKey, &'static str>,
>(
    func: F,
) -> Result<(), Box<dyn std::error::Error>> {
    let secp = Secp256k1::new();

    let test_cases = vec![
        ("cRdkA6GRwJU1QJwGvwz2d4BWgBGDH7CTxC2mdQSwfj6RKZreBkTN", "02eb8f286ec6729c4834313a8c4d6c747e879e77625a0b1b727331d056ae99fe9e"),
        ("cUQE1o1du6sdVc1r7yfPMQUe1wK8SL6LrGfnncUtbLeQiQGGviHo", "3933d844b985fe137b51d81b43780ee0a1da696db7a66bc3c2438160b0ad9c31"),
        ("cRhENUqN6b3Ua2hVV1iemk86CSC6KR8c2E15ddWbwhtAhtRXC2y3", "2db6ef4e91944d5eb407fae46f0a83ade9fcead2294b274cb7f7d655040fa90a"),
        ("cUA9BceDsEDp8KY1SaugWrtZmUgF5X6nv7kUQRdvksfyQcyWBura", "tpubD6NzVbkrYhZ4WaWSyoBvQwbpLkojyoTZPRsgXELWz3Popb3qkjcJyJUGLnL4qHHoQvao8ESaAstxYSnhyswJ76uZPStJRJCTKvosUCJZL5B/1/2/3/4"),
    ];

    let (sks, pks): (Vec<_>, Vec<_>) = test_cases
        .iter()
        .enumerate()
        .map(|(i, (sk, pk))| {
            (
                (PrivateKey::from_str(sk).unwrap().inner, i),
                DescriptorPublicKey::from_str(pk).unwrap(),
            )
        })
        .unzip();

    for (sk, i) in &sks {
        assert_eq!(func(sk, &pks, &secp)?, &pks[*i]);
    }

    Ok(())
}
