// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A [`@keyframes`][keyframes] rule.
/// [keyframes]: https://drafts.csswg.org/css-animations/#keyframes
#[derive(Debug)]
pub struct KeyframesAtRule
{
	/// Vendor prefix type the @keyframes has.
	pub vendor_prefix: Option<VendorPrefix>,
	
	/// The name of the current animation.
	pub name: KeyframesName,
	
	/// The keyframes specified for this CSS rule.
	pub keyframes: Vec<Keyframe>,
	
	/// The line and column of the rule's source code.
	pub source_location: SourceLocation,
}

impl ToCssWithGuard for KeyframesRule {
	// Serialization of KeyframesRule is not specced.
	fn to_css<W>(&self, guard: &SharedRwLockReadGuard, dest: &mut W) -> fmt::Result
		where W: fmt::Write,
	{
		dest.write_str("@")?;
		if vendor_prefix = self.vendor_prefix
		{
			vendor_prefix.to_css(dest)
		}
		dest.write_str("keyframes ")?;
		self.name.to_css(dest)?;
		dest.write_str(" {")?;
		let iter = self.keyframes.iter();
		for lock in iter {
			dest.write_str("\n")?;
			let keyframe = lock.read_with(&guard);
			keyframe.to_css(guard, dest)?;
		}
		dest.write_str("\n}")
	}
}

impl KeyframesRule {
	/// Returns the index of the last keyframe that matches the given selector.
	/// If the selector is not valid, or no keyframe is found, returns None.
	///
	/// Related spec:
	/// https://drafts.csswg.org/css-animations-1/#interface-csskeyframesrule-findrule
	pub fn find_rule(&self, guard: &SharedRwLockReadGuard, selector: &str) -> Option<usize> {
		let mut input = ParserInput::new(selector);
		if let Ok(selector) = Parser::new(&mut input).parse_entirely(KeyframeSelector::parse) {
			for (i, keyframe) in self.keyframes.iter().enumerate().rev() {
				if keyframe.read_with(guard).selector == selector {
					return Some(i);
				}
			}
		}
		None
	}
}
