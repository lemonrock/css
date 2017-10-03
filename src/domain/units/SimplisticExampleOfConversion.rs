// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SimplisticExampleOfConversion<U: Unit>
{
	// Font relative lengths
	pub em: U::Number,
	pub ex: U::Number,
	pub ch: U::Number,
	pub rem: U::Number,
	
	// Viewport percentage lengths
	pub vw: U::Number,
	pub vh: U::Number,
	pub vmin: U::Number,
	pub vmax: U::Number,
	
	// Percent conversion
	pub one_hundred_percent_in_absolute_units: U::Number,
	
	// Simplistic approach
	pub cssVariablesWithLowerCaseNamesWithoutLeadingDoubleDash: HashMap<String, Box<ToUnit<U>>>,
	pub attributesWithLowerCaseNames: HashMap<String, Box<ToUnit<U>>>,
}

impl<U: Unit> FontRelativeLengthConversion<U::Number> for SimplisticExampleOfConversion<U>
{
	#[inline(always)]
	fn em(&self) -> U::Number
	{
		self.em
	}
	
	#[inline(always)]
	fn ex(&self) -> U::Number
	{
		self.ex
	}
	
	#[inline(always)]
	fn ch(&self) -> Number
	{
		self.ch
	}
	
	#[inline(always)]
	fn rem(&self) -> U::Number
	{
		self.rem
	}
}

impl<U: Unit> ViewportPercentageLengthConversion for SimplisticExampleOfConversion<U>
{
	#[inline(always)]
	fn vw(&self) -> U::Number
	{
		self.vw
	}
	
	#[inline(always)]
	fn vh(&self) -> U::Number
	{
		self.vh
	}
	
	#[inline(always)]
	fn vmin(&self) -> U::Number
	{
		self.vmin
	}
	
	#[inline(always)]
	fn vmax(&self) -> U::Number
	{
		self.vmax
	}
}

impl<U: Unit> PercentageConversion<U::Number> for SimplisticExampleOfConversion<U>
{
	#[inline(always)]
	fn one_hundred_percent_in_absolute_units(&self) -> U::Number
	{
		self.one_hundred_percent_in_absolute_units
	}
}

impl<U: Unit> CssVariableConversion<U> for SimplisticExampleOfConversion<U>
{
	#[inline(always)]
	fn cssVariableValue(&self, css_variable_lower_case_name_without_leading_double_dash: &str) -> Option<U>
	{
		self.cssVariablesWithLowerCaseNamesWithoutLeadingDoubleDash.get(css_variable_lower_case_name_without_leading_double_dash).map(|value| value.try_to_canonical_unit())
	}
}

impl<U: Unit> AttributeConversion<U> for SimplisticExampleOfConversion<U>
{
	#[inline(always)]
	fn attributeValue(&self, attribute_lower_case_name: &str) -> Option<U>
	{
		self.attributesWithLowerCaseNames.get(attribute_lower_case_name).map(|value| value.try_to_canonical_unit())
	}
}
