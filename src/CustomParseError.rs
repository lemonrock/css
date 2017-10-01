// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Debug)]
pub enum CustomParseError<'i>
{
	CouldNotParseSelectors,
	ThereAreNoSelectors,
	SelectorIsInvalidInContext(String),
	
	UnexpectedTokenForAtNamespaceRuleNamespaceValue(Token<'i>),
	
	UnexpectedCustomIdent(String),
	CustomIdentWasExcluded(String),
	
	KeyframePercentageWasNotBetweenZeroAndOneInclusive(f32),
	ImportantIsNotAllowedInKeyframePropertyDeclarationValues(SourceLocation),
	UnexpectedTokenWhenParsingZoom(Token<'i>),
	
	InvalidFontLanguageOverrideIdentifier(String),
	InvalidFontLanguageOverrideOpenTypeLanguageTag(String),
	FontFeatureSettingOpenTypeFeatureTagMustBeFourCharacters(String),
	FontFeatureSettingOpenTypeFeatureTagMustBePrintableAscii(String),
	FontFeatureSettingIfNotAnIntegerMustBeOnOrOff(String),
	FontFeatureSettingIntegerMustBePositive(i32),
	FontFaceAtRuleFontWeightWasNotAValidIdentifierOrInteger,
	FontFaceAtRuleFontFamilyCanNotBeGeneric,
	
	UnsupportedCounterStyleProperty(String),
	InvalidCounterStyleWithoutSymbols(System),
	InvalidCounterStyleNotEnoughSymbols(System),
	InvalidCounterStyleWithoutAdditiveSymbols,
	InvalidCounterStyleExtendsWithSymbols,
	InvalidCounterStyleExtendsWithAdditiveSymbols,
	CounterStyleSystemIsNotKnown(String),
	CounterStyleSymbolsCanNotBeEmpty,
	CounterStyleRangesCanNotHaveStartGreaterThanEnd(i32, i32),
	CounterStylePadMinLengthCanNotBeNegative(i32),
	CounterStyleAdditiveTupleWeightCanNotBeNegative(i32),
	CounterStyleAdditiveSymbolsCanNotHaveASecondWeightEqualToOrGreaterThanTheFirst,
	
	InvalidMediaType(String),
	DeprecatedMediaType(String),
	UnrecognisedMediaType(String),
	DeprecatedMediaQueryExpression(String),
	UnsupportedMediaQueryExpression(String),
	RatioNumeratorCanNotBeNegativeOrZero(i32),
	RatioDivisorCanNotBeNegativeOrZero(i32),
	UnrecognisedMediaQueryResolutionUnit(String),
	MediaQueryResolutionDoesNotSupportThisIdentifier(String),
	MediaQueryResolutionCanNotBeNegativeOrZero(CssFloat),
	UnexpectedTokenWhenParsingMediaQueryResolution(Token<'i>),
	MediaGridMustBeEitherZeroOrOne(i32),
	MediaTransform3DMustBeEitherZeroOrOne(i32),
	ColorBitDepthMustBeZeroOrAPositiveInteger(i32),
	ColorBitDepthMustBeAnInteger(f32),
	ColorIndexMustBeZeroOrAPositiveInteger(i32),
	ColorIndexMustBeAnInteger(f32),
	MonochromeBitDepthMustBeZeroOrAPositiveInteger(i32),
	MonochromeBitDepthMustBeAnInteger(f32),
	MediaTypeIsOnlyOptionalIfQualifiedIsNotSpecified,
	
	UnexpectedViewportProperty(String),
	
	AtRuleNamespaceMustBeBeforeAnyRuleExceptAtRuleCharsetAndAtRuleImport,
	InvalidParseState,
	UnexpectedCharsetAtRule,
	UnexpectedAtRule(String),
	
	UnsupportedFontFaceProperty(String),
	
	InvalidSupportsCondition(String),
	
	DocumentAtRuleUrlMatchingFunctionWasInvalid,
	
	BadUrlInDeclarationValueBlock(String),
	BadStringInDeclarationValueBlock(String),
	UnbalancedCloseParenthesisInDeclarationValueBlock,
	UnbalancedCloseSquareBracketInDeclarationValueBlock,
	UnbalancedCloseCurlyBracketInDeclarationValueBlock,
	
	CouldNotParseInteger,
	CouldNotParsePercentage,
	CouldNotParseNumber,
	CouldNotParseTimeDimension,
	CouldNotParseAngleDimension,
	CouldNotParseLengthDimension,
	CouldNotParseLengthDimensionOrPercentage,
	CouldNotParseTimeDimensionInCalcFunction,
	CouldNotParseAngleDimensionInCalcFunction,
	CouldNotParseLengthDimensionInCalcFunction,
}
