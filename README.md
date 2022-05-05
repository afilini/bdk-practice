# bdk-practice

This repo is a collection of exercises to get familiar with rust-bitcoin, rust-miniscript and BDK.

It's made up of many small Rust projects that are part of the same workspace, so that they all share the same target directory.

The tasks generally involve completing a function or adding a fragment of code, which can then be automatically validated by running `cargo test` from within the folder.

The `src/answer.rs` file, which should be present in every project, contains the solution for the task, in case you need a bit of help getting un-stuck somewhere.

Contributions are very welcome, I hope going forward we'll be able to build a nice collection which we could use to help onboard people on BDK or, more broadly, the whole rust-bitcoin ecosystem.

## How to get started

Currently most tasks assume a decent amount of familiarity with the Rust languge and Bitcoin in general. Going forward we could consider adding easier tasks to lower the entry barrier.

To get started simply clone the repo locally and check out the individual tasks! You can try working on them and then running `cargo test` to see if you've got them right.

You are also encouraged to fork this repo so that you can share your attempts with others and get help if you need it.

## Licensing

The code and documentation present in this repo are licensed under the [Creative Commons CC0 1.0 Universal license](LICENSE).
