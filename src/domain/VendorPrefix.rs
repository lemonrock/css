// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// Vendor prefixes
/// Sort order is such that -o- sorts before -webkit- and -ms- sorts after -webkit-
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum VendorPrefix
{
	/// -o- prefix (legacy Opera Presto prefix).
	o,
	
	/// -moz- prefix.
	moz,
	
	/// -webkit- prefix (Is sometimes also used by IE, Edge and Blink-based browsers (Chrome and Opera)).
	webkit,
	
	/// -ms- prefix.
	ms,
	
	/// -servo- prefix
	servo,
	
	Unrecognised(String),
}

impl ToCss for VendorPrefix
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		use self::VendorPrefix::*;
		
		match *self
		{
			moz => dest.write_str("-moz-"),
			
			webkit => dest.write_str("-webkit-"),
			
			ms => dest.write_str("-ms-"),
			
			o => dest.write_str("-o-"),
			
			servo => dest.write_str("-servo-"),
			
			Unrecognised(ref prefix) =>
			{
				dest.write_char('-')?;
				dest.write_str(prefix.as_str())?;
				dest.write_char('-')
			},
		}
	}
}

impl VendorPrefix
{
	#[inline(always)]
	pub fn prefix(&self, name: &str) -> String
	{
		use self::VendorPrefix::*;
		
		fn knownPrefix(prefix: &str, name: &str) -> String
		{
			let mut prefixed = String::with_capacity(prefix.len() + name.len());
			prefixed.push_str(prefix);
			prefixed.push_str(name);
			prefixed
		}
		
		match self
		{
			&moz => knownPrefix("-moz-", name),
			
			&webkit => knownPrefix("-webkit-", name),
			
			&ms => knownPrefix("-ms-", name),
			
			&o => knownPrefix("-o-", name),
			
			&servo => knownPrefix("-servo-", name),
			
			&Unrecognised(ref prefix) =>
			{
				let mut prefixed = String::with_capacity(1 + prefix.len() + 1 + name.len());
				prefixed.push('-');
				prefixed.push_str(prefix);
				prefixed.push('-');
				prefixed.push_str(name);
				prefixed
			},
		}
	}
}
