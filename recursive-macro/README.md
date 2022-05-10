# recursive-macro

Descriptors can be very useful when building layer-two protocols on top of bitcoin, because they let developers write their script "templates" once and then reuse them across their codebase.

For example, if you were to implement Lighting with descriptors you could represent a forwarded HTLC script as:

```
wsh(andor(pk(our),after(current_height + 6),and_v(v:pk(their),sha256(payment_hash))))
```

This lets us spend with `our` key `6` blocks after the current height, or it lets the receiver spend with `their` key plus the preimage of the `payment_hash`. Essentially this is a "template" with four variables
that would take different values depending on the specific channel and payment that is being forwarded.

If we wanted to implement a struct to wrap this template, a quick way to do it would be:

```rust
pub struct ForwardedHTCL {
    pub our_key: PrivateKey,
    pub their_key: PublicKey,
    pub current_height: u32,
    pub payment_hash: sha256::Hash,
}

impl ForwardedHTLC {
    pub fn get_descriptor(self) -> Result<(Descriptor<DescriptorPublicKey>, KeyMap), Error> {
        let desc = format!(wsh(andor(pk({}),after({}),and_v(v:pk({}),sha256({})))), self.our_key, self.current_height + 6, self.their_key, self.payment_hash);
        Descriptor::parse_descriptor(&desc)
    }
}
```

However, this approach has a few downsides:

- It's unnecessary slow, since it starts from deserialized keys in memory, serializes them to strings and then deserializes them again while parsing the descriptor
- It's only checked at runtime, if you make a mistake in the descriptor, even something obvious, it will only show up when execution hits that piece of code
- Serializing keys to strings may lose some information that they contained (for example, in BDK we keep track of the networks in which a key is valid separately, and this would not be serialized)

It would be much better if we could build the descriptor statically in the code: being able to parse a string is useful when we are gathering user inputs, but in this case we want to use "developer input", i.e. code.

One example of building the descriptor manually with code would roughly translate to (assuming you only have public keys because private keys are handled separately):

```rust
let descriptor = Descriptor::Wsh(
    Wsh::new(
        Miniscript::from_ast(
            Terminal::AndOr(
                Arc::new(Terminal::PkK(self.our_key)),
                Arc::new(Terminal::After(self.current_height + 6),
                Arc::new(Terminal::AndV(
                    Arc::new(Terminal::Verify(
                        Arc::new(Terminal::PkK(self.their_key))
                    )),
                    Arc::new(Terminal::Sha256(self.payment_hash))
                ))
            )
        )?
    )?
);
```

... which is not exactly easy to read or write, especially as scripts become larger and more complex.

And this is exactly why we need a macro! We can move most of the boring code that just wraps stuff into a macro and be left with a very clear syntax that gets expanded into that large tree. Bonus points if the syntax
is exactly the same (or very similar) to the "string" descriptor syntax.

## Task Explanation

In this task you are given a subset of the descriptor language consisting of:

- `Node::Pk` which is translated to `pk(KEY)`
- `Node::After` which is translated to `after(VALUE)`
- `Node::Verify` which is a wrapper applied to other operands and translates to `v:`
- `Node::Thresh` which can be used to put together multiple operands and translates to `thresh(THRESH, NODE LIST...)`

Note that it's allowed to have multiple wrappers on the same operands, i.e. `v:v:pk(KEY)` which translates to `Verify(Verify(Pk(KEY)))` in "pseudocode".

Also, there are no top-level containers like `sh()` or `wsh()`.

The `src/parser.rs` file contains a string parser for the syntax which you are free to check out for reference. 

**Your task is to write a macro called "parse" that parses a descriptor string and returns a `Node` tree**

## Reference

- [Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html)

## Tags

- Descriptor
- Macro

## Hints

This task is probably difficult for you if you are not familiar with rust macros. Below you can find a few hints that you may find helpful.

### Hint #1

You can use multiple macros if you'd like. The answer uses two, one to parse sequence of `Node`s and one to parse a single `Node`.

### Hint #2

If you find it hard to match the wrappers (`v:`), just start with the operands like `pk` and `after`. You should already start seeing some tests passing once that's implemented.
