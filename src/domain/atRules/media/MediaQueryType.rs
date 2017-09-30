// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// http://dev.w3.org/csswg/mediaqueries-3/#media0
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum MediaQueryType
{
	/// A media type that matches every device.
	All,
	
	/// A specific media type.
	Concrete(MediaType),
}

impl MediaQueryType
{
	fn parse<'i>(ident: &str) -> Result<Self, CustomParseError<'i>>
	{
		use self::MediaQueryType::*;
		
		match_ignore_ascii_case!
		{
			ident,
			
            "all" => Ok(All),
            
            _ => MediaType::parse(ident).map(Concrete),
        };
	}
	
	fn matches(&self, other: MediaType) -> bool
	{
		use self::MediaQueryType::*;
		
		match *self
		{
			All => true,
			Concrete(ref known_type) => *known_type == other,
		}
	}
}
