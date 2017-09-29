// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

/// A declaration [importance][importance].
///
/// [importance]: https://drafts.csswg.org/css-cascade/#importance
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Importance
{
	/// Indicates a declaration without `!important`.
	Normal,
	
	/// Indicates a declaration with `!important`.
	Important,
}

impl Importance
{
	/// Return whether this is an important declaration.
	#[inline(always)]
	pub fn important(self) -> bool
	{
		use self::Importance::*;
		
		match self
		{
			Normal => false,
			Important => true,
		}
	}
	
	#[inline(always)]
	pub fn from_bool(isImportant: bool) -> Self
	{
		if isImportant
		{
			Importance::Important
		}
		else
		{
			Importance::Normal
		}
	}
}
