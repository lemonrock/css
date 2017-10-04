// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum LengthOrPercentageUnit<Number: CssNumber>
{
	IsLength(LengthUnit<Number>),
	IsPercentage(PercentageUnit<Number>)
}

impl<Number: CssNumber> ToCss for LengthOrPercentageUnit<Number>
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		match *self
		{
			IsLength(ref length) => length.to_css(dest),
			IsPercentage(ref length) => length.to_css(dest),
		}
	}
}

impl<Number: CssNumber> Default for LengthOrPercentageUnit<Number>
{
	#[inline(always)]
	fn default() -> Self
	{
		LengthOrPercentageUnit::IsLength(LengthUnit::default())
	}
}

impl<Number: CssNumber> Add<Number> for LengthOrPercentageUnit<Number>
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: Number) -> Self::Output
	{
		match self
		{
			IsLength(length) => IsLength(length + rhs),
			IsPercentage(length) => IsPercentage(length + rhs),
		}
	}
}

impl<Number: CssNumber> AddAssign<Number> for LengthOrPercentageUnit<Number>
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: Number)
	{
		match *self
		{
			IsLength(ref mut length) => *length = *length + rhs,
			IsPercentage(ref mut length) => *length = *length + rhs,
		}
	}
}

impl<Number: CssNumber> Sub<Number> for LengthOrPercentageUnit<Number>
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: Number) -> Self::Output
	{
		match self
		{
			IsLength(length) => IsLength(length - rhs),
			IsPercentage(length) => IsPercentage(length - rhs),
		}
	}
}

impl<Number: CssNumber> SubAssign<Number> for LengthOrPercentageUnit<Number>
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: Number)
	{
		match *self
		{
			IsLength(ref mut length) => *length = *length - rhs,
			IsPercentage(ref mut length) => *length = *length - rhs,
		}
	}
}

impl<Number: CssNumber> Mul<Number> for LengthOrPercentageUnit<Number>
{
	type Output = Self;
	
	#[inline(always)]
	fn mul(self, rhs: Number) -> Self::Output
	{
		match self
		{
			IsLength(length) => IsLength(length * rhs),
			IsPercentage(length) => IsPercentage(length * rhs),
		}
	}
}

impl<Number: CssNumber> MulAssign<Number> for LengthOrPercentageUnit<Number>
{
	#[inline(always)]
	fn mul_assign(&mut self, rhs: Number)
	{
		match *self
		{
			IsLength(ref mut length) => *length = *length * rhs,
			IsPercentage(ref mut length) => *length = *length * rhs,
		}
	}
}

impl<Number: CssNumber> Div<Number> for LengthOrPercentageUnit<Number>
{
	type Output = Self;
	
	#[inline(always)]
	fn div(self, rhs: Number) -> Self::Output
	{
		match self
		{
			IsLength(length) => IsLength(length / rhs),
			IsPercentage(length) => IsPercentage(length / rhs),
		}
	}
}

impl<Number: CssNumber> DivAssign<Number> for LengthOrPercentageUnit<Number>
{
	#[inline(always)]
	fn div_assign(&mut self, rhs: Number)
	{
		match *self
		{
			IsLength(ref mut length) => *length = *length / rhs,
			IsPercentage(ref mut length) => *length = *length / rhs,
		}
	}
}

impl<Number: CssNumber> Rem<Number> for LengthOrPercentageUnit<Number>
{
	type Output = Self;
	
	#[inline(always)]
	fn rem(self, rhs: Number) -> Self::Output
	{
		match self
		{
			IsLength(length) => IsLength(length % rhs),
			IsPercentage(length) => IsPercentage(length % rhs),
		}
	}
}

impl<Number: CssNumber> RemAssign<Number> for LengthOrPercentageUnit<Number>
{
	#[inline(always)]
	fn rem_assign(&mut self, rhs: Number)
	{
		match *self
		{
			IsLength(ref mut length) => *length = *length % rhs,
			IsPercentage(ref mut length) => *length = *length % rhs,
		}
	}
}

impl<Number: CssNumber> Neg for LengthOrPercentageUnit<Number>
{
	type Output = Self;
	
	#[inline(always)]
	fn neg(self) -> Self::Output
	{
		match self
		{
			IsLength(length) => IsLength(-length),
			IsPercentage(length) => IsPercentage(-length),
		}
	}
}

impl<Number: CssNumber> CssNumberNewType <Number>for LengthOrPercentageUnit<Number>
{
	#[inline(always)]
	fn to_f32(&self) -> f32
	{
		self.to_CssNumber().to_f32()
	}
	
	#[inline(always)]
	fn as_CssNumber(&self) -> &Number
	{
		match *self
		{
			IsLength(ref length) => length.as_CssNumber(),
			IsPercentage(ref length) => length.as_CssNumber(),
		}
	}
}

impl<NumberX: CssNumber> Unit for LengthOrPercentageUnit<NumberX>
{
	type Number = NumberX;
	
	const HasDimension: bool = true;
	
