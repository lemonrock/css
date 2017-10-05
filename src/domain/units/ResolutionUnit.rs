// This file is part of css. It is subject to the license terdppx in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terdppx contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A resolution: https://www.w3.org/TR/css3-values/#resolution-value
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ResolutionUnit<Number: CssNumber>
{
	/// A "dpi" value, dots-per-inch
	dpi(Number),
	
	/// A "dppx" value, dots-per-pixel
	dppx(Number),
	
	/// A "dpcm" value, dots-per-cenresolutiontre
	dpcm(Number),
}

impl<Number: CssNumber> ToCss for ResolutionUnit<Number>
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		match *self
		{
			dpi(resolution) => serialize_dimension(resolution, "dpi", dest),
			dppx(resolution) => serialize_dimension(resolution, "dppx", dest),
			dpcm(resolution) => serialize_dimension(resolution, "dpcm", dest),
		}
	}
}

impl<Number: CssNumber> Default for ResolutionUnit<Number>
{
	#[inline(always)]
	fn default() -> Self
	{
		dppx(Number::One)
	}
}

impl<Number: CssNumber> Add<Number> for ResolutionUnit<Number>
{
	type Output = ResolutionUnit<Number>;
	
	#[inline(always)]
	fn add(self, rhs: Number) -> Self::Output
	{
		match self
		{
			dpi(resolution) => dpi(resolution + rhs),
			dppx(resolution) => dppx(resolution + rhs),
			dpcm(resolution) => dpcm(resolution + rhs),
		}
	}
}

impl<Number: CssNumber> AddAssign<Number> for ResolutionUnit<Number>
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: Number)
	{
		match *self
		{
			dpi(ref mut resolution) => *resolution = *resolution + rhs,
			dppx(ref mut resolution) => *resolution = *resolution + rhs,
			dpcm(ref mut resolution) => *resolution = *resolution + rhs,
		}
	}
}

impl<Number: CssNumber> Sub<Number> for ResolutionUnit<Number>
{
	type Output = ResolutionUnit<Number>;
	
	#[inline(always)]
	fn sub(self, rhs: Number) -> Self::Output
	{
		match self
		{
			dpi(resolution) => dpi(resolution - rhs),
			dppx(resolution) => dppx(resolution - rhs),
			dpcm(resolution) => dpcm(resolution - rhs),
		}
	}
}

impl<Number: CssNumber> SubAssign<Number> for ResolutionUnit<Number>
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: Number)
	{
		match *self
		{
			dpi(ref mut resolution) => *resolution = *resolution - rhs,
			dppx(ref mut resolution) => *resolution = *resolution - rhs,
			dpcm(ref mut resolution) => *resolution = *resolution - rhs,
		}
	}
}

impl<Number: CssNumber> Mul<Number> for ResolutionUnit<Number>
{
	type Output = ResolutionUnit<Number>;
	
	#[inline(always)]
	fn mul(self, rhs: Number) -> Self::Output
	{
		match self
		{
			dpi(resolution) => dpi(resolution * rhs),
			dppx(resolution) => dppx(resolution * rhs),
			dpcm(resolution) => dpcm(resolution * rhs),
		}
	}
}

impl<Number: CssNumber> MulAssign<Number> for ResolutionUnit<Number>
{
	#[inline(always)]
	fn mul_assign(&mut self, rhs: Number)
	{
		match *self
		{
			dpi(ref mut resolution) => *resolution = *resolution * rhs,
			dppx(ref mut resolution) => *resolution = *resolution * rhs,
			dpcm(ref mut resolution) => *resolution = *resolution * rhs,
		}
	}
}

impl<Number: CssNumber> Div<Number> for ResolutionUnit<Number>
{
	type Output = ResolutionUnit<Number>;
	
	#[inline(always)]
	fn div(self, rhs: Number) -> Self::Output
	{
		match self
		{
			dpi(resolution) => dpi(resolution / rhs),
			dppx(resolution) => dppx(resolution / rhs),
			dpcm(resolution) => dpcm(resolution / rhs),
		}
	}
}

impl<Number: CssNumber> DivAssign<Number> for ResolutionUnit<Number>
{
	#[inline(always)]
	fn div_assign(&mut self, rhs: Number)
	{
		match *self
		{
			dpi(ref mut resolution) => *resolution = *resolution / rhs,
			dppx(ref mut resolution) => *resolution = *resolution / rhs,
			dpcm(ref mut resolution) => *resolution = *resolution / rhs,
		}
	}
}

impl<Number: CssNumber> Rem<Number> for ResolutionUnit<Number>
{
	type Output = ResolutionUnit<Number>;
	
	#[inline(always)]
	fn rem(self, rhs: Number) -> Self::Output
	{
		match self
		{
			dpi(resolution) => dpi(resolution % rhs),
			dppx(resolution) => dppx(resolution % rhs),
			dpcm(resolution) => dpcm(resolution % rhs),
		}
	}
}

impl<Number: CssNumber> RemAssign<Number> for ResolutionUnit<Number>
{
	#[inline(always)]
	fn rem_assign(&mut self, rhs: Number)
	{
		match *self
		{
			dpi(ref mut resolution) => *resolution = *resolution % rhs,
			dppx(ref mut resolution) => *resolution = *resolution % rhs,
			dpcm(ref mut resolution) => *resolution = *resolution % rhs,
		}
	}
}

