// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// An expected unit we intend to parse within a `calc()` expression.
///
/// This is used as a hint for the parser to fast-reject invalid expressions.
#[derive(Clone, Copy, PartialEq)]
pub enum CalcUnit
{
	/// `<number>`
	Number,
	
	/// `<integer>`
	Integer,
	
	/// `<length>`
	Length,
	
	/// `<percentage>`
	Percentage,
	
	/// `<length> | <percentage>`
	LengthOrPercentage,
	
	/// `<angle>`
	Angle,
	
	/// `<time>`
	Time,
}
