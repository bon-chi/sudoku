#![warn(missing_docs)]
//#![allow(unused)]
#![cfg_attr(feature = "cargo-clippy",
    allow(
        clippy_complexity,
        match_bool,
        unreadable_literal,
        needless_pass_by_value,
        wrong_self_convention,
        write_with_newline,
    )
)]
#![feature(generators, generator_trait, conservative_impl_trait)]
//! The Sudoku library
//!
//! ## Overview
//!
//! Sudoku is a library that aims to provide a simple API to solve sudokus
//! without having to deal with too much details.
//!
//! ## Example
//!
//! ```
//! use sudoku::Sudoku;
//!
//! let sudoku_block =
//! "___|2__|_63
//! 3__|__5|4_1
//! __1|__3|98_
//! ---+---+---
//! ___|___|_9_
//! ___|538|___
//! _3_|___|___
//! ---+---+---
//! _26|3__|5__
//! 5_3|7__|__8
//! 47_|__1|___";
//!
//! let sudoku_line = "...2...633....54.1..1..398........9....538....3........263..5..5.37....847...1...";
//!
//! // Sudokus can be created from &str's in both block or line formats or directly from bytes.
//! let sudoku = Sudoku::from_str_block_permissive(sudoku_block).unwrap();
//! let sudoku = Sudoku::from_str_line(sudoku_line).unwrap();
//! // Sudoku::from_bytes(some_bytes_arr);
//! // Sudoku::from_bytes_slice(some_slice);
//!
//! // Solve, print or convert the sudoku to another format
//! if let Some(solution) = sudoku.solve_unique() {
//!     println!("{}", solution);
//!     println!("{}", solution.to_str_line());
//!
//!     let cell_contents: [u8; 81] = solution.to_bytes();
//! }
//! ```
#![feature(test)]
#[cfg(feature="serde")] extern crate serde;
extern crate core;
extern crate rand;

mod consts;
mod types;
mod sudoku;
mod strategy_solver;
mod solver;
mod generator;
mod positions;

pub use sudoku::Sudoku;
//pub use strategy_solver::StrategySolver;
//pub use strategy_solver::strategies;
//pub use types::{LineFormatParseError, BlockFormatParseError, PubEntry as Entry};

/// Contains errors for the various parsing modes
pub mod parse_errors {
    pub use types::{LineFormatParseError, BlockFormatParseError, NotEnoughRows, PubEntry as Entry};
}
