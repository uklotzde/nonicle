// SPDX-FileCopyrightText: The nonicle authors
// SPDX-License-Identifier: MPL-2.0

#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(missing_debug_implementations)]
#![warn(unreachable_pub)]
#![warn(unsafe_code)]
#![warn(clippy::pedantic)]
// Repeating the type name in `..Default::default()` expressions
// is not needed since the context is obvious.
#![allow(clippy::default_trait_access)]
#![warn(rustdoc::broken_intra_doc_links)]
#![cfg_attr(not(feature = "std"), no_std)]

//! # nonicle
//!
//! Tools for type-safe, canonical data representations.
