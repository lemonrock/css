// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// Parses a CSS selector
#[inline(always)]
pub fn parse_css_selector<'i>(selector_css: &'i str) -> Result<OurSelector, ParseError<'i, CustomParseError<'i>>>
{
	OurSelectorImpl::parse_selector(selector_css)
}
