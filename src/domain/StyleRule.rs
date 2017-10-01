// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A style rule, with selectors and declarations.
#[derive(Debug)]
pub struct StyleRule
{
	/// The list of selectors in this rule.
	pub selectors: DeduplicatedSelectors,
	
	/// The declaration block with the properties it contains.
	pub property_declarations: PropertyDeclarations,
	
	/// The location in the sheet where it was found.
	pub source_location: SourceLocation,
}

impl ToCss for StyleRule
{
	/// https://drafts.csswg.org/cssom/#serialize-a-css-rule
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		self.selectors.to_css(dest)?;
		
		dest.write_char('{')?;
		
		self.property_declarations.to_css(dest)?;
		
		dest.write_char('}')
	}
}
