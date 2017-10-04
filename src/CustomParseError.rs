// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) enum CustomParseError<'i>
{
	SpecificSelectorParseError(Box<SelectorParseError<'i, CustomParseError<'i>>>),
	ThereAreNoSelectors,
	SelectorIsInvalidInContext(String),
	UnsupportedPseudoClassOrElement(String),
	NonTreeStructuralPseudoClassScopeIsObsoleteAsOfFirefox55,
	
	UnexpectedTokenForAtNamespaceRuleNamespaceValue(Token<'i>),
	
	UnsupportedAtRule(CowRcStr<'i>),
	
	UnexpectedCustomIdent(CowRcStr<'i>),
	CustomIdentWasExcluded(CowRcStr<'i>),
	
	KeyframePercentageWasNotBetweenZeroAndOneInclusive(f32),
	ImportantIsNotAllowedInKeyframePropertyDeclarationValues(SourceLocation),
	UnexpectedTokenWhenParsingZoom(Token<'i>),
	
	UnsupportedFontFaceProperty(CowRcStr<'i>),
	
	InvalidFontLanguageOverrideIdentifier(CowRcStr<'i>),
	InvalidFontLanguageOverrideOpenTypeLanguageTag(CowRcStr<'i>),
	FontFeatureSettingOpenTypeFeatureTagMustBeFourCharacters(CowRcStr<'i>),
	FontFeatureSettingOpenTypeFeatureTagMustBePrintableAscii(CowRcStr<'i>),
	FontFeatureSettingIfNotAnIntegerMustBeOnOrOff(CowRcStr<'i>),
	FontFeatureSettingIntegerMustBePositive(i32),
	FontFaceAtRuleFontWeightWasNotAValidIdentifierOrInteger,
	FontFaceAtRuleFontFamilyCanNotBeGeneric,
	
	UnsupportedCounterStyleProperty(CowRcStr<'i>),
	InvalidCounterStyleWithoutSymbols(System),
	InvalidCounterStyleNotEnoughSymbols(System),
	InvalidCounterStyleWithoutAdditiveSymbols,
	InvalidCounterStyleExtendsWithSymbols,
	InvalidCounterStyleExtendsWithAdditiveSymbols,
	CounterStyleSystemIsNotKnown(CowRcStr<'i>),
	CounterStyleSymbolsCanNotBeEmpty,
	CounterStyleRangesCanNotHaveStartGreaterThanEnd(i32, i32),
	CounterStylePadMinLengthCanNotBeNegative(i32),
	CounterStyleAdditiveTupleWeightCanNotBeNegative(i32),
	CounterStyleAdditiveSymbolsCanNotHaveASecondWeightEqualToOrGreaterThanTheFirst,
	DecimalOrDiscIsNotAllowedInACounterStyleIdentInACounterStyleAtRule,
	NoneIsNotAllowedInACounterStyleIdent,
	
	InvalidMediaType(CowRcStr<'i>),
	DeprecatedMediaType(CowRcStr<'i>),
	UnrecognisedMediaType(CowRcStr<'i>),
	DeprecatedMediaQueryExpression(CowRcStr<'i>),
	UnsupportedMediaQueryExpression(CowRcStr<'i>),
	RatioNumeratorCanNotBeNegativeOrZero(i32),
	RatioDivisorCanNotBeNegativeOrZero(i32),
	UnrecognisedMediaQueryResolutionUnit(CowRcStr<'i>),
	MediaQueryResolutionDoesNotSupportThisIdentifier(CowRcStr<'i>),
	MediaQueryResolutionCanNotBeNegativeOrZero,
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
	
	UnexpectedTokenForViewportLength(Token<'i>),
	UnexpectedViewportProperty(CowRcStr<'i>),
	
	ViewportLengthsAreNotAllowedInAPageAtRule,
	LengthDimensionWasUnrecognised(String),
	
	AtRuleImportMustBeBeforeAnyRuleExceptAtRuleCharset,
	AtRuleNamespaceMustBeBeforeAnyRuleExceptAtRuleCharsetAndAtRuleImport,
	InvalidParseState,
	UnexpectedCharsetAtRule,
	UnexpectedAtRule(String),
	
	InvalidSupportsCondition(CowRcStr<'i>),
	
	DocumentAtRuleUrlMatchingFunctionWasInvalid,
	
	BadUrlInDeclarationValueBlock(CowRcStr<'i>),
	BadStringInDeclarationValueBlock(CowRcStr<'i>),
	UnbalancedCloseParenthesisInDeclarationValueBlock,
	UnbalancedCloseSquareBracketInDeclarationValueBlock,
	UnbalancedCloseCurlyBracketInDeclarationValueBlock,
	
	CouldNotParseCssSignedNumber(::domain::numbers::CssNumberConversionError, f32),
	CouldNotParseCssUnsignedNumber(::domain::numbers::CssNumberConversionError, f32),
	CouldNotParseDimensionLessNumber(f32),
	CouldNotParseDimension(f32, CowRcStr<'i>),
	UnknownFunctionInValueExpression(CowRcStr<'i>),
	
	CssVariablesInVarExpressionsMustStartWithTwoDashes(CowRcStr<'i>),
	
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
