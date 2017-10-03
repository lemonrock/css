// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#![feature(plugin)]
#![plugin(phf_macros)]


#[macro_use] extern crate bitflags;
extern crate cssparser;
extern crate kuchiki;
extern crate num_traits;
extern crate ordermap;
extern crate phf;
extern crate precomputed_hash;
extern crate selectors;
extern crate smallvec;


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

/*

Analysis of autoprefixer rules still needed

Needed (Prefixes, either property names or property values)
css-animation
	-webkit- UC Browser for Android
transforms2d
	-webkit- UC Browser for Android
transforms3d
	-webkit- UC Browser for Android
	backface-visibility needs a prefix for Safari 9 / 10
css-filters
	-webkit- UC Browser, Samsung Internet, older Android Browser
user-select-none
	-ms-, -moz-, -webkit- but not -o-
font-feature
	-webkit- UC Browser for Android and older Samsung
font-kerning
	-webkit- for iOS Safari, surprisingly
css-hyphens
	-ms-, -webkit-
	-broken on new blink (chrome, opera)
css3-tabsize
	-moz- required
css-sticky

Needed (Selectors)
css-selection
	- the ::selection psuedo-element needs a -moz- prefix
css-placeholder
	- the ::selection psuedo-element needs a -ms- and -webkit- prefix, but is a bit messy: see http://caniuse.com/#search=css-placeholder&feature_sort=alpha
fullscreen
	-ms-, -moz-, -webkit- prefixes
	- the :fullscreen and ::backdrop selectors (note the single colon for :fullscreen is correct)
	- given the current status is quite a mess, probably best avoided
intrinsic-width
	- a bit of a mess
text-decoration
	- prefix with -webkit; avoid text-decoration-skip and text-decoration-style
	(prefix text-decoration-skip with -webkit- and covert values to none and skip); blink browsers only support objects and ink
text-size-adjust
	- prefix with -moz and -webkit
css-masks
	- prefix with -moz and -webkit
css-clip-path
	- prefix with -webkit
css-boxdecorationbreak
	- prefix with -webkit
css-logical-props
	- prefix with -webkit, -moz
css-appearance
	- prefix with -webkit, -moz
css-writing-mode
	- prefix with -webkit
css-read-only-write
	- prefix ':read-write' selector with -moz
	- prefix ':read-only' selector with -moz
text-emphasis
	- prefix with -webkit
css-grid
	- prefix with -ms
css-any-link
	- prefix with -moz, -webkit for the :any-link CSS pseudo-class

Complex mess not readily supportable
multicolumn
	- lots of bugs, some historic, some of which can't be fixed with a simple autoprefixer
flexbox
	- only older syntax supported by UC Browser for Android, and with -webkit- prefix
	- IE 11 has a large number of bugs making flexbox effectively unsable
css-crisp-edges
	- need -moz- prefix but only supports crisp-edges, however, chrome, with official support, only supports pixelated...
	- need -ms- for IE111, but IE11 is quite broken
	- need -webkit- for UC Browser and a non-standard value, too

Not enough support
css-snappoints
	- not supported in chrome, so not interesting
css-regions
	- not supported in chrome or firefox
css-image-set
	- prefix with -webkit; needs parsing of the background property to properly implement
css-cross-fade
	- prefix with -webkit; needs parsing of data

Viewport
	css-deviceadaptation
		- add -ms- prefix; remove all but width and height
		- for older opera mobile, -o- prefix, remove all but orientation

Media Queries
	css-media-resolution
		- need -webkit- prefix
		- IE11 only supports the dpi unit


Non-Standard
	- Firefox, Chrome and Safari support background-img-opts / -webkit-background-clip: text


Not Needed
border-radius
css-boxshadow
css-transitions
css-gradients
css3-boxsizing
expressions
border-image
css3-cursors-newer
css3-cursors-grab
pointer
object-fit
css-shapes
text-overflow
css-text-align-last
css-unicode-bidi

Not enough support
css-filter-function
css-element-function	needs -moz-
css-backdrop-filter		needs -webkit-, but only supported by Safari

*/
