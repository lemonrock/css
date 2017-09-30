// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A computed percentage.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Percentage(pub CssFloat);

impl Percentage
{
	/// 0%
	#[inline]
	pub fn zero() -> Self
	{
		Percentage(0.)
	}
	
	/// 100%
	#[inline]
	pub fn hundred() -> Self
	{
		Percentage(1.)
	}
	
	/// Returns the absolute value for this percentage.
	#[inline]
	pub fn abs(&self) -> Self
	{
		Percentage(self.0.abs())
	}
}

impl ToCss for Percentage
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		serialize_percentage(self.0, dest)
	}
}
