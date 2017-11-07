// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A trait representing that !important can or can not be present in a property declaration
pub trait HasImportance: ToCss + Sized + Debug + Copy + Clone + Ord + PartialOrd + Eq + PartialEq + Hash
{
	/// Validate and convert importance, if permitted
	#[inline(always)]
	fn validateParsedImportance<'i>(importance: Importance) -> Result<Self, ParseError<'i, CustomParseError<'i>>>;
	
	/// Return whether this is an important declaration.
	#[inline(always)]
	fn isImportant(&self) -> bool;
}
