// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VarExpression
{
	pub custom_property_lower_case_name_without_double_dash: String,

	pub default_value_css: String,

	pub is_not_in_page_rule: bool,
}

impl ToCss for VarExpression
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		dest.write_str("--")?;
		serialize_identifier(&self.attribute, dest)?;
		
		if let Some(ref default_value) = self.default_value_css
		{
			dest.write_char(',')?;
			dest.write_str(self.default_value_css)
		}
		
		Ok(())
	}
}

impl<U: Unit> Expression<U> for VarExpression
{
	/// Division by zero is handled by returning the maximum possible f32 value
	/// Subtractions for UnsignedCssNumber that are negative are handled by returning 0.0
	#[inline(always)]
	fn evaluate<Conversion: FontRelativeLengthConversion<U::Number> + ViewportPercentageLengthConversion<U::Number> + PercentageConversion<U::Number> + AttributeConversion<U> + CssVariableConversion>(&self, conversion: &Conversion) -> Option<U::Number>
	{
		match conversion.cssVariableValue(self.custom_property_lower_case_name_without_double_dash)
		{
			Some(value_css) => U::from_raw_css_for_var_expression_evaluation(value_css, self.is_not_in_page_rule),
			None => U::from_raw_css_for_var_expression_evaluation(&self.default_value_css, self.is_not_in_page_rule),
		}
	}
}

impl VarExpression
{
	#[inline(always)]
	pub(crate) fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		input.parse_nested_block(|input|
		{
			let identifier = input.expect_ident()?;
			if !identifier.starts_with("--")
			{
				return Err(ParseError::Custom(CustomParseError::CssVariablesInVarExpressionsMustStartWithTwoDashes(identifier)))
			}
			let mut custom_property_lower_case_name_without_double_dash = (&identifier[2 .. ]).to_owned();
			custom_property_lower_case_name_without_double_dash.make_ascii_lowercase();
			
			let startPosition = input.position();
			
			let result = input.try(|input|
			{
				input.expectComma()?;
				input.skip_whitespace()?;
				
				if input.is_exhausted()
				{
					Ok(None)
				}
				else
				{
					input.parse_entirely(|input| input.slice_from(startPosition).map(|slice| Some(slice.to_owned())))
				}
			});
			
			let default_value_css = if let Ok(Some(default_value_css)) = result
			{
				if default_value_css.is_empty()
				{
					None
				}
				else
				{
					Some(default_value_css)
				}
			}
			else
			{
				None
			};
			
			Ok
			(
				Self
				{
					custom_property_lower_case_name_without_double_dash,
					default_value_css,
					is_not_in_page_rule: context.isNotInPageRule(),
				}
			)
		})
	}
}
