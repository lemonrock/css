// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A URL matching function for a `@document` rule's condition.
#[derive(Clone, Debug)]
pub enum UrlMatchingFunction
{
	/// Exact URL matching function.
	/// It evaluates to true whenever the URL of the document being styled is exactly the URL given.
	Url(SpecifiedUrl),
	
	/// URL prefix matching function.
	/// It evaluates to true whenever the URL of the document being styled has the argument to the function as an initial substring (which is true when the two strings are equal).
	/// When the argument is the empty string, it evaluates to true for all documents.
	UrlPrefix(String),
	
	/// Domain matching function.
	/// It evaluates to true whenever the URL of the document being styled has a host subcomponent and that host subcomponent is exactly the argument to the ‘domain()’ function or a final substring of the host component is a  period (U+002E) immediately followed by the argument to the ‘domain()’ function.
	Domain(String),
	
	/// Regular expression matching function.
	/// It evaluates to true whenever the regular expression matches the entirety of the URL of the document being styled.
	RegExp(String),
}

macro_rules! parse_quoted_or_unquoted_string
{
    ($input:ident, $url_matching_function:expr) =>
    {
        $input.parse_nested_block(|input|
        {
            let start = input.position();
            input.parse_entirely(|input|
            {
                match input.next()
                {
                    Ok(&Token::QuotedString(ref value)) => Ok($url_matching_function(value.as_ref().to_owned())),
                    Ok(t) => Err(BasicParseError::UnexpectedToken(t.clone()).into()),
                    Err(e) => Err(e.into()),
                }
            }).or_else(|_: ParseError|
            {
                while let Ok(_) = input.next()
                {
                }
                
                Ok($url_matching_function(input.slice_from(start).to_string()))
            })
        })
    }
}

impl ToCss for UrlMatchingFunction
{
	fn to_css<W: fmt::Write >(&self, dest: &mut W) -> fmt::Result
	{
		match *self
		{
			UrlMatchingFunction::Url(ref url) => url.to_css(dest),
			
			UrlMatchingFunction::UrlPrefix(ref url_prefix) =>
				{
					dest.write_str("url-prefix(")?;
					url_prefix.to_css(dest)?;
					dest.write_str(")")
				},
			
			UrlMatchingFunction::Domain(ref domain) =>
				{
					dest.write_str("domain(")?;
					domain.to_css(dest)?;
					dest.write_str(")")
				},
			
			UrlMatchingFunction::RegExp(ref regex) =>
				{
					dest.write_str("regexp(")?;
					regex.to_css(dest)?;
					dest.write_str(")")
				},
		}
	}
}

impl UrlMatchingFunction
{
	/// Parse a URL matching function for a `@document` rule's condition.
	pub(crate) fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<UrlMatchingFunction, ParseError<'i>>
	{
		if input.try(|input| input.expect_function_matching("url-prefix")).is_ok()
		{
			parse_quoted_or_unquoted_string!(input, UrlMatchingFunction::UrlPrefix)
		}
		else if input.try(|input| input.expect_function_matching("domain")).is_ok()
		{
			parse_quoted_or_unquoted_string!(input, UrlMatchingFunction::Domain)
		}
		else if input.try(|input| input.expect_function_matching("regexp")).is_ok()
		{
			input.parse_nested_block(|input|
			{
				Ok(UrlMatchingFunction::RegExp(input.expect_string()?.as_ref().to_owned()))
			})
		}
		else if let Ok(url) = input.try(|input| SpecifiedUrl::parse(context, input))
		{
			Ok(UrlMatchingFunction::Url(url))
		}
		else
		{
			Err(StyleParseError::UnspecifiedError.into())
		}
	}
	
	/// Evaluate a document condition.
	pub fn evaluate<D: Device>(&self, device: &D) -> bool
	{
		device.documentMatchesUrl(self)
	}
}
