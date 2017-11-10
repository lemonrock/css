// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(plugin)]
#![plugin(phf_macros)]


//! # css
//!
//! A Rust library crate for parsing, manipulating and serializing CSS stylesheets.
//! Makes use of existing CSS parser logic from Servo.
//! Includes forks of code from Servo because these are unpublished on <https://crates.io>.
//! One use of this library is to minify CSS, as the serialized form it produces is minimal.
//! Another use is provide a crate that others can use for auto-prefixing CSS and to eliminate unused CSS.
//! The values of property declarations are currently stored as a string. Parsing property declarations is a monster job (in effect there are bespoke rules for every property). If you feel like helping...
//!
//!
//! ## Usages
//!
//!
//! ### Loading and Saving Stylesheets
//!
//! ```
//! extern crate css;
//! use ::css::Stylesheet;
//!
//! let some_css = "margin-left: 10pt; top: 20px;".to_owned();
//! let stylesheet = Stylesheet::parse(&some_css).expect("CSS was invalid");
//!
//! // Alternatively, load from a file using Stylesheet::from_file_path("/path/to/stylesheet.css").unwrap();
//!
//! let mut destination = String::new();
//!
//! // Don't write source-map and source-url comments if any are present in the stylesheet
//! let include_source_urls = false;
//!
//! stylesheet.to_css(&mut destination, include_source_urls).expect("Failed to write to destination");
//!
//! assert_eq!(&destination, "margin-left:10pt;top:20px");
//!
//! // To serialize to a Vec<u8> of bytes instead
//! let mut bytes = stylesheet.to_bytes();
//!
//! // To serialize to a file instead
//! stylesheet.to_file_path("/path/to/to/stylesheet.css").unwrap();
//! ```
//!
//!
//! ### To parse a single CSS selector
//!
//! ```
//! extern crate css;
//! use ::css::parse_selector;
//!
//! let selector = parse_selector("P.myclass").unwrap();
//! ```
//!
//!
//! ### To match CSS selectors to HTML
//!
//! Use the `html5ever_ext` crate. (The function `domain::selectors::matches()` can do matching but needs a lot of HTML logic to do so).
//!


#[macro_use] extern crate bitflags;
#[macro_use] pub extern crate cssparser;
extern crate either;
pub extern crate ordermap;
extern crate phf;
extern crate precomputed_hash;
#[macro_use] extern crate quick_error;
// To be re-introduced once selectors 0.19.0 lands in crates.io
// pub extern crate selectors;
pub extern crate smallvec;


use self::domain::*;
use self::domain::atRules::counterStyle::System;
use self::domain::atRules::namespace::Namespaces;
use self::domain::selectors::*;
use self::parsers::*;
use self::serializers::*;
use ::cssparser::*;
use ::quick_error::ResultExt;
use ::selectors::parser::SelectorParseError;
use ::std::fmt;
use ::std::fs::File;
use ::std::io::Read;
use ::std::mem::uninitialized;
use ::std::path::Path;
use ::std::path::PathBuf;


/// Contains definitions of objects used in Stylesheet.
pub mod domain;
pub(crate) mod parsers;
pub(crate) mod serializers;


// This module is forked from the servo repository 'https://github.com/servo/servo' component selectors crate (components/selectors) at revision 4f984a6428a0f497e311a0800efa55166c15aac6
// To be removed once selectors 0.19.0 lands in crates.io
#[macro_use] extern crate log;
#[macro_use] extern crate matches;
extern crate fnv;
/// Temporary fork of CSS selectors as the version we need is not yet available in <https://crates.io>.
pub mod selectors;


// This module is forked from the servo repository 'https://github.com/servo/servo' component servo_arc crate (components/servo_arc) at revision 4f984a6428a0f497e311a0800efa55166c15aac6
// To be removed once selectors 0.19.0 lands in crates.io
extern crate nodrop;
extern crate stable_deref_trait;
/// Temporary fork of Servo Arc as the version we need to support CSS selectors is not yet available in <https://crates.io>.
pub mod servo_arc;


include!("BlockingIoOnlyStdFmtWriteToStdIoWriteAdaptor.rs");
include!("CustomParseError.rs");
include!("parse_selector.rs");
include!("Stylesheet.rs");
include!("StylesheetError.rs");
