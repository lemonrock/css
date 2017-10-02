// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// An [`@media`][media] url.
///
/// [media]: https://drafts.csswg.org/css-conditional/#at-ruledef-media
#[derive(Debug, Clone)]
pub struct MediaAtRule
{
	/// The list of media queries used by this media rule.
	pub media_queries: MediaList,
	
	/// The nested rules to this media rule.
	pub rules: CssRules,
	
	/// The source position where this media rule was found.
	pub source_location: SourceLocation,
}

impl ToCss for MediaAtRule
{
	// https://drafts.csswg.org/cssom/#serialize-a-css-rule CSSMediaRule
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		dest.write_str("@media ")?;
		self.media_queries.to_css(dest)?;
		dest.write_str(" {")?;
		for rule in self.rules.iter()
		{
			dest.write_str(" ")?;
			rule.to_css(dest)?;
		}
		dest.write_str(" }")
	}
}
