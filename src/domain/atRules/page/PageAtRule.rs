// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A [`@page`][page] rule.
///
/// This implements only a limited subset of the CSS 2.2 syntax.
///
/// In this subset, [page selectors][page-selectors] are not implemented.
///
/// [page]: https://drafts.csswg.org/css2/page.html#page-box
/// [page-selectors]: https://drafts.csswg.org/css2/page.html#page-selectors
#[derive(Debug)]
pub struct PageAtRule
{
	/// The declaration block this page rule contains.
	pub property_declarations: PropertyDeclarations,
	
	/// The source position this rule was found at.
	pub source_location: SourceLocation,
}

impl ToCss for PageAtRule
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		dest.write_str("@page { ")?;
		self.declarations.to_css(dest)?;
		if !self.declarations.is_empty()
		{
			dest.write_str(" ")?;
		}
		dest.write_str("}")
	}
}
