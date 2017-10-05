// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(plugin)]
#![plugin(phf_macros)]


#[macro_use] extern crate bitflags;
#[macro_use] pub extern crate cssparser;
extern crate either;
pub extern crate ordermap;
extern crate phf;
extern crate precomputed_hash;
// To be re-introduced once selectors 0.19.0 lands in crates.io
// pub extern crate selectors;
pub extern crate smallvec;


use self::domain::*;
use self::domain::atRules::counterStyle::System;
use self::domain::atRules::namespace::Namespaces;
use self::parsers::*;
use self::serializers::*;
use ::cssparser::*;
use ::selectors::parser::SelectorParseError;
use ::std::fmt;


pub mod domain;
pub(crate) mod parsers;
pub(crate) mod serializers;

// This module is forked from the servo repository 'https://github.com/servo/servo' component selectors crate (components/selectors) at revision 4f984a6428a0f497e311a0800efa55166c15aac6
// To be removed once selectors 0.19.0 lands in crates.io
#[macro_use] extern crate log;
#[macro_use] extern crate matches;
extern crate fnv;
pub mod selectors;

// This module is forked from the servo repository 'https://github.com/servo/servo' component servo_arc crate (components/servo_arc) at revision 4f984a6428a0f497e311a0800efa55166c15aac6
// To be removed once selectors 0.19.0 lands in crates.io
extern crate nodrop;
extern crate stable_deref_trait;
pub mod servo_arc;


include!("CustomParseError.rs");
include!("Stylesheet.rs");
