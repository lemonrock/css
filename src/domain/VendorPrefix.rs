// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// Vendor prefix.
#[derive(Clone, Debug)]
pub enum VendorPrefix
{
	/// -moz- prefix.
	moz,
	
	/// -webkit- prefix.
	webkit,

	Unrecognised(String),
}

impl ToCss for DocumentAtRule
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		let prefix = match *self
		{
			moz => "-moz-",
			webkit => "-webkit-",
			Unrecognised(ref prefix) => prefix.as_str(),
		};
		
		dest.write_str(prefix)
	}
}
