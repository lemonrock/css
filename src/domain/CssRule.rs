// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum CssRule
{
	/// No Charset here, CSSCharsetRule has been removed from CSSOM (https://drafts.csswg.org/cssom/#changes-from-5-december-2013)
	// Charset(xxx),
	Namespace(NamespaceAtRule),
	Import(ImportAtRule),
	Style(StyleRule),
	Media(MediaAtRule),
	FontFace(FontFaceAtRule),
	FontFeatureValues(FontFeatureValuesAtRule),
	CounterStyle(CounterStyleAtRule),
	Viewport(ViewportAtRule),
	Keyframes(KeyframesAtRule),
	Supports(SupportsAtRule),
	Page(PageAtRule),
	Document(DocumentAtRule),
}

impl ToCss for CssRule
{
	// https://drafts.csswg.org/cssom/#serialize-a-css-rule
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		use self::CssRule::*;
		
		match *self
		{
			Namespace(ref rule) => rule.to_css(dest),
			Import(ref rule) => rule.to_css(dest),
			Style(ref rule) => rule.to_css(dest),
			FontFace(ref rule) => rule.to_css(dest),
			FontFeatureValues(ref rule) => rule.to_css(dest),
			CounterStyle(ref rule) => rule.to_css(dest),
			Viewport(ref rule) => rule.to_css(dest),
			Keyframes(ref rule) => rule.to_css(dest),
			Media(ref rule) => rule.to_css(dest),
			Supports(ref rule) => rule.to_css(dest),
			Page(ref rule) => rule.to_css(dest),
			Document(ref rule) => rule.to_css(dest),
		}
	}
}

impl CssRule
{
	/// Returns the CSSOM rule type of this rule.
	pub fn rule_type(&self) -> CssRuleType
	{
		use self::CssRule::*;
		
		match *self
		{
			Style(_) => CssRuleType::Style,
			Import(_) => CssRuleType::Import,
			Media(_) => CssRuleType::Media,
			FontFace(_) => CssRuleType::FontFace,
			FontFeatureValues(_) => CssRuleType::FontFeatureValues,
			CounterStyle(_) => CssRuleType::CounterStyle,
			Keyframes(_) => CssRuleType::Keyframes,
			Namespace(_) => CssRuleType::Namespace,
			Viewport(_) => CssRuleType::Viewport,
			Supports(_) => CssRuleType::Supports,
			Page(_) => CssRuleType::Page,
			Document(_)  => CssRuleType::Document,
		}
	}
	
	fn rule_state(&self) -> State
	{
		use self::CssRule::*;
		use State::*;
		
		match *self
		{
			/// No Charset here, CSSCharsetRule has been removed from CSSOM (https://drafts.csswg.org/cssom/#changes-from-5-december-2013)
			// Charset(..) => Start,
			Import(..) => Imports,
			Namespace(..) => Namespaces,
			_ => Body,
		}
	}
}
