// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct OurSelectorParser
{
	namespaces: Rc<Namespaces>,
}

impl<'i> ::selectors::parser::Parser<'i> for OurSelectorParser
{
	type Impl = OurSelectorImpl;
	
	type Error = CustomParseError<'i>;
	
	#[inline(always)]
	fn parse_non_ts_pseudo_class(&self, name: CowRcStr<'i>) -> Result<<Self::Impl as SelectorImpl>::NonTSPseudoClass, ParseError<'i, SelectorParseError<'i, Self::Error>>>
	{
		NonTreeStructuralPseudoClass::parse_without_arguments(name)
	}
	
	#[inline(always)]
	fn parse_non_ts_functional_pseudo_class<'t>(&self, name: CowRcStr<'i>, arguments: &mut Parser<'i, 't>) -> Result<<Self::Impl as SelectorImpl>::NonTSPseudoClass, ParseError<'i, SelectorParseError<'i, Self::Error>>>
	{
		NonTreeStructuralPseudoClass::parse_with_arguments(name, arguments, self)
	}
	
	#[inline(always)]
	fn parse_pseudo_element(&self, name: CowRcStr<'i>) -> Result<<Self::Impl as SelectorImpl>::PseudoElement, ParseError<'i, SelectorParseError<'i, Self::Error>>>
	{
		PseudoElement::parse_without_arguments(name)
	}
	
	#[inline(always)]
	fn parse_functional_pseudo_element<'t>(&self, name: CowRcStr<'i>, arguments: &mut Parser<'i, 't>) -> Result<<Self::Impl as SelectorImpl>::PseudoElement, ParseError<'i, SelectorParseError<'i, Self::Error>>>
	{
		PseudoElement::parse_with_arguments(name, arguments, self)
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
	pub(crate) fn parse<'i, 't>(&self, input: &mut Parser<'i, 't>) -> Result<DeduplicatedSelectors, ParseError<'i, CustomParseError<'i>>>
	{
		self.parse_internal(input, |_| false)
	}
	
	#[inline(always)]
	fn parse_internal<'i, 't, F: Fn(&OurSelector) -> bool>(&self, input: &mut Parser<'i, 't>, isInvalidSelector: F) -> Result<DeduplicatedSelectors, ParseError<'i, CustomParseError<'i>>>
	{
		let selectors = self.parse_selectors(input).map_err(|parseError|
		{
			match parseError
			{
				// We are changing from a ParseError<SelectorParseError> to a ParseError<CustomParseError<'i>>, hence this superficially looking redundant code
				ParseError::Basic(basicParseError) => ParseError::Basic(basicParseError),
				ParseError::Custom(selectorParseError) => ParseError::Custom(CustomParseError::SpecificSelectorParseError(Box::new(selectorParseError))),
			}
		})?;
		
		if selectors.is_empty()
		{
			return Err(ParseError::Custom(CustomParseError::ThereAreNoSelectors));
		}
		
		let mut deduplicatedSelectors = OrderMap::with_capacity(selectors.len());
		for selector in selectors
		{
			let selectorCss = selector.to_css_string();
			if isInvalidSelector(&selector)
			{
				return Err(ParseError::Custom(CustomParseError::SelectorIsInvalidInContext(selectorCss)))
			}
			
			// Selector does not implement Eq or Hash... Grrr...
			deduplicatedSelectors.insert(selectorCss, selector);
		}
		
		Ok(DeduplicatedSelectors(deduplicatedSelectors))
	}
	
	#[inline(always)]
	fn parse_selectors<'i, 't>(&self, input: &mut Parser<'i, 't>) -> Result<SmallVec<[OurSelector; 1]>, ParseError<'i, SelectorParseError<'i, CustomParseError<'i>>>>
	{
		let selectorList = SelectorList::parse(self, input)?;
		Ok(selectorList.0)
	}
}
