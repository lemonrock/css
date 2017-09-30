// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A number from 0 to 1, indicating the percentage of the animation when this keyframe should run.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct KeyframePercentage(pub f32);

impl Ord for KeyframePercentage
{
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering
	{
		// We know we have a number from 0 to 1, so unwrap() here is safe.
		self.0.partial_cmp(&other.0).unwrap()
	}
}

impl Eq for KeyframePercentage
{
}

impl ToCss for KeyframePercentage
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		if self.0 == 1.
		{
			dest.write_str("to")
		}
		else
		{
			serialize_percentage(self.0, dest)
		}
	}
}

impl KeyframePercentage
{
	/// Trivially constructs a new `KeyframePercentage`.
	#[inline]
	pub fn new(value: f32) -> KeyframePercentage
	{
		debug_assert!(value >= 0. && value <= 1., "value must be between 0 and 1 inclusive");
		KeyframePercentage(value)
	}
	
	fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<KeyframePercentage, ParseError<'i, CustomParseError<'i>>>
	{
		let percentage = if input.try(|input| input.expect_ident_matching("from")).is_ok()
		{
			KeyframePercentage::new(0.)
		}
		else if input.try(|input| input.expect_ident_matching("to")).is_ok()
		{
			KeyframePercentage::new(1.)
		}
		else
		{
			let percentage = input.expect_percentage()?;
			if percentage >= 0. && percentage <= 1.
			{
				KeyframePercentage::new(percentage)
			}
			else
			{
				return Err(ParseError::Custom(CustomParseError::KeyframePercentageWasNotBetweenZeroAndOneInclusive(percentage)));
			}
		};
		
		Ok(percentage)
	}
}
