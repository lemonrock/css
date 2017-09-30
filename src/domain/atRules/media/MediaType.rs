// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// https://drafts.csswg.org/mediaqueries/#media-types
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum MediaType
{
	print,
	screen,
	speech,
}

impl ToCss for MediaType
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		use self::MediaType::*;
		
		let identifier = match *self
		{
			print => "print",
			screen => "screen",
			speech => "speech",
		};
		
		dest.write_str(identifier)
	}
}

impl MediaType
{
	fn parse<'i>(ident: &str) -> Result<Self, CustomParseError<'i>>
	{
		use self::MediaType::*;
		
		// From https://drafts.csswg.org/mediaqueries/#mq-syntax:
		//
		//   The <media-type> production does not include the keywords not, or, and, and only.
		//
		// Here we also perform the to-ascii-lowercase part of the serialization algorithm: https://drafts.csswg.org/cssom/#serializing-media-queries
		match_ignore_ascii_case!
		{
			ident,
			
			"print" => Ok(print),
			
			"screen" => Ok(screen),
			
			"speech" => Ok(speech),
			
			"aural" => Ok(speech),
			
			"tty" | "tv" | "projection" | "handheld" | "braille" | "embossed" | "3d-glasses" => Err(CustomParseError::DeprecatedMediaType(ident.to_owned())),
			
            "not" | "or" | "and" | "only" => Err(CustomParseError::InvalidMediaType(ident.to_owned())),
            
            _ => Err(CustomParseError::UnrecognisedMediaType(ident.to_owned())),
        }
	}
}
