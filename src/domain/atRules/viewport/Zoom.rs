// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// https://drafts.csswg.org/css-device-adapt/#descdef-viewport-zoom
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Zoom
{
    /// A number value.
    Number(f32),
	
    /// A percentage value.
    Percentage(f32),
	
    /// The `auto` keyword.
    Auto,
}

impl ToCss for Zoom
{
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
    {
		use self::Zoom::*;
		
        match *self
		{
            Number(number) => number.to_css(dest),
			
            Auto => dest.write_str("auto"),
			
            Percentage(percentage) =>
			{
                (percentage * 100.).to_css(dest)?;
                dest.write_char('%')
            }
        }
    }
}

impl Zoom
{
    /// Parse a zoom value per:
    ///
    /// https://drafts.csswg.org/css-device-adapt/#descdef-viewport-zoom
    pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>>
	{
		use self::Zoom::*;
		use AllowedNumericType::NonNegative;
		
        match *input.next()?
		{
            // TODO: This parse() method should take ParserContext as an argument, and pass ParsingMode owned by the ParserContext to is_ok() instead of using PARSING_MODE_DEFAULT directly.  In order to do so, we might want to move this code into style::stylesheets::viewport_rule.
            Token::Percentage { unit_value, .. } if NonNegative.is_ok(ParsingMode::PARSING_MODE_DEFAULT, unit_value) => Ok(Percentage(unit_value)),
			
            Token::Number { value, .. } if NonNegative.is_ok(ParsingMode::PARSING_MODE_DEFAULT, value) => Ok(Number(value)),
			
            Token::Ident(ref value) if value.eq_ignore_ascii_case("auto") => Ok(Auto),
			
            ref unexpectedToken => Err(ParseError::Custom(CustomParseError::UnexpectedTokenWhenParsingZoom(unexpectedToken.clone())))
        }
    }

    /// Get this zoom value as a float value. Returns `None` if the value is the `auto` keyword.
    #[inline]
    pub fn to_f32(&self) -> Option<f32>
	{
		use self::Zoom::*;
		
        match *self
		{
            Number(number) => Some(number as f32),
            Percentage(percentage) => Some(percentage as f32),
            Auto => None
        }
    }
}
impl FromMeta for Zoom
{
	fn from_meta(value: &str) -> Option<Self>
	{
		use self::Zoom::Number;
		
		Some
		(
			match value
			{
				yes if yes.eq_ignore_ascii_case("yes") => Number(1.),
				no if no.eq_ignore_ascii_case("no") => Number(0.1),
				deviceWidth if deviceWidth.eq_ignore_ascii_case("device-width") => Number(10.),
				deviceHeight if deviceHeight.eq_ignore_ascii_case("device-height") => Number(10.),
				_ =>
				{
					match value.parse::<f32>()
					{
						Ok(n) if n >= 0. => Number(n.max(0.1).min(10.)),
						Ok(_) => return None,
						Err(_) => Number(0.1),
					}
				}
			}
		)
	}
}
