// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A pseudo-element, both public and private.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[allow(missing_docs)]
pub enum PseudoElement
{
	after,
	backdrop(Option<VendorPrefix>),
	before,
	cue,
	first_letter,
	first_line,
	placeholder,
	selection,
	
	// Not standardized and quite variable
	progress_bar(Option<VendorPrefix>),
	range_progress(Option<VendorPrefix>),
	range_thumb(Option<VendorPrefix>),
	range_track(Option<VendorPrefix>),
	
	details_summary(Option<VendorPrefix>),
	details_content(Option<VendorPrefix>),
	text(Option<VendorPrefix>),
	input_text(Option<VendorPrefix>),
	table_wrapper(Option<VendorPrefix>),
	anonymous_table_wrapper(Option<VendorPrefix>),
	anonymous_table(Option<VendorPrefix>),
	anonymous_table_row(Option<VendorPrefix>),
	anonymous_table_cell(Option<VendorPrefix>),
	anonymous_block(Option<VendorPrefix>),
	inline_block_wrapper(Option<VendorPrefix>),
	inline_absolute(Option<VendorPrefix>),
}

impl ToCss for PseudoElement
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		use self::PseudoElement::*;
		
		#[inline(always)]
		fn write<W: fmt::Write>(dest: &mut W, classWithColonColon: &str) -> fmt::Result
		{
			dest.write_str(classWithColonColon)
		}
		
		#[inline(always)]
		fn write_with_vendor_prefix<W: fmt::Write>(dest: &mut W, vendorPrefix: &Option<VendorPrefix>, classWithoutColonColon: &str) -> fmt::Result
		{
			dest.write_str("::")?;
			if let &Some(ref vendorPrefix) = vendorPrefix
			{
				vendorPrefix.to_css(dest)?;
			}
			dest.write_str(classWithoutColonColon)
		}
		
		match *self
		{
			after => write(dest, "::after"),
			
			before => write(dest, "::before"),
			
			first_letter => write(dest, "::first-letter"),
			
			first_line => write(dest, "::first-line"),
			
			placeholder => write(dest, "::placeholder"),
			
			selection => write(dest, "::selection"),
			
			progress_bar(ref vendorPrefix) =>
			{
				match *vendorPrefix
				{
					Some(moz) => write(dest, "::-moz-progress-bar"),
					Some(webkit) => write(dest, "::-webkit-progress-bar"),
					Some(ms) => write(dest, "::-ms-fill"),
					_ => write_with_vendor_prefix(dest, vendorPrefix, "progress-bar"), // almost certainly wrong
				}
			}
			
			range_progress(ref vendorPrefix) =>
			{
				match *vendorPrefix
				{
					Some(moz) => write(dest, "::-moz-range-progress"),
					Some(ms) => write(dest, "::-ms-fill-upper"),
					_ => write_with_vendor_prefix(dest, vendorPrefix, "range-progress"), // almost certainly wrong
				}
			},
			
			range_thumb(ref vendorPrefix) =>
			{
				match *vendorPrefix
				{
					Some(moz) => write(dest, "::-moz-range-thumb"),
					Some(webkit) => write(dest, "::-webkit-slider-thumb"),
					Some(ms) => write(dest, "::-ms-thumb"),
					_ => write_with_vendor_prefix(dest, vendorPrefix, "range-thumb"), // almost certainly wrong
				}
			},
			
			range_track(ref vendorPrefix) =>
			{
				match *vendorPrefix
				{
					Some(moz) => write(dest, "::-moz-range-track"),
					Some(webkit) => write(dest, "::-webkit-slider-runnable-track"),
					Some(ms) => write(dest, "::-ms-track"),
					_ => write_with_vendor_prefix(dest, vendorPrefix, "range-track"), // almost certainly wrong
				}
			},
			
			details_summary(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "details-summary"),
			
			details_content(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "details-content"),
			
			text(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "text"),
			
			input_text(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "input-text"),
			
			table_wrapper(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "table-wrapper"),
			
			anonymous_table_wrapper(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "anonymous-table-wrapper"),
			
			anonymous_table(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "anonymous-table"),
			
			anonymous_table_row(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "anonymous-table-row"),
			
			anonymous_table_cell(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "anonymous-table-cell"),
			
			anonymous_block(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "anonymous-block"),
			
			inline_block_wrapper(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "inline-block-wrapper"),
			
			inline_absolute(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "inline-absolute"),
		}
	}
}

impl ::selectors::parser::PseudoElement for PseudoElement
{
	type Impl = OurSelectorImpl;
	
