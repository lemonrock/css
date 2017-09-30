// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A node inside a `Calc` expression's AST.
#[derive(Clone, Debug)]
pub enum CalcNode
{
	/// `<length>`
	Length(NoCalcLength),
	
	/// `<angle>`
	Angle(Angle),
	
	/// `<time>`
	Time(Time),
	
	/// `<percentage>`
	Percentage(CssFloat),
	
	/// `<number>`
	Number(CssFloat),
	
	/// An expression of the form `x + y`
	Sum(Box<CalcNode>, Box<CalcNode>),
	
	/// An expression of the form `x - y`
	Sub(Box<CalcNode>, Box<CalcNode>),
	
	/// An expression of the form `x * y`
	Mul(Box<CalcNode>, Box<CalcNode>),
	
	/// An expression of the form `x / y`
	Div(Box<CalcNode>, Box<CalcNode>),
}

impl CalcNode
{
	/// Convenience parsing function for integers.
	pub(crate) fn parse_integer<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<CssInteger, ParseError<'i, CustomParseError<'i>>>
	{
		Self::parse(context, input, CalcUnit::Integer)?.to_number().map(|n| n as CssInteger)	.map_err(|()| ParseError::Custom(CustomParseError::CouldNotParseInteger))
	}
	
	/// Convenience parsing function for `<length> | <percentage>`.
	pub(crate) fn parse_length_or_percentage<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>, clamping_mode: AllowedNumericType) -> Result<CalcLengthOrPercentage, ParseError<'i, CustomParseError<'i>>>
	{
		Self::parse(context, input, CalcUnit::LengthOrPercentage)?.to_length_or_percentage(clamping_mode).map_err(|()| ParseError::Custom(CustomParseError::CouldNotParseLengthDimensionOrPercentage))
	}
	
	/// Convenience parsing function for percentages.
	pub(crate) fn parse_percentage<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<CssFloat, ParseError<'i, CustomParseError<'i>>>
	{
		Self::parse(context, input, CalcUnit::Percentage)?.to_percentage().map_err(|()| ParseError::Custom(CustomParseError::CouldNotParsePercentage).into())
	}
	
	/// Convenience parsing function for `<length>`.
	pub(crate) fn parse_length<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>, clamping_mode: AllowedNumericType) -> Result<CalcLengthOrPercentage, ParseError<'i, CustomParseError<'i>>>
	{
		Self::parse(context, input, CalcUnit::Length)?.to_length_or_percentage(clamping_mode).map_err(|()| ParseError::Custom(CustomParseError::CouldNotParseLengthDimension))
	}
	
	/// Convenience parsing function for `<number>`.
	pub(crate) fn parse_number<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<CssFloat, ParseError<'i, CustomParseError<'i>>>
	{
		Self::parse(context, input, CalcUnit::Number)?.to_number().map_err(|()| ParseError::Custom(CustomParseError::CouldNotParseNumber))
	}
	
	/// Convenience parsing function for `<angle>`.
	pub(crate) fn parse_angle<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Angle, ParseError<'i, CustomParseError<'i>>>
	{
		Self::parse(context, input, CalcUnit::Angle)?.to_angle().map_err(|()| ParseError::Custom(CustomParseError::CouldNotParseAngleDimension))
	}
	
	/// Convenience parsing function for `<time>`.
	pub(crate) fn parse_time<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Time, ParseError<'i, CustomParseError<'i>>>
	{
		Self::parse(context, input, CalcUnit::Time)?.to_time().map_err(|()| ParseError::Custom(CustomParseError::CouldNotParseTimeDimension))
	}
	
	/// Parse a top-level `calc` expression, with all nested sub-expressions.
	///
	/// This is in charge of parsing, for example, `2 + 3 * 100%`.
	fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>, expected_unit: CalcUnit) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		use self::CalcNode::*;
		
		let mut root = Self::parse_product(context, input, expected_unit)?;
		
		loop
		{
			let start = input.state();
			match input.next_including_whitespace()
			{
				Ok(&Token::WhiteSpace(_)) =>
				{
					if input.is_exhausted()
					{
						// allow trailing whitespace
						break;
					}
					
					match input.next()?.clone()
					{
						Token::Delim('+') =>
						{
							let rhs = Self::parse_product(context, input, expected_unit)?;
							let new_root = Sum(Box::new(root), Box::new(rhs));
							root = new_root;
						}
						
						Token::Delim('-') =>
						{
							let rhs = Self::parse_product(context, input, expected_unit)?;
							let new_root = Sub(Box::new(root), Box::new(rhs));
							root = new_root;
						}
						
						unexpectedToken => return Err(BasicParseError::UnexpectedToken(unexpectedToken).into()),
					}
				}
				_ =>
				{
					input.reset(&start);
					break
				}
			}
		}
		
		Ok(root)
	}
	
	/// Parse a top-level `calc` expression, and all the products that may follow, and stop as soon as a non-product expression is found.
	///
	/// This should parse correctly:-
	///
	/// * `2`
	/// * `2 * 2`
	/// * `2 * 2 + 2` (but will leave the `+ 2` unparsed).
	///
	fn parse_product<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>, expected_unit: CalcUnit) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		use self::CalcNode::*;
		
		let mut root = Self::parse_one(context, input, expected_unit)?;
		
		loop
		{
			let start = input.state();
			match input.next()
			{
				Ok(&Token::Delim('*')) =>
				{
					let rhs = Self::parse_one(context, input, expected_unit)?;
					let new_root = Mul(Box::new(root), Box::new(rhs));
					root = new_root;
				}
				
				// TODO(emilio): Figure out why the `Integer` check.
				Ok(&Token::Delim('/')) if expected_unit != CalcUnit::Integer =>
				{
					let rhs = Self::parse_one(context, input, expected_unit)?;
					let new_root = Div(Box::new(root), Box::new(rhs));
					root = new_root;
				}
				
				_ =>
				{
					input.reset(&start);
					break
				}
			}
		}
		
		Ok(root)
	}
	
	/// Tries to parse a single element in the expression, that is, a `<length>`, `<angle>`, `<time>`, `<percentage>`, according to an `expected_unit`.
	///
	/// May return a "complex" `CalcNode`, in the presence of a parenthesized expression, for example.
	fn parse_one<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>, expected_unit: CalcUnit) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		const IsFromCalc: bool = true;
		
		match (input.next()?, expected_unit)
		{
			(&Token::Number { value, .. }, _) => return Ok(CalcNode::Number(value)),
			(&Token::Dimension { value, ref unit, .. }, CalcUnit::Length) |	(&Token::Dimension { value, ref unit, .. }, CalcUnit::LengthOrPercentage) => return NoCalcLength::parse_dimension(context, value, unit).map(CalcNode::Length).map_err(|()| CustomParseError::CouldNotParseLengthDimensionInCalcFunction),
			(&Token::Dimension { value, ref unit, .. }, CalcUnit::Angle) => return Angle::parse_dimension(value, unit, IsFromCalc).map(CalcNode::Angle).map_err(|()| ParseError::Custom(CustomParseError::CouldNotParseAngleDimensionInCalcFunction)),
			(&Token::Dimension { value, ref unit, .. }, CalcUnit::Time) => return Time::parse_dimension(value, unit, IsFromCalc).map(CalcNode::Time).map_err(|()| ParseError::Custom(CustomParseError::CouldNotParseTimeDimensionInCalcFunction)),
			(&Token::Percentage { unit_value, .. }, CalcUnit::LengthOrPercentage) |	(&Token::Percentage { unit_value, .. }, CalcUnit::Percentage) => return Ok(CalcNode::Percentage(unit_value)),
			(&Token::ParenthesisBlock, _) => {}
			(&Token::Function(ref name), _) if name.eq_ignore_ascii_case("calc") => {}
			(unexpectedToken, _) => return Err(BasicParseError::UnexpectedToken(unexpectedToken.clone()).into())
		}
		input.parse_nested_block(|i| CalcNode::parse(context, i, expected_unit))
	}
	
	/// Tries to simplify this expression into a `<length>` or `<percentage`> value.
	fn to_length_or_percentage(&self, clamping_mode: AllowedNumericType) -> Result<CalcLengthOrPercentage, ()>
	{
		let mut result = CalcLengthOrPercentage
		{
			clamping_mode,
			.. Default::default()
		};
		self.add_length_or_percentage_to(&mut result, 1.0)?;
		Ok(result)
	}
	
	/// Tries to simplify this expression into a `<percentage>` value.
	fn to_percentage(&self) -> Result<CssFloat, ()>
	{
		use self::CalcNode::*;
		
		Ok
		(
			match *self
			{
				Percentage(percentage) => percentage,
				
				Sub(ref a, ref b) =>
				{
					a.to_percentage()? - b.to_percentage()?
				}
				
				Sum(ref a, ref b) =>
				{
					a.to_percentage()? + b.to_percentage()?
				}
				
				Mul(ref a, ref b) =>
				{
					match a.to_percentage()
					{
						Ok(lhs) =>
						{
							let rhs = b.to_number()?;
							lhs * rhs
						}
						Err(..) =>
						{
							let lhs = a.to_number()?;
							let rhs = b.to_percentage()?;
							lhs * rhs
						}
					}
				}
				
				Div(ref a, ref b) =>
				{
					let lhs = a.to_percentage()?;
					let rhs = b.to_number()?;
					if rhs == 0.
					{
						return Err(())
					}
					lhs / rhs
				}
				
				Number(..) | Length(..) | Angle(..) | Time(..) => return Err(()),
			}
		)
	}
	
	/// Puts this `<length>` or `<percentage>` into `result`, or error.
	///
	/// `factor` is the sign or multiplicative factor to account for the sign (this allows adding and subtracting into the return value).
	fn add_length_or_percentage_to(&self, result: &mut CalcLengthOrPercentage, factor: CssFloat) -> Result<(), ()>
	{
		use self::FontRelativeLength::*;
		use self::ViewportPercentageLength::*;
		
		match *self
		{
			CalcNode::Percentage(percentage) =>
			{
				result.percentage = Some(Percentage(result.percentage.map_or(0., |p| p.0) + percentage * factor));
			}
			
			CalcNode::Length(ref length) =>
			{
				match *length
				{
					NoCalcLength::Absolute(absoluteLength) =>
					{
						result.absolute = Some
						(
							match result.absolute
							{
								Some(value) => value + absoluteLength * factor,
								None => absoluteLength * factor,
							}
						);
					}
					
					NoCalcLength::FontRelative(relativeLength) =>
					{
						match relativeLength
						{
							Em(em) =>
							{
								result.em = Some(result.em.unwrap_or(0.) + em * factor);
							}
							
							Ex(ex) =>
							{
								result.ex = Some(result.em.unwrap_or(0.) + ex * factor);
							}
							
							Ch(ch) =>
							{
								result.ch = Some(result.ch.unwrap_or(0.) + ch * factor);
							}
							
							Rem(rem) =>
							{
								result.rem = Some(result.rem.unwrap_or(0.) + rem * factor);
							}
						}
					}
					
					NoCalcLength::ViewportPercentage(relativeLength) =>
					{
						match relativeLength
						{
							Vh(vh) =>
							{
								result.vh = Some(result.vh.unwrap_or(0.) + vh * factor)
							}
							
							Vw(vw) =>
							{
								result.vw = Some(result.vw.unwrap_or(0.) + vw * factor)
							}
							
							Vmax(vmax) =>
							{
								result.vmax = Some(result.vmax.unwrap_or(0.) + vmax * factor)
							}
							
							Vmin(vmin) =>
							{
								result.vmin = Some(result.vmin.unwrap_or(0.) + vmin * factor)
							}
						}
					}
				}
			}
			
			CalcNode::Sub(ref a, ref b) =>
			{
				a.add_length_or_percentage_to(result, factor)?;
				b.add_length_or_percentage_to(result, factor * -1.0)?;
			}
			
			CalcNode::Sum(ref a, ref b) =>
			{
				a.add_length_or_percentage_to(result, factor)?;
				b.add_length_or_percentage_to(result, factor)?;
			}
			
			CalcNode::Mul(ref a, ref b) =>
			{
				match b.to_number()
				{
					Ok(rhs) =>
					{
						a.add_length_or_percentage_to(result, factor * rhs)?;
					}
					Err(..) =>
					{
						let lhs = a.to_number()?;
						b.add_length_or_percentage_to(result, factor * lhs)?;
					}
				}
			}
			
			CalcNode::Div(ref a, ref b) =>
			{
				let new_factor = b.to_number()?;
				if new_factor == 0.
				{
					return Err(());
				}
				a.add_length_or_percentage_to(result, factor / new_factor)?;
			}
			
			CalcNode::Angle(..) | CalcNode::Time(..) | CalcNode::Number(..) => return Err(()),
		}
		
		Ok(())
	}
	
	/// Tries to simplify this expression into a `<time>` value.
	fn to_time(&self) -> Result<Time, ()>
	{
		Ok
		(
			match *self
			{
				CalcNode::Time(ref time) => time.clone(),
				
				CalcNode::Sub(ref a, ref b) =>
				{
					let lhs = a.to_time()?;
					let rhs = b.to_time()?;
					Time::from_calc(lhs.seconds() - rhs.seconds())
				}
				
				CalcNode::Sum(ref a, ref b) =>
				{
					let lhs = a.to_time()?;
					let rhs = b.to_time()?;
					Time::from_calc(lhs.seconds() + rhs.seconds())
				}
				
				CalcNode::Mul(ref a, ref b) =>
				{
					match b.to_number()
					{
						Ok(rhs) =>
						{
							let lhs = a.to_time()?;
							Time::from_calc(lhs.seconds() * rhs)
						}
						
						Err(()) =>
						{
							let lhs = a.to_number()?;
							let rhs = b.to_time()?;
							Time::from_calc(lhs * rhs.seconds())
						}
					}
				}
				
				CalcNode::Div(ref a, ref b) =>
				{
					let lhs = a.to_time()?;
					let rhs = b.to_number()?;
					if rhs == 0.
					{
						return Err(())
					}
					Time::from_calc(lhs.seconds() / rhs)
				}
				
				CalcNode::Number(..) | CalcNode::Length(..) | CalcNode::Percentage(..) | CalcNode::Angle(..) => return Err(()),
			}
		)
	}
	
	/// Tries to simplify this expression into an `Angle` value.
	fn to_angle(&self) -> Result<Angle, ()>
	{
		Ok
		(
			match *self
			{
				CalcNode::Angle(ref angle) => angle.clone(),
				
				CalcNode::Sub(ref a, ref b) =>
				{
					let lhs = a.to_angle()?;
					let rhs = b.to_angle()?;
					Angle::from_calc(lhs.radians() - rhs.radians())
				}
				
				CalcNode::Sum(ref a, ref b) =>
				{
					let lhs = a.to_angle()?;
					let rhs = b.to_angle()?;
					Angle::from_calc(lhs.radians() + rhs.radians())
				}
				
				CalcNode::Mul(ref a, ref b) =>
				{
					match a.to_angle()
					{
						Ok(lhs) =>
						{
							let rhs = b.to_number()?;
							Angle::from_calc(lhs.radians() * rhs)
						}
						Err(..) =>
						{
							let lhs = a.to_number()?;
							let rhs = b.to_angle()?;
							Angle::from_calc(lhs * rhs.radians())
						}
					}
				}
				
				CalcNode::Div(ref a, ref b) =>
				{
					let lhs = a.to_angle()?;
					let rhs = b.to_number()?;
					if rhs == 0.
					{
						return Err(())
					}
					Angle::from_calc(lhs.radians() / rhs)
				}
				
				CalcNode::Number(..) | CalcNode::Length(..) | CalcNode::Percentage(..) | CalcNode::Time(..) => return Err(()),
			}
		)
	}
	
	/// Tries to simplify this expression into a `<number>` value.
	fn to_number(&self) -> Result<CssFloat, ()>
	{
		Ok
		(
			match *self
			{
				CalcNode::Number(n) => n,
				
				CalcNode::Sum(ref a, ref b) =>
				{
					a.to_number()? + b.to_number()?
				}
				
				CalcNode::Sub(ref a, ref b) =>
				{
					a.to_number()? - b.to_number()?
				}
				
				CalcNode::Mul(ref a, ref b) =>
				{
					a.to_number()? * b.to_number()?
				}
				
				CalcNode::Div(ref a, ref b) =>
				{
					let lhs = a.to_number()?;
					let rhs = b.to_number()?;
					if rhs == 0.
					{
						return Err(())
					}
					lhs / rhs
				}
				
				CalcNode::Length(..) | CalcNode::Percentage(..) | CalcNode::Angle(..) | CalcNode::Time(..) => return Err(()),
			}
		)
	}
}
