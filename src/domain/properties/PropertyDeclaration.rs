// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PropertyDeclaration
{
	pub name: Atom,
	pub value: UnparsedPropertyValue,
	pub importance: Importance,
}

impl ToCss for PropertyDeclaration
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		self.name.to_css(dest)?;
		dest.write_str(":")?;
		
		self.value.to_css(dest)?;
		
		if self.importance.important()
		{
			dest.write_str("!important")?;
		}
		
		dest.write_str(";")
	}
}

impl PropertyDeclaration
{
	/// https://drafts.csswg.org/css-variables/#typedef-custom-property-name
	pub fn hasACustomPropertyName(&self) -> bool
	{
		self.name.starts_with("--")
	}
	
	pub fn hasAVendorPrefix(&self) -> bool
	{
		if self.hasACustomPropertyName()
		{
			false
		}
		else
		{
			self.name.starts_with("-")
		}
	}
}
