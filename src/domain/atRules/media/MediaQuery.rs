// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A [media query][mq].
///
/// [mq]: https://drafts.csswg.org/mediaqueries/
#[derive(Clone, Debug, PartialEq)]
pub struct MediaQuery
{
	/// The qualifier for this query.
	pub qualifier: Option<Qualifier>,
	
	/// The media type for this query, that can be known, unknown, or "all".
	pub media_type: MediaQueryType,
	
	/// The set of expressions that this media query contains.
	pub expressions: Vec<Expression>,
}

impl ToCss for MediaQuery
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		if let Some(qualifier) = self.qualifier
		{
			qualifier.to_css(dest)?;
			dest.write_char(' ')?;
		}
		
		use self::MediaQueryType::*;
		
		match self.media_type
		{
			All =>
			{
				// We need to print "all" if there's a qualifier, or there's just an empty list of expressions.
				//
				// Otherwise, we'd serialize media queries like "(min-width: 40px)" in "all (min-width: 40px)", which is unexpected.
				if self.qualifier.is_some() || self.expressions.is_empty()
				{
					dest.write_str("all")?;
				}
			}
			
			Concrete(MediaType(ref desc)) => desc.to_css(dest)?,
		}
		
		if self.expressions.is_empty()
		{
			return Ok(());
		}
		
		if self.media_type != MediaQueryType::All || self.qualifier.is_some()
		{
			dest.write_str(" and ")?;
		}
		
		self.expressions[0].to_css(dest)?;
		
		for expr in self.expressions.iter().skip(1)
		{
			dest.write_str(" and ")?;
			expr.to_css(dest)?;
		}
		
		Ok(())
	}
}

impl MediaQuery
{
	/// Return a media query that never matches, used for when we fail to parse a given media query.
	#[inline(always)]
	fn never_matching() -> Self
	{
		Self::new(Some(Qualifier::Not), MediaQueryType::All, vec![])
	}
	
	/// Trivially constructs a new media query.
	#[inline(always)]
	pub fn new(qualifier: Option<Qualifier>, media_type: MediaQueryType, expressions: Vec<Expression>) -> Self
	{
		Self
		{
			qualifier,
			media_type,
			expressions,
		}
	}
	
	/// Parse a media query given css input.
	///
	/// Returns an error if any of the expressions is unknown.
	pub(crate) fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<MediaQuery, ParseError<'i>>
	{
		let mut expressions = vec![];
		
		use self::Qualifier::*;
		
		let qualifier = if input.try(|input| input.expect_ident_matching("only")).is_ok()
		{
			Some(Only)
		}
		else if input.try(|input| input.expect_ident_matching("not")).is_ok()
		{
			Some(Not)
		}
		else
		{
			None
		};
		
		let media_type = match input.try(|i| i.expect_ident_cloned())
		{
			Ok(ident) =>
			{
				let result: Result<_, ParseError> = MediaQueryType::parse(&*ident).map_err(|()| SelectorParseError::UnexpectedIdent(ident.clone()).into());
				result?
			}
			
			Err(_) =>
			{
				// Media type is only optional if qualifier is not specified.
				if qualifier.is_some()
				{
					return Err(StyleParseError::UnspecifiedError.into())
				}
				
				// Without a media type, require at least one expression.
				expressions.push(Expression::parse(context, input)?);
				
				MediaQueryType::All
			}
		};
		
		// Parse any subsequent expressions
		loop
		{
			if input.try(|input| input.expect_ident_matching("and")).is_err()
			{
				return Ok(Self::new(qualifier, media_type, expressions))
			}
			expressions.push(Expression::parse(context, input)?)
		}
	}
}
