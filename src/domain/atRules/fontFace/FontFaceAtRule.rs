// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A `@font-face` rule.
///
/// https://drafts.csswg.org/css-fonts/#font-face-rule
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontFaceRuleData
{
	/// The name of this font face
	pub family: Option<FamilyName>,
	
	/// The alternative sources for this font face.
	pub sources: Option<Vec<Source>>,
	
	/// The style of this font face
	pub style: Option<FontStyle>,
	
	/// The weight of this font face
	pub weight: Option<FontWeight>,
	
	/// The stretch of this font face
	pub stretch: Option<FontStretch>,
	
	/// The display of this font face
	pub display: Option<FontDisplay>,
	
	/// The ranges of code points outside of which this font face should not be used.
	pub unicode_range: Option<UnicodeRange>,
	
	/// The feature settings of this font face.
	pub feature_settings: Option<FontFeatureSettings>,
	
	/// The language override of this font face.
	pub language_override: Option<FontLanguageOverride>,

	/// Line and column of the @font-face rule source code.
	pub source_location: SourceLocation,
}

impl ToCss for FontFaceRuleData
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		#[inline(always)]
		fn writePropertyDeclaration<W: fmt::Write, T>(dest: &mut W, name: &str, value: &Option<T>)
		{
			if let Some(ref value) = value
			{
				dest.write_str("  ")?;
				dest.write_str(name)?;
				dest.write_str(": ")?;
				value.to_css(dest)?;
				dest.write_str(";\n")?;
			}
		}
		
		dest.write_str("@font-face {\n")?;
		
		writePropertyDeclaration(dest, "font-family", &self.family);
		writePropertyDeclaration(dest, "src", &self.sources);
		writePropertyDeclaration(dest, "font-style", &self.style);
		writePropertyDeclaration(dest, "font-weight", &self.weight);
		writePropertyDeclaration(dest, "font-stretch", &self.stretch);
		writePropertyDeclaration(dest, "font-display", &self.display);
		writePropertyDeclaration(dest, "unicode-range", &self.unicode_range);
		writePropertyDeclaration(dest, "font-feature-settings", &self.feature_settings);
		writePropertyDeclaration(dest, "font-language-override", &self.language_override);
		
		dest.write_str("}")
	}
}

impl FontFaceAtRule
{
	fn empty(source_location: SourceLocation) -> Self
	{
		Self
		{
			family: None,
			sources: None,
			style: None,
			weight: None,
			stretch: None,
			display: None,
			unicode_range: None,
			feature_settings: None,
			language_override: None,
			source_location,
		}
	}

	/// Parse the block inside a `@font-face` rule.
	pub(crate) fn parse_body<'i>(context: &ParserContext, input: &mut Parser, source_location: SourceLocation) -> Result<FontFaceAtRule, ParseError<'i>>
	{
		let mut rule = Self::empty(source_location);
		
		{
			let parser = FontFaceAtRuleParser
			{
				context,
				rule: &mut rule,
			};
			
			let mut iter = DeclarationListParser::new(input, parser);
			while let Some(declaration) = iter.next()
			{
				if declaration.is_err()
				{
					return declaration;
				}
			}
		}
		
		Ok(rule)
	}
	
	/// Per https://github.com/w3c/csswg-drafts/issues/1133 an @font-face rule is valid as far as the CSS parser is concerned even if it doesn't have a font-family or src declaration.
	///
	/// However both are required for the rule to represent an actual font face.
	pub fn font_face(&self) -> Option<FontFace>
	{
		if self.family.is_some() && self.sources.is_some()
		{
			Some(FontFace(self))
		}
		else
		{
			None
		}
	}
}
