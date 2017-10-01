// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A non tree-structural pseudo-class.
/// See https://drafts.csswg.org/selectors-4/#structural-pseudos
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[allow(missing_docs)]
pub enum NonTreeStructuralPseudoClass
{
	active,
	any(Option<VendorPrefix>, DeduplicatedSelectors),
	any_link(Option<VendorPrefix>),
	case_sensitive_type_attr(Option<VendorPrefix>, Atom),
	checked,
	default,
	dir(Option<VendorPrefix>, TextDirectionality),
	disabled,
	enabled,
	first, // Only valid in @page
	focus,
	focus_within,
	in_range,
	invalid,
	fullscreen(Option<VendorPrefix>),
	hover,
	indeterminate,
	lang(LanguageRanges),
	left, // Only valid in @page
	link,
	non_zero_border(Option<VendorPrefix>),
	optional,
	out_of_range,
	placeholder_shown(Option<VendorPrefix>),
	read_only(Option<VendorPrefix>),
	read_write(Option<VendorPrefix>),
	required,
	right, // Only valid in @page
	target,
	valid,
	visited,
}

impl ToCss for NonTreeStructuralPseudoClass
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		#[inline(always)]
		fn write<W: fmt::Write>(dest: &mut W, classWithColon: &str) -> fmt::Result
		{
			dest.write_str(classWithColon)
		}
		
		#[inline(always)]
		fn write_with_vendor_prefix<W: fmt::Write>(dest: &mut W, vendorPrefix: &Option<VendorPrefix>, classWithoutColon: &str) -> fmt::Result
		{
			dest.write_char(':')?;
			if let Some(vendorPrefix) = vendorPrefix
			{
				vendorPrefix.to_css(dest)?;
			}
			dest.write_str(classWithoutColon)
		}
		
		#[inline(always)]
		fn write_with_vendor_prefix_value<W: fmt::Write, T: ToCss>(dest: &mut W, vendorPrefix: &Option<VendorPrefix>, classWithoutColon: &str, value: &T) -> fmt::Result
		{
			dest.write_char(':')?;
			if let Some(vendorPrefix) = vendorPrefix
			{
				vendorPrefix.to_css(dest)?;
			}
			dest.write_str(classWithoutColon)?;
			dest.write_char('(')?;
			value.to_css(dest)?;
			dest.write_char(')')
		}
		
		use self::NonTreeStructuralPseudoClass::*;
		
		match *self
		{
			active => write(dest, ":active"),
			
			any(ref vendorPrefix, ref value) => write_with_vendor_prefix_value(dest, vendorPrefix, "any", value),
			
			any_link(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "any-link"),
			
			case_sensitive_type_attr(ref vendorPrefix, ref value) => write_with_vendor_prefix_value(dest, vendorPrefix, "case-sensitive-type-attr", value),
			
			checked => write(dest, ":checked"),
			
			default => write(dest, ":default"),
			
			disabled => write(dest, ":disabled"),
			
			dir(ref vendorPrefix, ref value) => write_with_vendor_prefix_value(dest, vendorPrefix, "dir", value),
			
			enabled => write(dest, ":enabled"),
			
			first => write(dest, ":first"),
			
			focus => write(dest, ":focus"),
			
			focus_within => write(dest, ":focus-within"),
			
			fullscreen(ref vendorPrefix) =>
			{
				dest.write_char(':')?;
				let name = if let Some(ref vendorPrefix) = vendorPrefix
				{
					vendorPrefix.to_css(dest)?;
					
					match *vendorPrefix
					{
						moz => "full-screen",
						webkit => "full-screen",
						_ => "fullscreen",
					}
				}
				else
				{
					"fullscreen"
				};
				dest.write_str(name)?;
			}
			
			hover => write(dest, ":hover"),
			
			indeterminate => write(dest, ":indeterminate"),
			
			in_range => write(dest, ":in-range"),
			
			invalid => write(dest, ":invalid"),
			
			lang(ref languages) =>
			{
				dest.write_str(":lang(")?;
				languages.to_css(dest)?;
				dest.write_char(')')
			}
			
			left => write(dest, ":left"),
			
			link => write(dest, ":link"),
			
			non_zero_border(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "non-zero-border"),
			
			optional => write(dest, ":optional"),
			
			out_of_range => write(dest, ":out-of-range"),
			
			placeholder_shown(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "placeholder-shown"),
			
			read_only(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "read-only"),
			
			read_write(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "read-write"),
			
			required => write(dest, ":required"),
			
			right => write(dest, ":right"),
			
			target => write(dest, ":target"),
			
			valid => write(dest, ":valid"),
			
			visited => write(dest, ":visited"),
		}
	}
}

impl SelectorMethods for NonTreeStructuralPseudoClass
{
	type Impl = OurSelectorImpl;
	
	fn visit<V: SelectorVisitor<Impl = Self::Impl>>(&self, _visitor: &mut V) -> bool
	{
		true
	}
}

