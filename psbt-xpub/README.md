# psbt-xpub

In this task we have a watch-only multisig descriptor. All of our keys are safely stored on
hardware wallets, which we are gonna use through HWI.

We have a PSBT which is ready to go, but using it as-is can lead to unexpected results: many
hardware wallets (like Coldcard and Specter) support multisigs natively, but they
need a bit of extra help to thoroughly check the PSBT.

The PSBT contains the full script (witness script in this case, because we are using `wsh`)
for every output, so the hardware wallets can check that there's at least one multisig
output which is, supposedly, the change. But how can they verify that the keys used in the
multisig are correct? Well, the answer is: most of the times they can't... unless we provide
them with all the xpubs!

If we do everything correctly, instead of showing two "foreign" outputs, the wallet will
correctly show one foreign output and one change, which is gonna be much less scary for
the end user.

**Your task is to modify this PSBT to fill in the [`PartiallySignedTransaction::xpub`][xpub] field**

## Reference

- [BIP174](https://github.com/bitcoin/bips/blob/master/bip-0174.mediawiki)

## Tags

- PSBT
- Descriptor
- Extended Keys


[xpub]: https://docs.rs/bitcoin/0.28.1/bitcoin/util/psbt/struct.PartiallySignedTransaction.html#structfield.xpub

