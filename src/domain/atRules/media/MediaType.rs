// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// https://drafts.csswg.org/mediaqueries/#media-types
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaType(pub CustomIdent);

impl MediaType
{
	/// The `screen` media type.
	#[inline(always)]
	pub fn screen() -> Self
	{
		MediaType(CustomIdent(Atom::from("screen")))
	}
	
	/// The `print` media type.
	#[inline(always)]
	pub fn print() -> Self
	{
		MediaType(CustomIdent(Atom::from("print")))
	}
	
	fn parse(name: &str) -> Result<Self, ()>
	{
		// From https://drafts.csswg.org/mediaqueries/#mq-syntax:
		//
		//   The <media-type> production does not include the keywords not, or, and, and only.
		//
		// Here we also perform the to-ascii-lowercase part of the serialization algorithm: https://drafts.csswg.org/cssom/#serializing-media-queries
		match_ignore_ascii_case!
		{
			name,
            "not" | "or" | "and" | "only" => Err(()),
            _ => Ok(MediaType(CustomIdent(Atom(name.to_ascii_lowercase())))),
        }
	}
}
