// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// ViewportLength is a length | percentage | auto | extend-to-zoom
/// See:
/// * http://dev.w3.org/csswg/css-device-adapt/#min-max-width-desc
/// * http://dev.w3.org/csswg/css-device-adapt/#extend-to-zoom
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum ViewportLength
{
	Specified(LengthOrPercentageOrAuto),
	ExtendToZoom
}

use self::ViewportLength::*;

impl ToCss for ViewportLength
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		match *self
		{
			Specified(ref length) => length.to_css(dest),
			ExtendToZoom => dest.write_str("extend-to-zoom"),
		}
	}
}

impl FromMeta for ViewportLength
{
	fn from_meta(value: &str) -> Option<ViewportLength>
	{
		macro_rules! specified
		{
            ($value:expr) =>
            {
                Specified(LengthOrPercentageOrAuto::Length($value))
            }
        }
		
		Some
		(
			match value
			{
				deviceWidth if deviceWidth.eq_ignore_ascii_case("device-width") => specified!(NoCalcLength::ViewportPercentage(ViewportPercentageLength::Vw(100.))),
				deviceHeight if deviceHeight.eq_ignore_ascii_case("device-height") => specified!(NoCalcLength::ViewportPercentage(ViewportPercentageLength::Vh(100.))),
				_ =>
				{
					match value.parse::<f32>()
					{
						Ok(n) if n >= 0. => specified!(NoCalcLength::from_px(n.max(1.).min(10000.))),
						Ok(_) => return None,
						Err(_) => specified!(NoCalcLength::from_px(1.))
					}
				}
			
		})
	}
}

impl ViewportLength
{
	fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>>
	{
		// we explicitly do not accept 'extend-to-zoom', since it is a UA internal value for <META> viewport translation
		LengthOrPercentageOrAuto::parse_non_negative(context, input).map(Specified)
	}
}
