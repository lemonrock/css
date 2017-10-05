// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


pub(crate) struct ViewportAtRuleParser<'a>
{
	pub(crate) context: &'a ParserContext
}

impl<'a, 'i> AtRuleParser<'i> for ViewportAtRuleParser<'a>
{
	type PreludeNoBlock = ();
	
	type PreludeBlock = ();
	
	type AtRule = Vec<ViewportDescriptorDeclaration>;
	
	type Error = CustomParseError<'i>;
}

impl<'a, 'i> DeclarationParser<'i> for ViewportAtRuleParser<'a>
{
	type Declaration = Vec<ViewportDescriptorDeclaration>;
	
	type Error = CustomParseError<'i>;
	
	fn parse_value<'t>(&mut self, name: CowRcStr<'i>, input: &mut Parser<'i, 't>) -> Result<Self::Declaration, ParseError<'i, CustomParseError<'i>>>
	{
		macro_rules! declaration
		{
            ($declaration:ident($parse:expr)) =>
            {
                declaration!($declaration(value: try!($parse(input)), important: input.try(parse_important).is_ok()))
            };
            ($declaration:ident(value: $value:expr, important: $important:expr)) =>
            {
                ViewportDescriptorDeclaration::new(ViewportDescriptor::$declaration($value), $important)
            }
        }
		
		macro_rules! ok
		{
            ($declaration:ident($parse:expr)) =>
            {
                Ok(vec![declaration!($declaration($parse))])
            };
            (shorthand -> [$min:ident, $max:ident]) =>
            {
            	{
                	let shorthand = Self::parse_shorthand(self.context, input)?;
                	let important = input.try(parse_important).is_ok();
					
                	Ok(vec![declaration!($min(value: shorthand.0, important: important)), declaration!($max(value: shorthand.1, important: important))])
            	}
            }
        }
		
		match_ignore_ascii_case!
		{
			&*name,
			
            "min-width" => ok!(MinWidth(|i| ViewportLength::parse(self.context, i))),
            
            "max-width" => ok!(MaxWidth(|i| ViewportLength::parse(self.context, i))),
            
            "width" => ok!(shorthand -> [MinWidth, MaxWidth]),
            
            "min-height" => ok!(MinHeight(|i| ViewportLength::parse(self.context, i))),
            
            "max-height" => ok!(MaxHeight(|i| ViewportLength::parse(self.context, i))),
            
            "height" => ok!(shorthand -> [MinHeight, MaxHeight]),
            
            "zoom" => ok!(Zoom(Zoom::parse)),
            
            "min-zoom" => ok!(MinZoom(Zoom::parse)),
            
            "max-zoom" => ok!(MaxZoom(Zoom::parse)),
            
            "user-zoom" => ok!(UserZoom(UserZoom::parse)),
            
            "orientation" => ok!(Orientation(ViewportOrientation::parse)),
            
            _ => Err(ParseError::Custom(CustomParseError::UnexpectedViewportProperty(name.clone()))),
        }
	}
}

impl<'a: 'a> ViewportAtRuleParser<'a>
{
	fn parse_shorthand<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<(ViewportLength, ViewportLength), ParseError<'i, CustomParseError<'i>>>
	{
		let minimum = ViewportLength::parse(context, input)?;
		match input.try(|i| ViewportLength::parse(context, i))
		{
			Err(_) => Ok((minimum.clone(), minimum)),
			Ok(maximum) => Ok((minimum, maximum))
		}
	}
}
