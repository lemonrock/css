// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// NOTE: At some future point, Atom may become a wrapper around a string cache value
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Atom(String);

impl Deref for Atom
{
	type Target = str;
	
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl ToCss for Atom
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		serialize_identifier(&self.0, dest)
	}
}

impl Display for Atom
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		self.0.fmt(f)
	}
}

impl From<String> for Atom
{
	fn from(value: String) -> Self
	{
		Atom(value)
	}
}

impl<'a> From<&'a str> for Atom
{
	fn from(value: &'a str) -> Self
	{
		Atom(value.to_owned())
	}
}

impl<'i> From<CowRcStr<'i>> for Atom
{
	fn from(value: CowRcStr<'i>) -> Self
	{
		Atom::from(value.as_ref())
	}
}

impl<'a, 'i> From<&'a CowRcStr<'i>> for Atom
{
	fn from(value: &'a CowRcStr<'i>) -> Self
	{
		Atom::from(value.as_ref())
	}
}

impl PrecomputedHash for Atom
{
	fn precomputed_hash(&self) -> u32
	{
		let mut state = DefaultHasher::new();
		self.0.hash(&mut state);
		state.finish() as u32
	}
}

impl Atom
{
	fn from(value: &str) -> Self
	{
		Atom(value.to_owned())
	}
}