	fn supports_pseudo_class(&self, pseudo_class: &<Self::Impl as SelectorImpl>::NonTSPseudoClass) -> bool
	{
		if !self.supports_user_action_state()
		{
			return false;
		}
		
		pseudo_class.is_safe_user_action_state()
	}
}

impl PseudoElement
{
	/// Whether this pseudo-element supports user action selectors.
	pub fn supports_user_action_state(&self) -> bool
	{
		use self::PseudoElement::*;
		
		match *self
		{
			after => false,
			before => false,
			backdrop(..) => false,
			cue => false,
			first_letter => false,
			first_line => false,
			progress_bar(..) => true,
			range_track(..) => true,
			range_progress(..) => true,
			range_thumb(..) => true,
			placeholder => true,
			_ => false,
		}
	}
	
	#[inline(always)]
	fn parse_without_arguments<'i>(name: CowRcStr<'i>) -> Result<Self, ParseError<'i, SelectorParseError<'i, CustomParseError>>>
	{
		use self::PseudoElement::*;
		use self::VendorPrefix::*;
		
		match_ignore_ascii_case!
		{
			&name,
			
			"after" => Ok(after),
			
			"backdrop" => Ok(backdrop(None)),
			
			"-ms-backdrop" => Ok(backdrop(Some(ms))),
			
			"-webkit-backdrop" => Ok(backdrop(Some(webkit))),
			
			"before" => Ok(before),
			
			"cue" => Ok(cue),
			
			"first-letter" => Ok(first_letter),
			
			"first-line" => Ok(first_line),
			
			"placeholder" => Ok(placeholder),
			
			"selection" => Ok(selection),
			
			// Doesn't really exist
			"progress-bar" => Ok(progress_bar(None)),
			
			"-moz-progress-bar" => Ok(progress_bar(Some(moz))),
			
			"-webkit-progress-bar" => Ok(progress_bar(Some(webkit))),
			
			"-ms-fill" => Ok(progress_bar(Some(ms))),
			
			// Doesn't really exist
			"range-progress" => Ok(range_progress(None)),
			
			"-moz-range-progress" => Ok(range_progress(Some(moz))),
			
			"-ms-fill-upper" => Ok(range_progress(Some(ms))),
			
			// Doesn't really exist
			"range-thumb" => Ok(range_thumb(None)),
			
			"-moz-range-thumb" => Ok(range_thumb(Some(moz))),
			
			"-webkit-slider-thumb" => Ok(range_thumb(Some(webkit))),
			
			"-ms-thumb" => Ok(range_thumb(Some(ms))),
			
			// Doesn't really exist
			"range-track" => Ok(range_track(None)),
			
			"-moz-range-thumb" => Ok(range_track(Some(moz))),
			
			"-webkit-slider-runnable-track" => Ok(range_track(Some(webkit))),
			
			"-ms-track" => Ok(range_track(Some(ms))),
			
			"-servo-details-summary" => Ok(details_summary(Some(servo))),
			
			"-servo-details-content" => Ok(details_content(Some(servo))),
			
			"-servo-text" => Ok(text(Some(servo))),
			
			"-servo-input-text" => Ok(input_text(Some(servo))),
			
			"-servo-table-wrapper" => Ok(table_wrapper(Some(servo))),
			
			"-servo-anonymous-table-wrapper" => Ok(anonymous_table_wrapper(Some(servo))),
			
			"-servo-anonymous-table" => Ok(anonymous_table(Some(servo))),
			
			"-servo-anonymous-table-row" => Ok(anonymous_table_row(Some(servo))),
			
			"-servo-anonymous-table-cell" => Ok(anonymous_table_cell(Some(servo))),
			
			"-servo-anonymous-block" => Ok(anonymous_block(Some(servo))),
			
			"-servo-inline-block-wrapper" => Ok(inline_block_wrapper(Some(servo))),
			
			"-servo-inline-absolute" => Ok(inline_absolute(Some(servo))),
			
			_ => Err(ParseError::Custom(SelectorParseError::UnsupportedPseudoClassOrElement(name))),
		}
	}
	
	#[inline(always)]
	fn parse_with_arguments<'i, 't>(name: CowRcStr<'i>, _arguments: &mut Parser<'i, 't>, _ourSelectorParser: &OurSelectorParser) -> Result<Self, ParseError<'i, SelectorParseError<'i, CustomParseError<'i>>>>
	{
		Err(ParseError::Custom(SelectorParseError::UnsupportedPseudoClassOrElement(name)))
	}
}
