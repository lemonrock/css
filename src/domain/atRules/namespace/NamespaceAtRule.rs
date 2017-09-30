// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A `@namespace` rule.
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct NamespaceAtRule
{
	/// The namespace prefix, and `None` if it's the default Namespace
	pub prefix: Option<NamespacePrefix>,
	
	/// The actual namespace url.
	pub url: NamespaceUrl,
	
	/// The source location this rule was found at.
	pub source_location: SourceLocation,
}

impl ToCss for NamespaceAtRule
{
	// https://drafts.csswg.org/cssom/#serialize-a-css-rule
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		dest.write_str("@namespace ")?;
		
		if let Some(ref prefix) = self.prefix
		{
			dest.write_str(&(prefix.0).0)?;
			dest.write_str(" ")?;
		}
		
		dest.write_str("url(")?;
		dest.write_str(&(self.url.0).0)?;
		dest.write_str(");")
	}
}
