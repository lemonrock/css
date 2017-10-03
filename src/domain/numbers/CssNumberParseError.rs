// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CssNumberParseError
{
	Float(FloatParseError),
	Conversion(CssFloatConversionError),
}

impl Display for CssNumberParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		use self::CssNumberParseError::*;
		
		match *self
		{
			Float(ref error) => write!(f, "Could not parse string to CssFloat because parsing it as a float caused '{}'", error),
			Conversion(ref error) => write!(f, "Could not parse string to CssFloat because converting it from a parsed float caused '{}'", error),
		}
	}
}

impl Error for CssNumberParseError
{
	#[inline(always)]
	fn description(&self) -> &str
	{
		use self::CssNumberParseError::*;
		
		match *self
		{
			Float(ref error) => error.description(),
			Conversion(ref error) => error.description(),
		}
	}
	
	#[inline(always)]
	fn cause(&self) -> Option<&Error>
	{
		use self::CssNumberParseError::*;
		
		match *self
		{
			Float(ref error) => Some(error),
			Conversion(ref error) => Some(error),
		}
	}
}

impl From<FloatParseError> for CssNumberParseError
{
	#[inline(always)]
	fn from(error: FloatParseError) -> CssFloat
	{
		CssNumberParseError::Float(error)
	}
}

impl From<CssFloatConversionError> for CssNumberParseError
{
	#[inline(always)]
	fn from(error: CssFloatConversionError) -> CssFloat
	{
		CssNumberParseError::Conversion(error)
	}
}
