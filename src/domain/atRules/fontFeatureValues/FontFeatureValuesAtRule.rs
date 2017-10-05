// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// The [`@font-feature-values`][font-feature-values] at-rule.
///
/// [font-feature-values]: https://drafts.csswg.org/css-fonts-3/#at-font-feature-values-rule
#[derive(Clone, Debug, PartialEq)]
pub struct FontFeatureValuesAtRule
{
	/// Font family list for @font-feature-values rule.
	/// Family names cannot contain generic families.
	/// FamilyName also accepts only non-generic names.
	pub family_names: Vec<FamilyName>,
	
	/// A @swash block. Specifies a feature name that will work with the swash() functional notation of font-variant-alternates.
	pub swash: Vec<FontFeatureValuesDeclaration<SingleValue>>,
	
	/// A @stylistic block. Specifies a feature name that will work with the annotation() functional notation of font-variant-alternates.
	pub stylistic: Vec<FontFeatureValuesDeclaration<SingleValue>>,
	
	/// A @ornaments block. Specifies a feature name that will work with the ornaments() ] functional notation of font-variant-alternates.
	pub ornaments: Vec<FontFeatureValuesDeclaration<SingleValue>>,
	
	/// A @annotation block. Specifies a feature name that will work with the stylistic() functional notation of font-variant-alternates.
	pub annotation: Vec<FontFeatureValuesDeclaration<SingleValue>>,
	
	/// A @character-variant block. Specifies a feature name that will work with the styleset() functional notation of font-variant-alternates. The value can be a pair.
	pub character_variant: Vec<FontFeatureValuesDeclaration<PairValues>>,
	
	/// A @styleset block. Specifies a feature name that will work with the character-variant() functional notation of font-variant-alternates. The value can be a list.
	pub styleset: Vec<FontFeatureValuesDeclaration<VectorValues>>,
	
	/// The line and column of the rule's source code.
	pub source_location: SourceLocation,
}

impl ToCss for FontFeatureValuesAtRule
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		dest.write_str("@font-feature-values ")?;
		self.font_family_to_css(dest)?;
		dest.write_str("{\n")?;
		self.value_to_css(dest)?;
		dest.write_str("}")
	}
}

impl FontFeatureValuesAtRule
{
	fn new(family_names: Vec<FamilyName>, source_location: SourceLocation) -> Self
	{
		Self
		{
			family_names,
			swash: vec![],
			stylistic: vec![],
			ornaments: vec![],
			annotation: vec![],
			character_variant: vec![],
			styleset: vec![],
			source_location,
		}
	}
	
	pub(crate) fn parse_body<'i: 't, 't>(context: &ParserContext, input: &mut Parser<'i, 't>, family_names: Vec<FamilyName>, source_location: SourceLocation) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		let mut fontFeatureValuesRule = Self::new(family_names, source_location);
		
		{
			let mut iterator = RuleListParser::new_for_nested_rule(input, FontFeatureValuesAtRuleParser
			{
				context,
				rule: &mut fontFeatureValuesRule,
			});
			while let Some(possiblePreciseParseError) = iterator.next()
			{
				if possiblePreciseParseError.is_err()
				{
					return Err(possiblePreciseParseError.unwrap_err().error);
				}
			}
		}
		
		Ok(fontFeatureValuesRule)
	}
	
	/// Prints font family names.
	pub(crate) fn font_family_to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		let mut iter = self.family_names.iter();
		iter.next().unwrap().to_css(dest)?;
		for val in iter
		{
			dest.write_str(", ")?;
			val.to_css(dest)?;
		}
		Ok(())
	}
	
	/// Prints inside of `@font-feature-values` block.
	pub(crate) fn value_to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		#[inline(always)]
		fn writeBlock<W: fmt::Write, T: ToCss>(dest: &mut W, name: &str, block: &Vec<FontFeatureValuesDeclaration<T>>) -> fmt::Result
		{
			if block.len() > 0
			{
				dest.write_char('@')?;
				dest.write_str(name)?;
				dest.write_char('{')?;
				for declaration in block.iter()
				{
					declaration.to_css(dest)?;
				}
				dest.write_char('}')
			}
			else
			{
				Ok(())
			}
		}
		
		writeBlock(dest, "swash", &self.swash)?;
		writeBlock(dest, "stylistic", &self.stylistic)?;
		writeBlock(dest, "ornaments", &self.ornaments)?;
		writeBlock(dest, "annotation", &self.annotation)?;
		writeBlock(dest, "character-variant", &self.character_variant)?;
		writeBlock(dest, "styleset", &self.styleset)
	}
	
	/// Returns length of all at-rules.
	pub fn len(&self) -> usize
	{
		let mut len = self.swash.len();
		len += self.stylistic.len();
		len += self.ornaments.len();
		len += self.annotation.len();
		len += self.character_variant.len();
		len + self.styleset.len()
	}
}
