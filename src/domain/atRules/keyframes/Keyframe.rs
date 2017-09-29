// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A keyframe.
#[derive(Debug)]
pub struct Keyframe
{
	/// The selector this keyframe was specified from.
	pub selector: KeyframeSelector,
	
	/// The declaration block that was declared inside this keyframe.
	pub propertyDeclarations: Vec<PropertyDeclaration>,
	
	/// The line and column of the rule's source code.
	pub source_location: SourceLocation,
}

impl ToCss for Keyframe
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		self.selector.to_css(dest)?;
		dest.write_str(" { ")?;
		self.block.to_css(dest)?;
		dest.write_str(" }")?;
		Ok(())
	}
}
