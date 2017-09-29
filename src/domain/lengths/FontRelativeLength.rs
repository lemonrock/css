// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A font relative length.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontRelativeLength
{
	/// A "em" value: https://drafts.csswg.org/css-values/#em
	Em(CSSFloat),
	
	/// A "ex" value: https://drafts.csswg.org/css-values/#ex
	Ex(CSSFloat),
	
	/// A "ch" value: https://drafts.csswg.org/css-values/#ch
	Ch(CSSFloat),
	
	/// A "rem" value: https://drafts.csswg.org/css-values/#rem
	Rem(CSSFloat)
}

impl ToCss for FontRelativeLength
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		use self::FontRelativeLength::*;
		
		match *self
		{
			Em(length) => serialize_dimension(length, "em", dest),
			Ex(length) => serialize_dimension(length, "ex", dest),
			Ch(length) => serialize_dimension(length, "ch", dest),
			Rem(length) => serialize_dimension(length, "rem", dest)
		}
	}
}
