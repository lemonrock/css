// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(plugin)]
#![plugin(phf_macros)]


#[macro_use] extern crate bitflags;
pub extern crate cssparser;
extern crate either;
pub extern crate ordermap;
extern crate phf;
extern crate precomputed_hash;
pub extern crate selectors;
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


include!("CustomParseError.rs");
include!("Stylesheet.rs");
