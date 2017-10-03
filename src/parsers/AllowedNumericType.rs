// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// Whether to allow negative units or not.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub enum AllowedNumericType
{
	/// Allow all kind of numeric values.
	All,
	
	/// Allow only non-negative numeric values.
	NonNegative,
	
	/// Allow only numeric values greater or equal to 1.0.
	AtLeastOne,
}

impl Default for AllowedNumericType
{
	#[inline]
	fn default() -> Self
	{
		AllowedNumericType::All
	}
}

impl AllowedNumericType
{
	/// Whether the value fits the rules of this numeric type.
	#[inline]
	pub fn is_ok(&self, parsing_mode: ParsingMode, value: f32) -> bool
	{
		use self::AllowedNumericType::*;
		
		if parsing_mode.allows_all_numeric_values()
		{
			true
		}
		else
		{
			match *self
			{
				All => true,
				NonNegative => value >= 0.0,
				AtLeastOne => value >= 1.0,
			}
		}
	}
	
	/// Clamp the value following the rules of this numeric type.
	#[inline]
	pub fn clamp(&self, value: f32) -> f32
	{
		use self::AllowedNumericType::*;
		
		match *self
		{
			NonNegative if value < 0. => 0.,
			AtLeastOne if value < 1. => 1.,
			_ => value,
		}
	}
}
