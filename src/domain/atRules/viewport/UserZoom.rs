// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


define_css_keyword_enum!(UserZoom:
                         "zoom" => Zoom,
                         "fixed" => Fixed);

impl FromMeta for UserZoom
{
	fn from_meta(value: &str) -> Option<Self>
	{
		use self::UserZoom::*;
		
		Some
		(
			match value
			{
				yes if yes.eq_ignore_ascii_case("yes") => Zoom,
				
				no if no.eq_ignore_ascii_case("no") => Fixed,
				
				deviceWidth if deviceWidth.eq_ignore_ascii_case("device-width") => Zoom,
				
				deviceHeight if deviceHeight.eq_ignore_ascii_case("device-height") => Zoom,
				
				_ => match value.parse::<f32>()
				{
					Ok(n) if n >= 1. || n <= -1. => Zoom,
					_ => Fixed
				}
			}
		)
	}
}