	#[inline(always)]
	fn parse_one_outside_calc_function<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<CalculablePropertyValue<Self>, ParseError<'i, CustomParseError<'i>>>
	{
		use self::CalculablePropertyValue::*;
		use self::CustomParseError::*;
		
		match *input.next()?
		{
			Token::Number { value, .. } =>
			{
				if value == 0.
				{
					Ok(Constant(IsLength(LengthOrPercentageUnit::from_px(LengtUnit::Number::Zero))))
				}
				else
				{
					Err(ParseError::Custom(CouldNotParseDimensionLessNumber(value)))
				}
			}
			
			Token::Dimension { value, ref unit, .. } =>
			{
				let cssNumber = Self::Number::new(value).map_err(|cssNumberConversionError| ParseError::Custom(CouldNotParseCssSignedNumber(cssNumberConversionError, value)))?;
				
				match_ignore_ascii_case!
				{
					&*unit,
					
					"px" => Ok(Constant(IsLength(Absolute(px(cssNumber))))),
					
					"in" => Ok(Constant(IsLength(Absolute(in_(cssNumber))))),
					
					"cm" => Ok(Constant(IsLength(Absolute(cm(cssNumber))))),
					
					"mm" => Ok(Constant(IsLength(Absolute(mm(cssNumber))))),
					
					"q" => Ok(Constant(IsLength(Absolute(q(cssNumber))))),
					
					"pt" => Ok(Constant(IsLength(Absolute(pt(cssNumber))))),
					
					"pc" => Ok(Constant(IsLength(Absolute(pc(cssNumber))))),
					
					"em" => Ok(Constant(IsLength(FontRelative(em(cssNumber))))),
					
					"ex" => Ok(Constant(IsLength(FontRelative(ex(cssNumber))))),
					
					"ch" => Ok(Constant(IsLength(FontRelative(ch(cssNumber))))),
					
					"rem" => Ok(Constant(IsLength(FontRelative(rem(cssNumber))))),
					
					"vw" => if context.isNotInPageRule()
					{
						Ok(Constant(IsLength(ViewportPercentage(vw(cssNumber)))))
					}
					else
					{
						Err(ParseError::Custom(CustomParseError::ViewportLengthsAreNotAllowedInAPageAtRule))
					},
					
					"vh" => if context.isNotInPageRule()
					{
						Ok(Constant(IsLength(ViewportPercentage(vh(cssNumber)))))
					}
					else
					{
						Err(ParseError::Custom(CustomParseError::ViewportLengthsAreNotAllowedInAPageAtRule))
					},
					
					"vmin" => if context.isNotInPageRule()
					{
						Ok(Constant(IsLength(ViewportPercentage(vmin(cssNumber)))))
					}
					else
					{
						Err(ParseError::Custom(CustomParseError::ViewportLengthsAreNotAllowedInAPageAtRule))
					},
					
					"vmax" => if context.isNotInPageRule()
					{
						Ok(Constant(IsLength(ViewportPercentage(vmax(cssNumber)))))
					}
					else
					{
						Err(ParseError::Custom(CustomParseError::ViewportLengthsAreNotAllowedInAPageAtRule))
					},
					
					_ => Err(ParseError::Custom(CouldNotParseDimension(value, unit.clone()))),
				}
			}
			
			Token::Percentage { value, .. } =>
			{
				let cssNumber = Self::Number::new(value).map_err(|cssNumberConversionError| ParseError::Custom(CouldNotParseCssSignedNumber(cssNumberConversionError, value)))?;
				
				Ok(IsPercentage(Percentage(cssNumber)))
			}
			
			Token::Function(ref name) =>
			{
				match name
				{
					"calc" => Ok(CalcFunction(CalcExpression::parse(context, input)?)),
					
					"attr" => Ok(AttrFunction(AttrExpression::parse(context, input)?)),
					
					"var" => Ok(VarFunction(VarExpression::parse(context, input)?)),
					
					_ => return Err(ParseError::Custom(UnknownFunctionInValueExpression(name.to_owned())))
				}
			},
			
			unexpectedToken @ _ => Err(BasicParseError::UnexpectedToken(unexpectedToken.clone()).into()),
		}
	}
	
