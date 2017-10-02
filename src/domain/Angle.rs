// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A computed angle.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Angle
{
	/// An angle with degree unit.
	Degree(CssFloat),
	
	/// An angle with gradian unit.
	Gradian(CssFloat),
	
	/// An angle with radian unit.
	Radian(CssFloat),
	
	/// An angle with turn unit.
	Turn(CssFloat),
}

impl ToCss for Angle
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		let mut write = |value: CssFloat, unit: &str|
		{
			value.to_css(dest)?;
			dest.write_str(unit)
		};
		
		use self::Angle::*;
		
		match *self
		{
			Degree(value) => write(value, "deg"),
			Gradian(value) => write(value, "grad"),
			Radian(value) => write(value, "rad"),
			Turn(value) => write(value, "turn"),
		}
	}
}

impl Angle
{
	/// Creates a computed `Angle` value from a radian amount.
	#[inline(always)]
	pub fn from_radians(radians: CssFloat) -> Self
	{
		Angle::Radian(radians)
	}
	
	#[inline(always)]
	pub(crate) fn from_calc(radians: CssFloat) -> Self
	{
		Self::from_radians(radians)
	}
	
	/// Returns the amount of radians this angle represents.
	#[inline(always)]
	pub fn radians(&self) -> CssFloat
	{
		use ::std::f32::MAX;
		use ::std::f32::MIN;
		
		self.radians64().min(MAX as f64).max(MIN as f64) as f32
	}
	
	#[inline(always)]
	pub fn radians64(&self) -> f64
	{
		use ::std::f64::consts::PI;
		use ::std::f64::MAX;
		use ::std::f64::MIN;
		
		const RadiansPerDegree: f64 = PI / 180.0;
		const RadianPerGradian: f64 = PI / 200.0;
		const RadiansPerTurn: f64 = PI * 2.0;
		
		use self::Angle::*;
		
		let radians = match *self
		{
			Degree(val) => val as f64 * RadiansPerDegree,
			Gradian(val) => val as f64 * RadianPerGradian,
			Radian(val) => val as f64,
			Turn(val) => val as f64 * RadiansPerTurn,
		};
		radians.min(MAX).max(MIN)
	}
	
	/// Returns an angle that represents a rotation of zero radians.
	pub fn zero() -> Self
	{
		Angle::Radian(0.0)
	}
}
