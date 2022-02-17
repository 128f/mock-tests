### Test of the mockall crate

Run `cargo test` to see overlapping tests fail because of shared context.

You can [see here](https://github.com/asomers/mockall/issues/338#issuecomment-923594273) the owner of the `mockall` repo mention that static methods have globally shared expectations, and the onus is on the user to synchronize them.
