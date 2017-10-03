// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


css_number!(CssSignedNumber);

impl Unit for CssSignedNumber
{
	type Number = Self;

	#[inline(always)]
	fn parse_one<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<CalcExpression, ParseError<'i, CustomParseError<'i>>>
	{
		use ::cssparser::Token::*;
		
		match input.next()?
		{
			&Number { value, .. } => Self::new(value).map_err(|cssNumberConversionError| ParseError::Custom(CustomParseError::CouldNotParseCssSignedNumber(cssNumberConversionError, value))).map(CalcExpression::Constant),
			
			&Percentage { unit_value, .. } => CssUnsignedNumber::new(unit_value).map_err(|cssNumberConversionError| ParseError::Custom(CustomParseError::CouldNotParseCssSignedNumber(cssNumberConversionError, value))).map(CalcExpression::Percentage),
			
			ParenthesisBlock => Ok(CalcExpression::Parentheses(CalcExpression::parse(context, input)?)),
			
			Function(ref name) =>
			{
				match name
				{
					"expressions" => Ok(CalcExpression::NestedCalcFunction(CalcExpression::parse(context, input)?)),
					
					"attr" => Ok(CalcExpression::NestedAttrFunction(AttrExpression::parse(context, input)?)),
					
					"var" => Ok(CalcExpression::NestedVarFunction(VarExpression::parse(context, input)?)),
					
					_ => return Err(ParseError::Custom(CustomParseError::UnknownFunctionInValueExpression(name.to_owned())))
				}
				
				
			},
			
			unexpectedToken @ _ => Err(BasicParseError::UnexpectedToken(unexpectedToken.clone()).into()),
		}
	}
	
	#[inline(always)]
	fn to_canonical_dimension(self) -> Self
	{
		self
	}
}

impl CssSignedNumber
{
	#[inline(always)]
	fn parseNested<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<SimpleOrComplex, ParseError<'i, CustomParseError<'i>>>
	{
		let simpleOrComplex = CalcExpression::parseCalcExpression(context, input)?;
		if simpleOrComplex.is_simple()
		{
			CalcExpression::Invariant(simpleOrComplex.to_simple())
		}
		else
		{
			CalcExpression::Parentheses(simpleOrComplex.to_complex())
		}
		
		// CalcExpression::Parentheses
		// input.parse_nested_block(|i| CalcNode::parse(context, i, expected_unit))
	}
}
