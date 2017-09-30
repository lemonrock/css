// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

/// A specified url() value
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SpecifiedUrl(pub String);

impl ToCss for SpecifiedUrl
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		dest.write_str("url(")?;
		dest.write_str(&self.0)?;
		dest.write_str(")")
	}
}

impl SpecifiedUrl
{
	/// See https://drafts.csswg.org/css-values/#local-urls
	pub fn is_fragment(&self) -> bool
	{
		self.0.chars().next().map_or(false, |c| c == '#')
	}
}
