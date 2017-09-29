// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A `@viewport` rule.
#[derive(Clone, Debug, PartialEq)]
pub struct ViewportAtRule
{
	/// The declarations contained in this @viewport rule.
	pub declarations: Vec<ViewportDescriptorDeclaration>
}

impl ViewportAtRule
{
	/// Parse a single @viewport rule.
	pub(crate) fn parse_body<'i, 't, R: ParseErrorReporter>(context: &ParserContext, error_context: &ParserErrorContext<R>, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>>
	{
		let parser = ViewportRuleParser { context };
		
		let mut declarations = Vec::new();
		let mut parser = DeclarationListParser::new(input, parser);
		while let Some(result) = parser.next()
		{
			match result
			{
				Ok(viewportDescriptorDeclarations) =>
				{
					for viewportDescriptorDeclaration in viewportDescriptorDeclarations
					{
						declarations.push(viewportDescriptorDeclaration);
					}
				}
				
				Err(preciseParseError) => return Err(CustomParseError::UnsupportedViewportDescriptorDeclaration(preciseParseError)),
			}
		}
		Ok(ViewportAtRule { declarations })
	}
	
	#[allow(missing_docs)]
	pub(crate) fn from_meta(content: &str) -> Option<ViewportAtRule>
	{
		let mut declarations = vec![None; VIEWPORT_DESCRIPTOR_VARIANTS];
		
		macro_rules! push_descriptor
		{
            ($descriptor:ident($value:expr)) =>
            {
				{
					let descriptor = ViewportDescriptor::$descriptor($value);
					let discriminant = descriptor.discriminant_value();
					declarations[discriminant] = Some(ViewportDescriptorDeclaration::new(descriptor, false));
				}
        	}
        }
		
		let mut has_width = false;
		let mut has_height = false;
		let mut has_zoom = false;
		
		let mut iter = content.chars().enumerate();
		
		macro_rules! start_of_name
		{
            ($iter:ident) => {
                $iter.by_ref().skip_while(|&(_, c)| is_whitespace_separator_or_equals(&c)).next()
            }
        }
		
		while let Some((start, _)) = start_of_name!(iter)
		{
			let property = ViewportAtRule::parse_meta_property(content, &mut iter, start);
			
			if let Some((name, value)) = property
			{
				macro_rules! push
				{
                    ($descriptor:ident($translate:path)) => {
                        if let Some(value) = $translate(value)
                        {
                            push_descriptor!($descriptor(value));
                        }
                    }
                }
				
				match name
				{
					width if width.eq_ignore_ascii_case("width") =>
					{
						if let Some(value) = ViewportLength::from_meta(value)
						{
							push_descriptor!(MinWidth(ViewportLength::ExtendToZoom));
							push_descriptor!(MaxWidth(value));
							has_width = true;
						}
					}
					
					height if height.eq_ignore_ascii_case("height") =>
					{
						if let Some(value) = ViewportLength::from_meta(value)
						{
							push_descriptor!(MinHeight(ViewportLength::ExtendToZoom));
							push_descriptor!(MaxHeight(value));
							has_height = true;
						}
					}
					
					initialScale if initialScale.eq_ignore_ascii_case("initial-scale") =>
					{
						if let Some(value) = Zoom::from_meta(value)
						{
							push_descriptor!(Zoom(value));
							has_zoom = true;
						}
					}
					
					minimumScale if minimumScale.eq_ignore_ascii_case("minimum-scale") =>	push!(MinZoom(Zoom::from_meta)),
					
					maximumScale if maximumScale.eq_ignore_ascii_case("maximum-scale") => push!(MaxZoom(Zoom::from_meta)),
					
					userScalable if userScalable.eq_ignore_ascii_case("user-scalable") => push!(UserZoom(UserZoom::from_meta)),
					
					_ =>
					{
					}
				}
			}
		}
		
		// DEVICE-ADAPT § 9.4 - The 'width' and 'height' properties
		// http://dev.w3.org/csswg/css-device-adapt/#width-and-height-properties
		if !has_width && has_zoom
		{
			if has_height
			{
				push_descriptor!(MinWidth(ViewportLength::Specified(LengthOrPercentageOrAuto::Auto)));
				push_descriptor!(MaxWidth(ViewportLength::Specified(LengthOrPercentageOrAuto::Auto)));
			}
			else
			{
				push_descriptor!(MinWidth(ViewportLength::ExtendToZoom));
				push_descriptor!(MaxWidth(ViewportLength::ExtendToZoom));
			}
		}
		
		let declarations: Vec<_> = declarations.into_iter().filter_map(|entry| entry).collect();
		if declarations.is_empty()
		{
			None
		}
		else
		{
			Some(ViewportAtRule { declarations })
		}
	}
	
	fn parse_meta_property<'a>(content: &'a str, iter: &mut Enumerate<Chars<'a>>, start: usize) -> Option<(&'a str, &'a str)>
	{
		fn end_of_token(iter: &mut Enumerate<Chars>) -> Option<(usize, char)>
		{
			iter.by_ref()
			.skip_while(|&(_, c)| !Self::is_whitespace_separator_or_equals(&c))
			.next()
		}
		
		fn skip_whitespace(iter: &mut Enumerate<Chars>) -> Option<(usize, char)>
		{
			iter.by_ref()
			.skip_while(|&(_, c)| Self::WHITESPACE.contains(&c))
			.next()
		}
		
		// <name> <whitespace>* '='
		let end = match end_of_token(iter)
		{
			Some((end, c)) if Self::WHITESPACE.contains(&c) =>
			{
				match skip_whitespace(iter)
				{
					Some((_, c)) if c == '=' => end,
					_ => return None
				}
			}
			
			Some((end, c)) if c == '=' => end,
			
			_ => return None
		};
		
		let name = &content[start..end];
		
		// <whitespace>* <value>
		let start = match skip_whitespace(iter)
		{
			Some((start, c)) if !Self::SEPARATOR.contains(&c) => start,
			_ => return None
		};
		
		let value = match end_of_token(iter)
		{
			Some((end, _)) => &content[start..end],
			_ => &content[start..]
		};
		
		Some((name, value))
	}
	
	/// Whitespace as defined by DEVICE-ADAPT § 9.2
	// TODO: should we just use whitespace as defined by HTML5?
	const WHITESPACE: &'static [char] = &['\t', '\n', '\r', ' '];
	
	/// Separators as defined by DEVICE-ADAPT § 9.2 need to use \x2c instead of ',' due to test-tidy
	const SEPARATOR: &'static [char] = &['\x2c', ';'];
	
	#[inline]
	fn is_whitespace_separator_or_equals(c: &char) -> bool
	{
		WHITESPACE.contains(c) || SEPARATOR.contains(c) || *c == '='
	}

}

impl ToCss for ViewportAtRule
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		dest.write_str("@viewport { ")?;
		let mut iter = self.declarations.iter();
		iter.next().unwrap().to_css(dest)?;
		for declaration in iter
		{
			dest.write_str(" ")?;
			declaration.to_css(dest)?;
		}
		dest.write_str(" }")
	}
}
