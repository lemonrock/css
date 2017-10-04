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
	pub(crate) fn parse_body<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		let parser = ViewportAtRuleParser { context };
		
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
				
				Err(preciseParseError) => return Err(preciseParseError.error),
			}
		}
		Ok(ViewportAtRule { declarations })
	}
	
	fn parse_meta_property<'a>(content: &'a str, iter: &mut Enumerate<Chars<'a>>, start: usize) -> Option<(&'a str, &'a str)>
	{
		fn end_of_token(iter: &mut Enumerate<Chars>) -> Option<(usize, char)>
		{
			iter.by_ref().skip_while(|&(_, c)| !ViewportAtRule::is_whitespace_separator_or_equals(&c)).next()
		}
		
		fn skip_whitespace(iter: &mut Enumerate<Chars>) -> Option<(usize, char)>
		{
			iter.by_ref().skip_while(|&(_, c)| ViewportAtRule::WHITESPACE.contains(&c)).next()
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
		
		let slice = match end_of_token(iter)
		{
			Some((end, _)) => &content[start..end],
			_ => &content[start..]
		};
		
		Some((name, slice))
	}
	
	/// Whitespace as defined by DEVICE-ADAPT § 9.2
	const WHITESPACE: &'static [char] = &['\t', '\n', '\r', ' '];
	
	/// Separators as defined by DEVICE-ADAPT § 9.2 need to use \x2c instead of ',' due to test-tidy
	const SEPARATOR: &'static [char] = &['\x2c', ';'];
	
	#[inline]
	fn is_whitespace_separator_or_equals(c: &char) -> bool
	{
		Self::WHITESPACE.contains(c) || Self::SEPARATOR.contains(c) || *c == '='
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
