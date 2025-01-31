#![doc = include_str!("../README.md")]
pub mod asm;
pub mod cached_lines;
pub mod demangle;
pub mod llvm;
pub mod mir;
pub mod opts;

#[macro_export]
macro_rules! color {
    ($item:expr, $color:expr) => {
        owo_colors::OwoColorize::if_supports_color(&$item, owo_colors::Stream::Stdout, $color)
    };
}
