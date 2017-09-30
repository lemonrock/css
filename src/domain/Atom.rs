// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// NOTE: At some future point, Atom may become a wrapper around a string cache value
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Atom(String);

impl Deref for Atom
{
	type Target = str;
	
	fn deref(&self) -> &Self::Target
	{
		self.0
	}
}

impl ToCss for Atom
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		serialize_identifier(&self.0, dest)
	}
}

impl  Atom
{
	fn from(value: &str) -> Self
	{
		Atom(value.to_owned())
	}
}
