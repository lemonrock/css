// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// Represents an absolute length with its unit
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AbsoluteLength
{
	/// An absolute length in pixels (px)
	Px(CssFloat),
	
	/// An absolute length in inches (in)
	In(CssFloat),
	
	/// An absolute length in centimeters (cm)
	Cm(CssFloat),
	
	/// An absolute length in millimeters (mm)
	Mm(CssFloat),
	
	/// An absolute length in quarter-millimeters (q)
	Q(CssFloat),
	
	/// An absolute length in points (pt)
	Pt(CssFloat),
	
	/// An absolute length in pica (pc)
	Pc(CssFloat),
}

impl AbsoluteLength
{
	fn is_zero(&self) -> bool
	{
		use self::AbsoluteLength::*;
		
		match *self
		{
			Px(v) | In(v) | Cm(v) | Mm(v) | Q(v) | Pt(v) | Pc(v) => v == 0.,
		}
	}
	
	/// Convert this into a pixel value.
	#[inline]
	pub fn to_px(&self) -> CssFloat
	{
		use ::std::f32;
		use self::AbsoluteLength::*;
		
		let pixel = match *self
		{
			Px(value) => value,
			In(value) => value * (AppUnitsPerIN / AppUnitsPerPX),
			Cm(value) => value * (AppUnitsPerCM / AppUnitsPerPX),
			Mm(value) => value * (AppUnitsPerMM / AppUnitsPerPX),
			Q(value) => value * (AppUnitsPerQ / AppUnitsPerPX),
			Pt(value) => value * (AppUnitsPerPT / AppUnitsPerPX),
			Pc(value) => value * (AppUnitsPerPC / AppUnitsPerPX),
		};
		
		pixel.min(f32::MAX).max(f32::MIN)
	}
}

impl ToCss for AbsoluteLength
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		use self::AbsoluteLength::*;
		
		match *self
		{
			Px(length) => serialize_dimension(length, "px", dest),
			In(length) => serialize_dimension(length, "in", dest),
			Cm(length) => serialize_dimension(length, "cm", dest),
			Mm(length) => serialize_dimension(length, "mm", dest),
			Q(length) => serialize_dimension(length, "q", dest),
			Pt(length) => serialize_dimension(length, "pt", dest),
			Pc(length) => serialize_dimension(length, "pc", dest),
		}
	}
}

impl Mul<CssFloat> for AbsoluteLength
{
	type Output = Self;
	
	#[inline]
	fn mul(self, scalar: CssFloat) -> Self
	{
		use self::AbsoluteLength::*;
		
		match self
		{
			Px(v) => Px(v * scalar),
			In(v) => In(v * scalar),
			Cm(v) => Cm(v * scalar),
			Mm(v) => Mm(v * scalar),
			Q(v) => Q(v * scalar),
			Pt(v) => Pt(v * scalar),
			Pc(v) => Pc(v * scalar),
		}
	}
}

impl Add<AbsoluteLength> for AbsoluteLength
{
	type Output = Self;
	
	#[inline]
	fn add(self, rhs: Self) -> Self
	{
		use self::AbsoluteLength::*;
		
		match (self, rhs)
		{
			(Px(x), Px(y)) => Px(x + y),
			(In(x), In(y)) => In(x + y),
			(Cm(x), Cm(y)) => Cm(x + y),
			(Mm(x), Mm(y)) => Mm(x + y),
			(Q(x), Q(y)) => Q(x + y),
			(Pt(x), Pt(y)) => Pt(x + y),
			(Pc(x), Pc(y)) => Pc(x + y),
			_ => Px(self.to_px() + rhs.to_px()),
		}
	}
}
