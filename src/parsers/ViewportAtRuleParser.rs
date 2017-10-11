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
	
	type AtRule = ViewportDescriptorDeclaration;
	
	type Error = CustomParseError<'i>;
}

impl<'a, 'i> DeclarationParser<'i> for ViewportAtRuleParser<'a>
{
	type Declaration = ViewportDescriptorDeclaration;
	
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
                Ok(declaration!($declaration($parse)))
            }
        }
		
		match_ignore_ascii_case!
		{
			&*name,
		
            "min-width" => ok!(MinWidth(|i| ViewportLength::parse(self.context, i))),
            
            "max-width" => ok!(MaxWidth(|i| ViewportLength::parse(self.context, i))),
            
            "width" =>
			{
				let (minimum, maximum) = Self::parse_shorthand_viewport_length(self.context, input)?;
				let important = input.try(parse_important).is_ok();
				Ok(ViewportDescriptorDeclaration::new(ViewportDescriptor::Width { minimum, maximum }, important))
			}
			
            "min-height" => ok!(MinHeight(|i| ViewportLength::parse(self.context, i))),
            
            "max-height" => ok!(MaxHeight(|i| ViewportLength::parse(self.context, i))),
            
            "height" =>
			{
				let (minimum, maximum) = Self::parse_shorthand_viewport_length(self.context, input)?;
				let important = input.try(parse_important).is_ok();
				Ok(ViewportDescriptorDeclaration::new(ViewportDescriptor::Height { minimum, maximum }, important))
			}
            
            "zoom" => ok!(Zoom(|i| Zoom::parse(self.context, i))),
            
            "min-zoom" => ok!(MinZoom(|i| Zoom::parse(self.context, i))),
            
            "max-zoom" => ok!(MaxZoom(|i| Zoom::parse(self.context, i))),
            
            "user-zoom" => ok!(UserZoom(UserZoom::parse)),
            
            "orientation" => ok!(Orientation(ViewportOrientation::parse)),
            
            _ => Err(ParseError::Custom(CustomParseError::UnexpectedViewportProperty(name.clone()))),
        }
	}
}

impl<'a> ViewportAtRuleParser<'a>
{
	fn parse_shorthand_viewport_length<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<(ViewportLength, Option<ViewportLength>), ParseError<'i, CustomParseError<'i>>>
	{
		let minimum = ViewportLength::parse(context, input)?;
		match input.try(|i| ViewportLength::parse(context, i))
		{
			Err(_) => Ok((minimum, None)),
			Ok(maximum) => Ok((minimum, Some(maximum)))
		}
	}
}
