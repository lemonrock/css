// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AttrExpression<U>
{
	pub attribute_lower_case_name: String,
	
	pub type_or_unit: TypeOrUnit,

	pub default_value: Option<U>,
}

impl<U: Unit> ToCss for AttrExpression<U>
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		serialize_identifier(&self.attribute, dest)?;
		
		if self.type_or_unit != TypeOrUnit::string
		{
			dest.write_char(' ')?;
			self.type_or_unit.to_css(dest)?;
		}
		
		if let Some(ref default_value) = self.default_value
		{
			dest.write_char(',')?;
			default_value.to_css(dest)?;
		}
		
		Ok(())
	}
}

impl<U: Unit> Expression<U> for AttrExpression<U>
{
	/// Evaluate the calc() expression, returning the numeric value of the canonical dimension
	/// Division by zero is handled by returning the maximum possible f32 value
	/// Subtractions for UnsignedCssNumber that are negative are handled by returning 0.0
	#[inline(always)]
	fn evaluate<Conversion: FontRelativeLengthConversion<U::Number> + ViewportPercentageLengthConversion<U::Number> + PercentageConversion<U::Number> + AttributeConversion<U::Number> + CssVariableConversion<U::Number>>(&self, conversion: &Conversion) -> Option<U::Number>
	{
		if let Some(value) = conversion.attributeValue(self.attribute_lower_case_name)
		{
			Some(value)
		}
		else
		{
			self.default_value
		}
	}
}

impl<U: Unit> AttrExpression<U>
{
	#[inline(always)]
	fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		input.parse_nested_block(|input| Self::parse_sum(context, input))
	}
}
