// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A `<length>` without taking `calc` expressions into account
///
/// https://drafts.csswg.org/css-values/#lengths
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NoCalcLength
{
	/// An absolute length
	///
	/// https://drafts.csswg.org/css-values/#absolute-length
	Absolute(AbsoluteLength),
	
	/// A font-relative length:
	///
	/// https://drafts.csswg.org/css-values/#font-relative-lengths
	FontRelative(FontRelativeLength),
	
	/// A viewport-relative length.
	///
	/// https://drafts.csswg.org/css-values/#viewport-relative-lengths
	ViewportPercentage(ViewportPercentageLength),
}

impl ToCss for NoCalcLength
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		use self::NoCalcLength::*;
		
		match *self
		{
			Absolute(ref length) => length.to_css(dest),
			FontRelative(ref length) => length.to_css(dest),
			ViewportPercentage(ref length) => length.to_css(dest),
		}
	}
}

impl Mul<CssFloat> for NoCalcLength
{
	type Output = Self;
	
	#[inline]
	fn mul(self, scalar: CssFloat) -> Self
	{
		use self::NoCalcLength::*;
		
		match self
		{
			Absolute(value) => Absolute(value * scalar),
			FontRelative(value) => FontRelative(value * scalar),
			ViewportPercentage(value) => ViewportPercentage(value * scalar),
		}
	}
}

impl NoCalcLength
{
	/// Parse a given absolute or relative dimension.
	pub(crate) fn parse_dimension(context: &ParserContext, value: CssFloat, unit: &str) -> Result<Self, ()>
	{
		use self::NoCalcLength::*;
		use self::AbsoluteLength::*;
		use self::FontRelativeLength::*;
		use self::ViewportPercentageLength::*;
		
		let in_page_rule = context.rule_type.map_or(false, |rule_type| rule_type == CssRuleType::Page);
		match_ignore_ascii_case!
		{
			unit,
			
            "px" => Ok(Absolute(Px(value))),
            
            "in" => Ok(Absolute(In(value))),
            
            "cm" => Ok(Absolute(Cm(value))),
            
            "mm" => Ok(Absolute(Mm(value))),
            
            "q" => Ok(Absolute(Q(value))),
            
            "pt" => Ok(Absolute(Pt(value))),
            
            "pc" => Ok(Absolute(Pc(value))),
            
            "em" => Ok(FontRelative(Em(value))),
            
            "ex" => Ok(FontRelative(Ex(value))),
            
            "ch" => Ok(FontRelative(Ch(value))),
            
            "rem" => Ok(FontRelative(Rem(value))),
            
            "vw" =>
            {
                if in_page_rule
                {
                    return Err(())
                }
                Ok(ViewportPercentage(Vw(value)))
            }
            
            "vh" =>
            {
                if in_page_rule
                {
                    return Err(())
                }
                Ok(ViewportPercentage(Vh(value)))
            }
            
            "vmin" =>
            {
                if in_page_rule
                {
                    return Err(())
                }
                Ok(ViewportPercentage(Vmin(value)))
            }
            
            "vmax" =>
            {
                if in_page_rule
                {
                    return Err(())
                }
                Ok(ViewportPercentage(Vmax(value)))
            }
            
            _ => Err(())
        }
	}
	
	/// Returns a `zero` length.
	#[inline]
	pub fn zero() -> Self
	{
		use self::NoCalcLength::Absolute;
		use self::AbsoluteLength::Px;
		
		Absolute(Px(0.))
	}
	
	/// Checks whether the length value is zero.
	#[inline]
	pub fn is_zero(&self) -> bool
	{
		use self::NoCalcLength::Absolute;
		
		match *self
		{
			Absolute(length) => length.is_zero(),
			_ => false
		}
	}
	
	/// Get an absolute length from a px value.
	#[inline]
	pub fn from_px(px_value: CssFloat) -> Self
	{
		use self::NoCalcLength::Absolute;
		use self::AbsoluteLength::Px;
		
		Absolute(Px(px_value))
	}
}
