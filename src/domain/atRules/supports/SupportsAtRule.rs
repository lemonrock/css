// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// An [`@supports`][supports] rule.
///
/// [supports]: https://drafts.csswg.org/css-conditional-3/#at-supports
#[derive(Debug)]
pub struct SupportsAtRule
{
	/// The parsed condition
	pub condition: SupportsCondition,
	
	/// Child rules
	pub rules: CssRules,
	
	/// The line and column of the rule's source code.
	pub source_location: SourceLocation,
}

impl ToCss for SupportsAtRule
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		dest.write_str("@supports ")?;
		self.condition.to_css(dest)?;
		dest.write_str(" {")?;
		for rule in self.rules.iter()
		{
			dest.write_str(" ")?;
			rule.to_css(dest)?;
		}
		dest.write_str(" }")
	}
}
