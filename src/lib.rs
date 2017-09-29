// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#![feature(plugin)]
#![plugin(phf_macros)]


#[macro_use] extern crate bitflags;
extern crate cssparser;
extern crate num_traits;
extern crate phf;


use self::domain::*;
use self::domain::atRules::counterStyle::System;
use self::parsers::*;
use self::serializers::*;
use ::cssparser::*;
use ::std::fmt;


pub mod domain;
pub(crate) mod parsers;
pub(crate) mod serializers;


include!("CustomParseError.rs");
include!("parse.rs");
include!("str.rs");


/*

			parse_family_name_list()
			serialize_comma_separated_list()
			
			try_match_ident_ignore_ascii_case!
			
			match!
			
			#[derive(..., ToCss)]
*/
/*

FamilyName

Expression (for media)
?Length
Angle
Time



	
	
	Look into selectors crate:-
	
	SelectorParser
	SelectorList
	SelectorImpl
	
	Also, in selectors there are:-
		type NamespaceUrl: Clone + Eq + Default + Borrow<Self::BorrowedNamespaceUrl> + PrecomputedHash
		 type NamespacePrefix: Clone + Eq + From<String> + for<'a> From<&'a str> + Display + Default;
   
*/

/*

// https://github.com/servo/servo/blob/097cea240fe9b1d96fa4ef5ffa984f09e0bd04f3/components/style_traits/values.rs
https://github.com/servo/servo/blob/3db1776326619be91db4ae15005b52eefc289e8a/components/style/values/specified/length.rs
	Font value structs and enums

/// The error type for all CSS parsing routines.
pub type ParseError<'i> = cssparser::ParseError<'i, SelectorParseError<'i, StyleParseError<'i>>>;

#[derive(Clone, Debug, PartialEq)]
/// Errors that can be encountered while parsing CSS values.
pub enum StyleParseError<'i> {
    /// A bad URL token in a DVB.
    BadUrlInDeclarationValueBlock(CowRcStr<'i>),
    /// A bad string token in a DVB.
    BadStringInDeclarationValueBlock(CowRcStr<'i>),
    /// Unexpected closing parenthesis in a DVB.
    UnbalancedCloseParenthesisInDeclarationValueBlock,
    /// Unexpected closing bracket in a DVB.
    UnbalancedCloseSquareBracketInDeclarationValueBlock,
    /// Unexpected closing curly bracket in a DVB.
    UnbalancedCloseCurlyBracketInDeclarationValueBlock,
    /// A property declaration parsing error.
    PropertyDeclaration(PropertyDeclarationParseError<'i>),
    /// A property declaration value had input remaining after successfully parsing.
    PropertyDeclarationValueNotExhausted,
    /// An unexpected dimension token was encountered.
    UnexpectedDimension(CowRcStr<'i>),
    /// Expected identifier not found.
    ExpectedIdentifier(Token<'i>),
    /// Missing or invalid media feature name.
    MediaQueryExpectedFeatureName(CowRcStr<'i>),
    /// Missing or invalid media feature value.
    MediaQueryExpectedFeatureValue,
    /// min- or max- properties must have a value.
    RangedExpressionWithNoValue,
    /// A function was encountered that was not expected.
    UnexpectedFunction(CowRcStr<'i>),
    /// @namespace must be before any rule but @charset and @import
    UnexpectedNamespaceRule,
    /// @import must be before any rule but @charset
    UnexpectedImportRule,
    /// Unexpected @charset rule encountered.
    UnexpectedCharsetRule,
    /// Unsupported @ rule
    UnsupportedAtRule(CowRcStr<'i>),
    /// A placeholder for many sources of errors that require more specific variants.
    UnspecifiedError,
    /// An unexpected token was found within a namespace rule.
    UnexpectedTokenWithinNamespace(Token<'i>),
    /// An error was encountered while parsing a property value.
    ValueError(ValueParseError<'i>),
}

/// Specific errors that can be encountered while parsing property values.
#[derive(Clone, Debug, PartialEq)]
pub enum ValueParseError<'i> {
    /// An invalid token was encountered while parsing a color value.
    InvalidColor(Token<'i>),
    /// An invalid filter value was encountered.
    InvalidFilter(Token<'i>),
}

impl<'a> From<ValueParseError<'a>> for ParseError<'a> {
    fn from(this: ValueParseError<'a>) -> Self {
        StyleParseError::ValueError(this).into()
    }
}

impl<'i> ValueParseError<'i> {
    /// Attempt to extract a ValueParseError value from a ParseError.
    pub fn from_parse_error(this: ParseError<'i>) -> Option<ValueParseError<'i>> {
        match this {
            cssparser::ParseError::Custom(
                SelectorParseError::Custom(
                    StyleParseError::ValueError(e))) => Some(e),
            _ => None,
        }
    }
}

/// The result of parsing a property declaration.
#[derive(Clone, Debug, PartialEq)]
pub enum PropertyDeclarationParseError<'i> {
    /// The property declaration was for an unknown property.
    UnknownProperty(CowRcStr<'i>),
    /// An unknown vendor-specific identifier was encountered.
    UnknownVendorProperty,
    /// The property declaration was for a disabled experimental property.
    ExperimentalProperty,
    /// The property declaration contained an invalid value.
    InvalidValue(CowRcStr<'i>, Option<ValueParseError<'i>>),
    /// The declaration contained an animation property, and we were parsing
    /// this as a keyframe block (so that property should be ignored).
    ///
    /// See: https://drafts.csswg.org/css-animations/#keyframes
    AnimationPropertyInKeyframeBlock,
    /// The property is not allowed within a page rule.
    NotAllowedInPageRule,
}

impl<'a> From<StyleParseError<'a>> for ParseError<'a> {
    fn from(this: StyleParseError<'a>) -> Self {
        cssparser::ParseError::Custom(SelectorParseError::Custom(this))
    }
}

impl<'a> From<PropertyDeclarationParseError<'a>> for ParseError<'a> {
    fn from(this: PropertyDeclarationParseError<'a>) -> Self {
        cssparser::ParseError::Custom(SelectorParseError::Custom(StyleParseError::PropertyDeclaration(this)))
    }
}

*/
// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of css, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


