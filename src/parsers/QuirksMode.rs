// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum QuirksMode
{
	/// Quirks mode.
	Quirks,
	
	/// Limited quirks mode.
	LimitedQuirks,
	
	/// No quirks mode.
	NoQuirks,
}

impl QuirksMode
{
	#[inline]
	pub(crate) fn classes_and_ids_are_case_sensitive(self) -> bool
	{
		use self::QuirksMode::*;
		
		match self
		{
			NoQuirks | LimitedQuirks => true,
			Quirks => false,
		}
	}
}
