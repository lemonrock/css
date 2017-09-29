// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A viewport-relative length.
///
/// https://drafts.csswg.org/css-values/#viewport-relative-lengths
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewportPercentageLength
{
	/// A vw unit: https://drafts.csswg.org/css-values/#vw
	Vw(CSSFloat),
	
	/// A vh unit: https://drafts.csswg.org/css-values/#vh
	Vh(CSSFloat),
	
	/// https://drafts.csswg.org/css-values/#vmin
	Vmin(CSSFloat),
	
	/// https://drafts.csswg.org/css-values/#vmax
	Vmax(CSSFloat)
}

impl ToCss for ViewportPercentageLength
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		use self::ViewportPercentageLength::*;
		
		match *self
		{
			Vw(length) => serialize_dimension(length, "vw", dest),
			Vh(length) => serialize_dimension(length, "vh", dest),
			Vmin(length) => serialize_dimension(length, "vmin", dest),
			Vmax(length) => serialize_dimension(length, "vmax", dest)
		}
	}
}

impl Mul<CSSFloat> for ViewportPercentageLength
{
	type Output = ViewportPercentageLength;
	
	#[inline]
	fn mul(self, scalar: CSSFloat) -> Self::Output
	{
		use self::ViewportPercentageLength::*;
		
		match self
		{
			Vw(v) => Vw(v * scalar),
			Vh(v) => Vh(v * scalar),
			Vmin(v) => Vmin(v * scalar),
			Vmax(v) => Vmax(v * scalar),
		}
	}
}
