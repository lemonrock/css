// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

/// A list of CSS rules.
#[derive(Debug, Clone)]
pub struct CssRules(pub Vec<CssRule>);

impl CssRules
{
	/// Whether this CSS rules is empty.
	pub fn is_empty(&self) -> bool
	{
		self.0.is_empty()
	}
}

impl CssRules
{
	/// Trivially construct a new set of CSS rules.
	#[inline(always)]
	pub fn new(rules: Vec<CssRule>) -> Self
	{
		CssRules(rules)
	}
	
	/// Returns whether all the rules in this list are namespace or import rules.
	fn only_ns_or_import(&self) -> bool
	{
		use self::CssRule::*;
		
		self.0.iter().all(|r|
		{
			match *r
			{
				Namespace(..) | Import(..) => true,
				_ => false
			}
		})
	}
	
	/// https://drafts.csswg.org/cssom/#remove-a-css-rule
	pub fn remove_rule(&mut self, index: usize) -> Result<(), RulesMutateError>
	{
		use self::CssRule::Namespace;
		use self::RulesMutateError::*;
		
		// Step 1, 2
		if index >= self.0.len()
		{
			return Err(IndexSize);
		}
		
		{
			// Step 3
			let ref rule = self.0[index];
			
			// Step 4
			if let Namespace(..) = *rule
			{
				if !self.only_ns_or_import()
				{
					return Err(InvalidState);
				}
			}
		}
		
		// Step 5, 6
		self.0.remove(index);
		
		Ok(())
	}
}
