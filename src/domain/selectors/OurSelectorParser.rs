// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct OurSelectorParser
{
	namespaces: Rc<Namespaces>,
}

impl ::selectors::parser::Parser for OurSelectorParser
{
	type Impl = OurSelectorImpl;
	
	#[inline(always)]
	fn parse_non_ts_pseudo_class(&self, name: Cow<str>) -> Result<<Self::Impl as SelectorImpl>::NonTSPseudoClass, ()>
	{
		NonTreeStructuralPseudoClass::parse_without_arguments(name)
	}
	
	#[inline(always)]
	fn parse_non_ts_functional_pseudo_class(&self, name: Cow<str>, arguments: &mut Parser) -> Result<<Self::Impl as SelectorImpl>::NonTSPseudoClass, ()>
	{
		NonTreeStructuralPseudoClass::parse_with_arguments(name, arguments, self)
	}
	
	#[inline(always)]
	fn parse_pseudo_element(&self, name: Cow<str>) -> Result<<Self::Impl as SelectorImpl>::PseudoElement, ()>
	{
		PseudoElement::parse_without_arguments(name)
	}
	
	#[inline(always)]
	fn default_namespace(&self) -> Option<<Self::Impl as SelectorImpl>::NamespaceUrl>
	{
		self.namespaces.default_namespace()
	}
	
	#[inline(always)]
	fn namespace_for_prefix(&self, prefix: &<Self::Impl as SelectorImpl>::NamespacePrefix) -> Option<<Self::Impl as SelectorImpl>::NamespaceUrl>
	{
		self.namespaces.namespace_for_prefix(prefix)
	}
}

impl OurSelectorParser
{
	#[inline(always)]
	pub(crate) fn parse<'i, 't>(&self, input: &mut Parser<'i, 't>) -> Result<DeduplicatedSelectors, ParseError<'i, CustomParseError>>
	{
		self.parse_internal(input, |_| false)
	}
	
	#[inline(always)]
	fn parse_internal<'i, 't, F: Fn(&OurSelector) -> bool>(&self, input: &mut Parser<'i, 't>, isInvalidSelector: F) -> Result<DeduplicatedSelectors, ParseError<'i, CustomParseError>>
	{
		let selectors = self.parse_selectors(input)?;
		debug_assert!(!selectors.is_empty());
		
		let mut deduplicatedSelectors = OrderMap::with_capacity(selectors.len());
		for selector in selectors
		{
			if isInvalidSelector(selector)
			{
				return Err(ParseError::Custom(CustomParseError::SelectorIsInvalidInContext(selector.to_css_string())))
			}
			
			// Selector does not implement Eq or Hash... Grrr...
			deduplicatedSelectors.insert(selector.to_css_string(), selector);
		}
		
		Ok(DeduplicatedSelectors(deduplicatedSelectors))
	}
	
	#[inline(always)]
	fn parse_selectors<'i, 't>(&self, input: &mut Parser<'i, 't>) -> Result<Vec<OurSelector>, ParseError<'i, CustomParseError>>
	{
		let selectorList = SelectorList::parse(self, input).map_err(|_| ParseError::Custom(CustomParseError::CouldNotParseSelectors))?;
		let selectors = selectorList.0;
		if selectors.is_empty()
		{
			Err(ParseError::Custom(CustomParseError::ThereAreNoSelectors))
		}
		else
		{
			Ok(selectors)
		}
	}
}
