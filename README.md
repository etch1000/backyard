# Backyard

The purpose of this crate is to understand how the modules work.

We define a module in let's say src/main.rs as `mod garden;`

The compiler will look for the module's code in these places:

- Inline, within curly bracket that replace the semicolon following mod garden
- In the file src/garden.rs
- In the file src/garden/mod.rs

To declare submodules, let's say we define a submodule `mod vegetable;` inside the garden module

The compiler will look for the module's code in these places:

- Inline, directly following `mod vegetables`, within curly brackets instead of the semicolon
- In the file src/garden/vegetables.rs
- In the file src/garden/vegetables/mod.rs
