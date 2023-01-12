# Minimal reproducible example (MRE) for iced aw issue #[77](https://github.com/iced-rs/iced_aw/issues/77)

This project is intended to demonstrate a bug encountered when mixing [iced](https://github.com/iced-rs/iced/)'s pick_list and [iced_aw](https://github.com/iced-rs/iced_aw)'s `FloatingElement`. If run without argument (using `cargo run`) it displays a working dummy pick list. If run with any number of arguments (using e. g. `cargo run 1`) it displays a `FloatingElement` from iced_aw and the same pick list but the list of selectable values never shows up.
