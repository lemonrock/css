// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Debug)]
/// A @document rule
pub struct DocumentAtRule
{
	pub vendor_prefix: Option<VendorPrefix>,
	
	/// The parsed condition
	pub condition: DocumentCondition,
	
	/// Child rules
	pub rules: CssRules,
	
	/// The line and column of the rule's source code.
	pub source_location: SourceLocation,
}

impl ToCss for DocumentAtRule
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		dest.write_str("@")?;
		if let Some(vendor_prefix) = self.vendor_prefix
		{
			vendor_prefix.to_css(dest)
		}
		dest.write_str("document ")?;
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

impl DocumentAtRule
{
	/// Evaluate a document condition.
	pub fn evaluate<D: Device>(&self, device: &D) -> bool
	{
		self.condition.evaluate(device)
	}
}
