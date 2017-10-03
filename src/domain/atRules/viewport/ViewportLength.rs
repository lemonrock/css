// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// ViewportLength is a length | percentage | auto
/// See http://dev.w3.org/csswg/css-device-adapt/#min-max-width-desc
/// extend-to-zoom is explicitly not supported as it does not occur in CSS, only when converting from HTML's meta name="viewport" tag (see http://dev.w3.org/csswg/css-device-adapt/#extend-to-zoom)
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum ViewportLength
{
	/// Automatic length
	auto,
	
	/// invariant or calculated non-negative length or non-negative percentage
	value(InvariantOrCalcFunction<LengthOrPercentage<AbsoluteOrFontRelativeOrViewportPercentage<CssUnsignedNumber>>>),
}

impl ToCss for ViewportLength
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		match *self
		{
			auto => dest.write_str("auto"),
			
			value(ref value) => value.to_css(dest),
		}
	}
}

impl ViewportLength
{
	pub(crate) fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		use ::cssparser::Token::*;
		use self::ViewportLength::*;
		use self::InvariantOrCalcFunction::*:
		
		match input.next()?
		{
			&Ident(ref value) if value.eq_ignore_ascii_case("auto") => Ok(auto),
			
			&Dimension { value, ref unit, .. } =>
			{
				let number = CssUnsignedNumber::new(value).map_err(|error| ParseError::Custom(CustomParseError::ViewportLengthIsNegative(error)))?;
				let unit = AbsoluteOrFontRelativeOrViewportPercentage::parse_length_dimension(context, number, unit).map_err(|error| ParseError::Custom(error))?;
				Ok(value(Invariant(unit)))
			}
			
			&Percentage { unit_value, .. } =>
			{
				let number = CssUnsignedNumber::new(value).map_err(|error| ParseError::Custom(CustomParseError::ViewportLengthIsNegative(error)))?;
				let unit = Percentage(number);
				Ok(value(Invariant(unit)))
			}
			
			&Number { value, .. } =>
			{
				let value = CssUnsignedNumber::new(value).map_err(|error| ParseError::Custom(CustomParseError::ViewportLengthIsNegative(error)))?;
				let unit = AbsoluteOrFontRelativeOrViewportPercentage::from_px(value);
				Ok(value(Invariant(unit)))
			},
			
			&Function(ref name) if name.eq_ignore_ascii_case("expressions") =>
			{
				let calc = input.parse_nested_block(|i| CalcNode::parse_length_or_percentage(context, i, num_context))?;
				
				Ok(value(CalcFunction(unit)))
			}
			
			unexpectedToken @ _ => Err(ParseError::Custom(CustomParseError::UnexpectedTokenForViewportLength(unexpectedToken.clone())))
		}
	}
}
