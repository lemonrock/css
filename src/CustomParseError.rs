// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


pub enum CustomParseError<'i>
{
	UnexpectedTokenForAtNamespaceRuleNamespaceValue(Token<'i>),
	UnexpectedTokenWhenParsingZoom(Token<'i>),
	
	AtRuleNamespaceMustBeBeforeAnyRuleExceptAtRuleCharsetAndAtRuleImport,
	KeyframePercentageWasNotBetweenZeroAndOneInclusive(f32),
	UnexpectedCustomIdent(String),
	CustomIdentWasExcluded(String),
	InvalidCounterStyleWithoutSymbols(System),
	InvalidCounterStyleNotEnoughSymbols(System),
	InvalidCounterStyleWithoutAdditiveSymbols,
	InvalidCounterStyleExtendsWithSymbols,
	InvalidCounterStyleExtendsWithAdditiveSymbols,
	
	UnsupportedKeyframePropertyDeclaration(PreciseParseError<'i>),
	UnsupportedCounterStyleDescriptorDeclaration(PreciseParseError<'i>),
	UnsupportedViewportDescriptorDeclaration(PreciseParseError<'i>),
	
	ImportantIsNotAllowedInKeyframePropertyDeclarationValues(SourceLocation),
	InvalidFontLanguageOverrideIdentifier(String),
	InvalidFontLanguageOverrideOpenTypeLanguageTag(String),
	FontFeatureSettingOpenTypeFeatureTagMustBeFourCharacters(String),
	FontFeatureSettingOpenTypeFeatureTagMustBePrintableAscii(String),
	FontFeatureSettingIfNotAnIntegerMustBeOnOrOff(String),
	FontFeatureSettingIntegerMustBePositive(i32),
}
