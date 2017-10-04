// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A percentage
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PercentageUnit<Number: CssNumber>(pub Number);

impl<Number: CssNumber> ToCss for PercentageUnit<Number>
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		serialize_percentage(self.0, dest)
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
		use self::CalculablePropertyValue::*;
		use self::CustomParseError::*;
		
		match *input.next()?
		{
			Token::Number { value, .. } =>
			{
				if value == 0.
				{
					Ok(Constant(Self::default()))
				}
				else
				{
					Err(ParseError::Custom(CouldNotParseDimensionLessNumber(value)))
				}
			}
			
			Token::Percentage { unit_value, .. } =>
			{
				let percentage = Self::Number::new(unit_value).map_err(|cssNumberConversionError| ParseError::Custom(CouldNotParseCssUnsignedNumber(cssNumberConversionError, unit_value)))?;
				Ok(Constant(PercentageUnit(percentage)))
			}
			
			Token::Function(ref name) =>
			{
				match_ignore_ascii_case!
				{
					&*name,
					
					"calc" => Ok(Calc(CalcFunction(Rc::new(CalcExpression::parse(context, input)?)))),
					
					"attr" => Ok(Attr(AttrFunction(Rc::new(AttrExpression::parse(context, input)?)))),
					
					"var" => Ok(Var(VarFunction(Rc::new(VarExpression::parse(context, input)?)))),
					
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
				let constant = Self::Number::new(value).map_err(|cssNumberConversionError| ParseError::Custom(CouldNotParseCssUnsignedNumber(cssNumberConversionError, value)))?;
				Ok(Right(CalcExpression::Number(constant)))
			}
			
			Token::Percentage { unit_value, .. } =>
			{
				let percentage = Self::Number::new(unit_value).map_err(|cssNumberConversionError| ParseError::Custom(CouldNotParseCssUnsignedNumber(cssNumberConversionError, unit_value)))?;
				Ok(Left(Constant(PercentageUnit(percentage))))
			}
			
			Token::ParenthesisBlock => Ok(Right(CalcExpression::parse(context, input)?)),
			
			Token::Function(ref name) =>
			{
				match_ignore_ascii_case!
				{
					&*name,
					
					"calc" => Ok(Left(Calc(CalcFunction(Rc::new(CalcExpression::parse(context, input)?))))),
					
					"attr" => Ok(Left(Attr(AttrFunction(Rc::new(AttrExpression::parse(context, input)?))))),
					
					"var" => Ok(Left(Var(VarFunction(Rc::new(VarExpression::parse(context, input)?))))),
					
					_ => return Err(ParseError::Custom(UnknownFunctionInValueExpression(name.to_owned())))
				}
			},
			
			unexpectedToken @ _ => Err(BasicParseError::UnexpectedToken(unexpectedToken.clone()).into()),
		}
	}
	
	#[inline(always)]
	fn to_canonical_dimension_value<Conversion: FontRelativeLengthConversion<Self::Number> + ViewportPercentageLengthConversion<Self::Number> + PercentageConversion<Self::Number>>(&self, conversion: &Conversion) -> Self::Number
	{
		self.to_absolute_value(conversion)
	}
	
	#[inline(always)]
	fn from_raw_css_for_var_expression_evaluation(value: &str, _is_not_in_page_rule: bool) -> Option<Self>
	{
		fn from_raw_css_for_var_expression_evaluation_internal<'i: 't, 't, Number: CssNumber>(input: &Parser<'i, 't>) -> Result<PercentageUnit<Number>, ParseError<'i, CustomParseError<'i>>>
		{
			let value = match *input.next()?
			{
				Token::Number { value, .. } =>
				{
					if value == 0.
					{
						Ok(PercentageUnit::default())
					}
					else
					{
						Err(ParseError::Custom(CouldNotParseDimensionLessNumber(value)))
					}
				}
				
				Token::Percentage { unit_value, .. } =>
				{
					let cssNumber = <PercentageUnit<Number> as Unit>::Number::new(unit_value).map_err(|cssNumberConversionError| ParseError::Custom(CouldNotParseCssSignedNumber(cssNumberConversionError, value)))?;
					Ok(PercentageUnit(cssNumber))
				}
				
				unexpectedToken @ _ => Err(BasicParseError::UnexpectedToken(unexpectedToken.clone()).into()),
			};
			
			input.skip_whitespace();
			
			input.expect_exhausted()?;
			
			value
		}
		
		const LineNumberingIsZeroBased: u32 = 0;
		
		let mut parserInput = ParserInput::new_with_line_number_offset(value, LineNumberingIsZeroBased);
		let mut input = Parser::new(&mut parserInput);
		
		from_raw_css_for_var_expression_evaluation_internal(&input).ok()
	}
}

impl<Number: CssNumber> PercentageUnit<Number>
{
	pub const ZeroPercent: PercentageUnit<Number> = PercentageUnit(Number::Zero);
	
	pub const OneHundredPercent: PercentageUnit<Number> = PercentageUnit(Number::One);
}

impl<Number: CssNumber> PercentageUnit<Number>
{
	#[inline(always)]
	pub fn to_absolute_value<Conversion: PercentageConversion<Number>>(&self, conversion: &Conversion) -> Number
	{
		self.to_CssNumber() * conversion.one_hundred_percent_in_absolute_units()
	}
}