impl NonTreeStructuralPseudoClass
{
	/// Returns true if the evaluation of the pseudo-class depends on the element's attributes.
	pub fn is_attr_based(&self) -> bool
	{
		use self::NonTreeStructuralPseudoClass::*;
		
		match *self
		{
			lang(..) => true,
			_ => false,
		}
	}
	
	#[inline(always)]
	fn parse_without_arguments(&self, name: Cow<str>) -> Result<Self, ()>
	{
		use self::NonTreeStructuralPseudoClass::*;
		use self::VendorPrefix::*;
		
		match_ignore_ascii_case!
		{
			&name,
			
			"active" => Ok(active),
			
			"any-link" => Ok(None, any_link),
			
			"-moz-any-link" => Ok(Some(moz), any_link),
			
			"-webkit-any-link" => Ok(Some(webkit), any_link),
			
			"checked" => Ok(checked),
			
			"default" => Ok(default),
			
			"disabled" => Ok(disabled),
			
			"enabled" => Ok(enabled),
			
			"first" => Ok(first),
			
			"focus" => Ok(focus),
			
			"focus-within" => Ok(focus_within),
			
			"fullscreen" => Ok(fullscreen(None)),
			
			"-ms-fullscreen" => Ok(fullscreen(Some(ms))),
			
			"-moz-full-screen" => Ok(fullscreen(Some(moz))),
			
			"-webkit-full-screen" => Ok(fullscreen(Some(webkit))),
			
			"hover" => Ok(hover),
			
			"indeterminate" => Ok(indeterminate),
			
			"in-range" => Ok(in_range),
			
			"invalid" => Ok(invalid),
			
			"left" => Ok(left),
			
			"link" => Ok(link),
			
			"-servo-non-zero-border" => Ok(non_zero_border(Some(servo))),
			
			"optional" => Ok(optional),
			
			"out-of-range" => Ok(out_of_range),
			
			"placeholder-shown" => Ok(placeholder_shown(None)),
			
			"-moz-placeholder-shown" => Ok(placeholder_shown(Some(moz))),
			
			"read-only" => Ok(read_only(None)),
			
			"-moz-read-only" => Ok(read_only(Some(moz))),
			
			"read-write" => Ok(read_write(None)),
			
			"-moz-read-write" => Ok(read_write(Some(moz))),
			
			"required" => Ok(required),
			
			"right" => Ok(right),
			
			"scope" => Err(()), // scope is obsolete as of Firefox 55
			
			"target" => Ok(target),
			
			"valid" => Ok(valid),
			
			"visited" => Ok(visited),
			
			_ => Err(()),
		}
	}
	
	#[inline(always)]
	fn parse_with_arguments<'i, 't>(&self, name: Cow<str>, input: &mut Parser<'i, 't>, ourSelectorParser: &OurSelectorParser) -> Result<Self, ()>
	{
		use self::NonTreeStructuralPseudoClass::*;
		use self::VendorPrefix::*;
		
		match_ignore_ascii_case!
		{
			&name,
			
			"any" => Ok(any(None, Self::parse_any(input)?)),
			
			"-moz-any" => Ok(any(Some(moz), Self::parse_any(input, ourSelectorParser)?)),
			
			"-webkit-any" => Ok(any(Some(webkit), Self::parse_any(input)?)),
			
			"-servo-case-sensitive-type-attr" => Ok(Some(servo), case_sensitive_type_attr(Atom::from(input.expect_ident()?.as_ref()))),
			
			"dir" => Ok(dir(None, Self::parse_text_directionality(input)?)),
			
			"-moz-dir" => Ok(dir(Some(moz), Self::parse_text_directionality(input)?)),
			
			"lang" => Ok(lang(Self::parse_lang(input)?)),
			
			_ => Err(()),
		}
	}
	
	#[inline(always)]
	fn parse_any<'i, 't>(input: &mut Parser<'i, 't>, ourSelectorParser: &OurSelectorParser) -> Result<DeduplicatedSelectors, ()>
	{
		ourSelectorParser.parse_internal(input, OurSelectorExt::is_false_if_any_selector_is_simple_and_only_uses_the_descendant_combinator).map_err(|_| ())
	}
	
	#[inline(always)]
	fn parse_text_directionality<'i, 't>(input: &mut Parser<'i, 't>) -> Result<TextDirectionality, ()>
	{
		TextDirectionality::parse(input).map_err(|_| ())
	}
	
	#[inline(always)]
	fn parse_lang<'i, 't>(input: Parser<'i, 't>) -> Result<LanguageRanges, ()>
	{
		// the :lang() pseudo-class represents an element that is in one of the languages listed in its argument. It accepts a comma-separated list of one or more language ranges as its argument. Each language range in :lang() must be a valid CSS <ident> or <string>. (Language ranges containing asterisks, for example, must be quoted as strings.)
		input.parse_comma_separated(|input| Ok(LanguageRange(Atom::from(input.expect_ident_or_string()?.as_ref())))).map(LanguageRanges).map_err(|_| ())
	}
}
