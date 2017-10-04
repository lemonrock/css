// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A percentage
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PercentageUnit<Number: CssNumber>(pub Number);

impl<Number: CssNumber> ToCss for PercentageUnit<Number>
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		serialize_percentage(self.0.to_f32(), dest)
	}
}

impl<Number: CssNumber> Default for PercentageUnit<Number>
{
	#[inline(always)]
	fn default() -> Self
	{
		PercentageUnit(Number::default())
	}
}

impl<Number: CssNumber> Add<Number> for PercentageUnit<Number>
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: Number) -> Self::Output
	{
		PercentageUnit(self.0 + rhs)
	}
}

impl<Number: CssNumber> AddAssign<Number> for PercentageUnit<Number>
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: Number)
	{
		*self =	PercentageUnit(self.0 + rhs);
	}
}

impl<Number: CssNumber> Sub<Number> for PercentageUnit<Number>
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: Number) -> Self::Output
	{
		PercentageUnit(self.0 - rhs)
	}
}

impl<Number: CssNumber> SubAssign<Number> for PercentageUnit<Number>
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: Number)
	{
		*self =	PercentageUnit(self.0 - rhs);
	}
}

impl<Number: CssNumber> Mul<Number> for PercentageUnit<Number>
{
	type Output = Self;
	
	#[inline(always)]
	fn mul(self, rhs: Number) -> Self::Output
	{
		PercentageUnit(self.0 * rhs)
	}
}

impl<Number: CssNumber> MulAssign<Number> for PercentageUnit<Number>
{
	#[inline(always)]
	fn mul_assign(&mut self, rhs: Number)
	{
		*self =	PercentageUnit(self.0 * rhs);
	}
}

impl<Number: CssNumber> Div<Number> for PercentageUnit<Number>
{
	type Output = PercentageUnit<Number>;
	
	#[inline(always)]
	fn div(self, rhs: Number) -> Self::Output
	{
		PercentageUnit(self.0 / rhs)
	}
}

impl<Number: CssNumber> DivAssign<Number> for PercentageUnit<Number>
{
	#[inline(always)]
	fn div_assign(&mut self, rhs: Number)
	{
		*self =	PercentageUnit(self.0 / rhs);
	}
}

impl<Number: CssNumber> Rem<Number> for PercentageUnit<Number>
{
	type Output = PercentageUnit<Number>;
	
	#[inline(always)]
	fn rem(self, rhs: Number) -> Self::Output
	{
		PercentageUnit(self.0 % rhs)
	}
}

impl<Number: CssNumber> RemAssign<Number> for PercentageUnit<Number>
{
	#[inline(always)]
	fn rem_assign(&mut self, rhs: Number)
	{
		*self =	PercentageUnit(self.0 % rhs);
	}
}

impl<Number: CssNumber> Neg for PercentageUnit<Number>
{
	type Output = PercentageUnit<Number>;
	
	#[inline(always)]
	fn neg(self) -> Self::Output
	{
		PercentageUnit(-self.0)
	}
}

impl<Number: CssNumber> CssNumberNewType<Number> for PercentageUnit<Number>
{
	#[inline(always)]
	fn to_f32(&self) -> f32
	{
		self.to_CssNumber().to_f32()
	}
	
	#[inline(always)]
	fn as_CssNumber(&self) -> &Number
	{
		self.0.as_CssNumber()
	}
}

impl<NumberX: CssNumber> Unit for PercentageUnit<NumberX>
{
	type Number = NumberX;
	
	const HasDimension: bool = true;
	
	#[inline(always)]
	fn parse_one_outside_calc_function<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<CalculablePropertyValue<Self>, ParseError<'i, CustomParseError<'i>>>
	{
		use ::cssparser::Token::*;
		use self::CalculablePropertyValue::*;
		use self::CustomParseError::*;
		
		match *input.next()?
		{
			Number { value, .. } =>
			{
				if value == 0.
				{
					Ok(Constant(PercentageUnit(Self::Number::Zero)))
				}
				else
				{
					Err(ParseError::Custom(CouldNotParseDimensionLessNumber(value)))
				}
			}
			
			PercentageUnit { unit_value, .. } =>
			{
				let percentage = Self::new(unit_value).map_err(|cssNumberConversionError| ParseError::Custom(CouldNotParseCssUnsignedNumber(cssNumberConversionError, unit_value)))?;
				Ok(Constant(PercentageUnit(percentage)))
			}
			
			Function(ref name) =>
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
		use ::cssparser::Token::*;
		use self::CalculablePropertyValue::*;
		use self::CustomParseError::*;
		
		match *input.next()?
		{
			Number { value, .. } =>
			{
				let constant = Self::new(value).map_err(|cssNumberConversionError| ParseError::Custom(CouldNotParseCssUnsignedNumber(cssNumberConversionError, value)))?;
				Ok(Right(CalcExpression::Number(constant)))
			}
			
			PercentageUnit { unit_value, .. } =>
			{
				let percentage = Self::new(unit_value).map_err(|cssNumberConversionError| ParseError::Custom(CouldNotParseCssUnsignedNumber(cssNumberConversionError, unit_value)))?;
				Ok(Left(PercentageUnit(percentage)))
			}
			
			ParenthesisBlock => Ok(Right(CalcExpression::parse(context, input)?)),
			
			Function(ref name) =>
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
		self.as_CssNumber() * conversion.one_hundred_percent_in_absolute_units()
	}
}

impl<Number: CssNumber> PercentageUnit<Number>
{
	pub const ZeroPercent: PercentageUnit<Number> = PercentageUnit(Number::Zero);
	
	pub const OneHundredPercent: PercentageUnit<Number> = PercentageUnit(Number::One);
}
