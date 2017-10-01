// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct OurSelectorImpl;

impl SelectorImpl for OurSelectorImpl
{
	type AttrValue = String;
	
	type Identifier = Atom;
	
	type ClassName = Atom;
	
	type LocalName = Atom;
	
	type NamespacePrefix = Atom;
	
	type NamespaceUrl = NamespaceUrl;
	
	type BorrowedNamespaceUrl = NamespaceUrl;
	
	type BorrowedLocalName = Atom;
	
	type NonTSPseudoClass = NonTreeStructuralPseudoClass;
	
	type PseudoElement = PseudoElement;
	
	#[inline(always)]
	fn is_active_or_hover(pseudo_class: &Self::NonTSPseudoClass) -> bool
	{
		use self::NonTreeStructuralPseudoClass::*;
		
		match *pseudo_class
		{
			active => true,
			hover => true,
			_ => false,
		}
	}
}