	#[inline(always)]
	fn parse_one_inside_calc_function<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Either<CalculablePropertyValue<Self>, CalcExpression<Self>>, ParseError<'i, CustomParseError<'i>>>
	{
		use self::CalculablePropertyValue::*;
		use self::CustomParseError::*;
		
		match *input.next()?
		{
			Token::Number { value, .. } =>
			{
				let constant = Self::new(value).map_err(|cssNumberConversionError| ParseError::Custom(CouldNotParseCssUnsignedNumber(cssNumberConversionError, value)))?;
				Ok(Right(CalcExpression::Number(constant)))
			}
			
			Token::Percentage { unit_value, .. } =>
			{
				let cssNumber = Self::new(unit_value).map_err(|cssNumberConversionError| ParseError::Custom(CouldNotParseCssUnsignedNumber(cssNumberConversionError, unit_value)))?;
				Ok(Left(Percentage(cssNumber)))
			}
			
			Token::Dimension { value, ref unit, .. } =>
			{
				let cssNumber = Self::Number::new(value).map_err(|cssNumberConversionError| ParseError::Custom(CouldNotParseCssSignedNumber(cssNumberConversionError, value)))?;
				
				match_ignore_ascii_case!
				{
					&*unit,
					
					"px" => Ok(Left(Constant(IsLength(Absolute(px(cssNumber)))))),
					
					"in" => Ok(Left(Constant(IsLength(Absolute(in_(cssNumber)))))),
					
					"cm" => Ok(Left(Constant(IsLength(Absolute(cm(cssNumber)))))),
					
					"mm" => Ok(Left(Constant(IsLength(Absolute(mm(cssNumber)))))),
					
					"q" => Ok(Left(Constant(IsLength(Absolute(q(cssNumber)))))),
					
					"pt" => Ok(Left(Constant(IsLength(Absolute(pt(cssNumber)))))),
					
					"pc" => Ok(Left(Constant(IsLength(Absolute(pc(cssNumber)))))),
					
					"em" => Ok(Left(Constant(IsLength(FontRelative(em(cssNumber)))))),
					
					"ex" => Ok(Left(Constant(IsLength(FontRelative(ex(cssNumber)))))),
					
					"ch" => Ok(Left(Constant(IsLength(FontRelative(ch(cssNumber)))))),
					
					"rem" => Ok(Left(Constant(IsLength(FontRelative(rem(cssNumber)))))),
					
					"vw" => if context.isNotInPageRule()
					{
						Ok(Left(Constant(IsLength(ViewportPercentage(vw(cssNumber))))))
					}
					else
					{
						Err(ParseError::Custom(CustomParseError::ViewportLengthsAreNotAllowedInAPageAtRule))
					},
					
					"vh" => if context.isNotInPageRule()
					{
						Ok(Left(Constant(IsLength(ViewportPercentage(vh(cssNumber))))))
					}
					else
					{
						Err(ParseError::Custom(CustomParseError::ViewportLengthsAreNotAllowedInAPageAtRule))
					},
					
					"vmin" => if context.isNotInPageRule()
					{
						Ok(Left(Constant(IsLength(ViewportPercentage(vmin(cssNumber))))))
					}
					else
					{
						Err(ParseError::Custom(CustomParseError::ViewportLengthsAreNotAllowedInAPageAtRule))
					},
					
					"vmax" => if context.isNotInPageRule()
					{
						Ok(Left(Constant(IsLength(ViewportPercentage(vmax(cssNumber))))))
					}
					else
					{
						Err(ParseError::Custom(CustomParseError::ViewportLengthsAreNotAllowedInAPageAtRule))
					},
					
					_ => Err(ParseError::Custom(CouldNotParseDimension(value, unit.clone()))),
				}
			}
			
			Token::ParenthesisBlock => Ok(Right(CalcExpression::parse(context, input)?)),
			
			Token::Function(ref name) =>
			{
				match name
				{
					"calc" => Ok(Left(CalcFunction(CalcExpression::parse(context, input)?))),
					
					"attr" => Ok(Left(AttrFunction(AttrExpression::parse(context, input)?))),
					
					"var" => Ok(Left(VarFunction(VarExpression::parse(context, input)?))),
					
					_ => return Err(ParseError::Custom(UnknownFunctionInValueExpression(name.to_owned())))
				}
			},
			
			unexpectedToken @ _ => Err(BasicParseError::UnexpectedToken(unexpectedToken.clone()).into()),
		}
	}
	
	#[inline(always)]
	fn to_canonical_dimension_value<Conversion: FontRelativeLengthConversion<Self::Number> + ViewportPercentageLengthConversion<Self::Number> + PercentageConversion<Self::Number>>(&self, conversion: &Conversion) -> Self::Number
	{
		use self::LengthOrPercentageUnit::*;
		
		match *self
		{
			IsLength(ref length) => length.to_canonical_dimension_value(conversion),
			IsPercentage(ref percentage) => percentage.to_canonical_dimension_value(conversion),
		}
	}
}

impl<Number: CssNumber> LengthOrPercentageUnit<Number>
{
	/// Get an absolute length using a conversion
	#[inline(always)]
	pub fn to_px<Conversion: FontRelativeLengthConversion<Number> + ViewportPercentageLengthConversion<Number> + PercentageConversion<Number>>(&self, conversion: &Conversion) -> Number
	{
		match *self
		{
			IsLength(ref length) => length.to_px(conversion),
			IsPercentage(ref percentage) => percentage.to_absolute_unit(conversion),
		}
	}
	
	/// Convert this into AppUnits.
	#[inline]
	pub fn to_app_units<Conversion: FontRelativeLengthConversion<Number> + ViewportPercentageLengthConversion<Number> + PercentageConversion<Number>>(&self, conversion: &Conversion) -> Number
	{
		self.to_px(conversion) * Number::AppUnitsPerPX
	}
}
