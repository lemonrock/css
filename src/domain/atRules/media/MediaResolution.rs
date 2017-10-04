// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A resolution.
#[derive(Clone, Debug, PartialEq)]
pub enum MediaResolution
{
	infinite,
	
	/// Dots per inch.
	dpi(CssFloat),
	
	/// Dots per pixel.
	dppx(CssFloat),
	
	/// Dots per centimetre.
	dpcm(CssFloat),
}

impl ToCss for MediaResolution
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		use self::MediaResolution::*;
		
		match *self
		{
			infinite => dest.write_str("infinite"),
			dpi(value) => serialize_dimension(value, "dpi", dest),
			dppx(value) => serialize_dimension(value, "dppx", dest),
			dpcm(value) => serialize_dimension(value, "dpcm", dest),
		}
	}
}

impl MediaResolution
{
	fn dotsPerInch(&self) -> CssFloat
	{
		use self::MediaResolution::*;
		
		match *self
		{
			infinite => ::std::f32::INFINITY,
			dpi(value) => value,
			dppx(value) => value * 96.0,
			dpcm(value) => value * 2.54,
		}
	}
	
	fn reduce(self) -> Self
	{
		use self::MediaResolution::*;
		
		match self
		{
			dpcm(dotsPerCentimetre) =>
			{
				let dotsPerInch = (dotsPerCentimetre / 2.54).round();
				let dotsPerPixel = dotsPerInch / 96.0;
				if dotsPerPixel.round() == dotsPerPixel
				{
					dppx(dotsPerPixel)
				}
				else
				{
					dpi(dotsPerInch)
				}
			},
			
			dpi(dotsPerInch) =>
			{
				let dotsPerPixel = dotsPerInch / 96.0;
				if dotsPerPixel.round() == dotsPerPixel
				{
					dppx(dotsPerPixel)
				}
				else
				{
					dpi(dotsPerInch)
				}
			}
			
			unchanged @ _ => unchanged,
		}
	}
	
	// WebKit only supports integer values of 1 and 2, but we more liberally support floating point values greater than 0
	fn parseWebKit<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		use self::MediaResolution::*;
		use ::cssparser::Token::*;
		
		match *input.next()?
		{
			Number { value, .. } =>
			{
				if value <= 0.
				{
					return Err(ParseError::Custom(CustomParseError::MediaQueryResolutionCanNotBeNegativeOrZero))
				}
				
				Ok(dppx(value))
			},
			
			ref unrecognisedToken => return Err(ParseError::Custom(CustomParseError::UnexpectedTokenWhenParsingMediaQueryResolution(unrecognisedToken.clone()))),
		}
	}
	
	fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		use self::MediaResolution::*;
		use ::cssparser::Token::*;
		
		match *input.next()?
		{
			Ident(ref identifier) =>
			{
				match_ignore_ascii_case!
				{
					&*identifier,
					
					"infinite" => Ok(infinite),
					
					_ => Err(ParseError::Custom(CustomParseError::MediaQueryResolutionDoesNotSupportThisIdentifier(identifier.to_owned())))
				}
			}
			
			Dimension { value, ref unit, .. } =>
			{
				if value <= 0.
				{
					return Err(ParseError::Custom(CustomParseError::MediaQueryResolutionCanNotBeNegativeOrZero))
				}
				
				let result = match_ignore_ascii_case!
				{
					&*unit,
					
					"dpi" => Ok(dpi(value)),
					
					"dppx" => Ok(dppx(value)),
					
					"dpcm" => Ok(dpcm(value)),
					
					_ => Err(ParseError::Custom(CustomParseError::UnrecognisedMediaQueryResolutionUnit(unit.to_owned())))
				};
				result.map(|resolution| resolution.reduce())
			},
			
			ref unrecognisedToken => return Err(ParseError::Custom(CustomParseError::UnexpectedTokenWhenParsingMediaQueryResolution(unrecognisedToken.clone()))),
		}
	}
}
