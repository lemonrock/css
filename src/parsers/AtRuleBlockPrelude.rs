// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A rule prelude for at-rule with block.
pub enum AtRuleBlockPrelude
{
	/// A @counter-style rule prelude, with its counter style name.
	CounterStyle(CounterStyleIdent),
	
	/// A @document rule, with its conditional.
	Document(Option<VendorPrefix>, DocumentCondition, SourceLocation),
	
	/// A @font-face rule prelude.
	FontFace(SourceLocation),
	
	/// A @font-feature-values rule prelude, with its FamilyName list.
	FontFeatureValues(Vec<FamilyName>, SourceLocation),
	
	/// A @keyframes rule, with its animation name and vendor prefix if exists.
	Keyframes(Option<VendorPrefix>, KeyframesName, SourceLocation),
	
	/// A @media rule prelude, with its media queries.
	Media(MediaList, SourceLocation),
	
	/// A @page rule prelude.
	Page(SourceLocation),
	
	/// An @supports rule, with its conditional
	Supports(SupportsCondition, SourceLocation),
	
	/// A @viewport rule prelude.
	Viewport,
}