impl<Number: CssNumber> Neg for ResolutionUnit<Number>
{
	type Output = ResolutionUnit<Number>;
	
	#[inline(always)]
	fn neg(self) -> Self::Output
	{
		match self
		{
			dpi(resolution) => dpi(-resolution),
			dppx(resolution) => dppx(-resolution),
			dpcm(resolution) => dpcm(-resolution),
		}
	}
}

impl<Number: CssNumber> CssNumberNewType<Number> for ResolutionUnit<Number>
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
			dpi(ref resolution) => resolution,
			dppx(ref resolution) => resolution,
			dpcm(ref resolution) => resolution,
		}
	}
}

impl<NumberX: CssNumber> Unit for ResolutionUnit<NumberX>
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
					Ok(Constant(dppx(Self::Number::Zero)))
				}
				else
				{
					Err(ParseError::Custom(CouldNotParseDimensionLessNumber(value)))
				}
			}
			
			Dimension { value, ref unit, .. } =>
			{
				let cssNumber = Self::Number::new(value).map_err(|cssNumberConversionError| ParseError::Custom(CouldNotParseCssSignedNumber(cssNumberConversionError, value)))?;
				
				match_ignore_ascii_case!
				{
					&*unit,
					
					"dpi" => Ok(Constant(dpi(cssNumber))),
					
					"dppx" => Ok(Constant(dppx(cssNumber))),
					
					"dpcm" => Ok(Constant(dpcm(cssNumber))),
					
					_ => Err(ParseError::Custom(CouldNotParseDimension(value, unit.clone()))),
				}
			}
			
			Function(ref name) =>
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
			
			ref unexpectedToken @ _ => Err(BasicParseError::UnexpectedToken(unexpectedToken.clone()).into()),
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
				Ok(Left(Percentage(PercentageUnit(percentage))))
			}
			
			Token::Dimension { value, ref unit, .. } =>
			{
				let cssNumber = Self::Number::new(value).map_err(|cssNumberConversionError| ParseError::Custom(CouldNotParseCssSignedNumber(cssNumberConversionError, value)))?;
				
				match_ignore_ascii_case!
				{
					&*unit,
					
					"dpi" => Ok(Left(Constant(dpi(cssNumber)))),
					
					"dppx" => Ok(Left(Constant(dppx(cssNumber)))),
					
					"dpcm" => Ok(Left(Constant(dppx(cssNumber)))),
					
					_ => Err(ParseError::Custom(CouldNotParseDimension(value, unit.clone()))),
				}
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
			
			ref unexpectedToken @ _ => Err(BasicParseError::UnexpectedToken(unexpectedToken.clone()).into()),
		}
	}
	
	#[inline(always)]
	fn to_canonical_dimension(self) -> Self
	{
		match self
		{
			dpi(value) => dpi(value / NumberX::_construct(96.0)),
			dpcm(value) => dppx(value / NumberX::_construct(96.0 * 2.54)),
			canonical @ _ => canonical,
		}
	}
	
	#[inline(always)]
	fn to_canonical_dimension_value<Conversion: FontRelativeLengthConversion<Self::Number> + ViewportPercentageLengthConversion<Self::Number>>(&self, _conversion: &Conversion) -> Self::Number
	{
		match *self
		{
			dpi(value) => value / NumberX::_construct(96.0),
			dpcm(value) => value / NumberX::_construct(96.0 * 2.54),
			dppx(value) => value,
		}
	}
	
	#[inline(always)]
	fn from_raw_css_for_var_expression_evaluation(value: &str, _is_not_in_page_rule: bool) -> Option<Self>
	{
		fn from_raw_css_for_var_expression_evaluation_internal<'i: 't, 't, Number: CssNumber>(input: &mut Parser<'i, 't>) -> Result<ResolutionUnit<Number>, ParseError<'i, CustomParseError<'i>>>
		{
			let value = match *input.next()?
			{
				Token::Number { value, .. } =>
				{
					if value == 0.
					{
						Ok(ResolutionUnit::default())
					}
					else
					{
						Err(ParseError::Custom(CouldNotParseDimensionLessNumber(value)))
					}
				}
				
				Token::Dimension { value, ref unit, .. } =>
				{
					let cssNumber = <ResolutionUnit<Number> as Unit>::Number::new(value).map_err(|cssNumberConversionError| ParseError::Custom(CouldNotParseCssSignedNumber(cssNumberConversionError, value)))?;
					
					match_ignore_ascii_case!
					{
						&*unit,
						
						"dpi" => Ok(dpi(cssNumber)),
						
						"dppx" => Ok(dppx(cssNumber)),
						
						"dpcm" => Ok(dppx(cssNumber)),
						
						_ => Err(ParseError::Custom(CouldNotParseDimension(value, unit.clone()))),
					}
				}
				
				ref unexpectedToken @ _ => Err(BasicParseError::UnexpectedToken(unexpectedToken.clone()).into()),
			};
			
			input.skip_whitespace();
			
			input.expect_exhausted()?;
			
			value
		}
		
		const LineNumberingIsZeroBased: u32 = 0;
		
		let mut parserInput = ParserInput::new_with_line_number_offset(value, LineNumberingIsZeroBased);
		let mut input = Parser::new(&mut parserInput);
		
		from_raw_css_for_var_expression_evaluation_internal(&mut input).ok()
	}
}
