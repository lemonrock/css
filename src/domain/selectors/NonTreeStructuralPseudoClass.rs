// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


//noinspection SpellCheckingInspection
/// A non tree-structural pseudo-class.
/// See https://drafts.csswg.org/selectors-4/#structural-pseudos
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum NonTreeStructuralPseudoClass
{
	active,
	any(Option<VendorPrefix>, DeduplicatedSelectors),
	any_link(Option<VendorPrefix>),
	checked,
	default,
	dir(Option<VendorPrefix>, TextDirectionality),
	disabled,
	enabled,
	/// Only valid in @page
	first,
	focus,
	focus_within,
	in_range,
	invalid,
	fullscreen(Option<VendorPrefix>),
	hover,
	indeterminate,
	lang(LanguageRanges),
	/// Only valid in @page
	left,
	link,
	optional,
	out_of_range,
	/// The obsolete (as of Firefox 51) `:-moz-placeholder` is re-written when parsed as this.
	placeholder_shown(Option<VendorPrefix>),
	read_only(Option<VendorPrefix>),
	read_write(Option<VendorPrefix>),
	required,
	/// Only valid in @page
	right,
	target,
	valid,
	visited,
	
	/// -servo- only
	case_sensitive_type_attr(Option<VendorPrefix>, Atom),
	
	/// -servo- only
	non_zero_border(Option<VendorPrefix>),

	/// -moz- only
	broken(Option<VendorPrefix>),
	
	/// -moz- only
	drag_over(Option<VendorPrefix>),
	
	/// -moz- only
	first_node(Option<VendorPrefix>),
	
	/// -moz- only
	focusring(Option<VendorPrefix>),
	
	/// -moz- only
	full_screen_ancestor(Option<VendorPrefix>),
	
	/// -moz- only
	handler_blocked(Option<VendorPrefix>),
	
	/// -moz- only
	handler_crashed(Option<VendorPrefix>),
	
	/// -moz- only
	handler_disabled(Option<VendorPrefix>),
	
	/// -moz- only
	last_node(Option<VendorPrefix>),
	
	/// -moz- only
	list_bullet(Option<VendorPrefix>),
	
	/// -moz- only
	list_number(Option<VendorPrefix>),
	
	/// -moz- only
	loading(Option<VendorPrefix>),
	
	//  -moz- only
	locale_dir(Option<VendorPrefix>, TextDirectionality),
	
	/// -moz- only
	lwtheme(Option<VendorPrefix>),
	
	/// -moz- only
	lwtheme_brighttext(Option<VendorPrefix>),
	
	/// -moz- only
	lwtheme_darktext(Option<VendorPrefix>),
	
	/// -moz- only
	native_anonymous(Option<VendorPrefix>),
	
	/// -moz- only
	only_whitespace(Option<VendorPrefix>),
	
	/// -moz- only
	submit_invalid(Option<VendorPrefix>),
	
	/// -moz- only
	suppressed(Option<VendorPrefix>),
	
	/// -moz- only (not listed with other pseudo-classes)
	system_metric(Option<VendorPrefix>, SystemMetric),
	
	/// -moz- only
	tree_cell(Option<VendorPrefix>),
	
	/// -moz- only.
	// A psuedo-class function with one value, hover
	tree_cell_text(Option<VendorPrefix>, TreeHover),
	
	/// -moz- only
	tree_checkbox(Option<VendorPrefix>),
	
	/// -moz- only
	tree_column(Option<VendorPrefix>),
	
	/// -moz- only
	tree_drop_feedback(Option<VendorPrefix>),
	
	/// -moz- only
	tree_image(Option<VendorPrefix>),
	
	/// -moz- only
	tree_indentation(Option<VendorPrefix>),
	
	/// -moz- only
	tree_line(Option<VendorPrefix>),
	
	/// -moz- only
	tree_progressmeter(Option<VendorPrefix>),
	
	/// -moz- only.
	// A psuedo-class function with one value, hover
	tree_row(Option<VendorPrefix>, TreeHover),
	
	/// -moz- only
	tree_separator(Option<VendorPrefix>),
	
	/// -moz- only
	tree_twisty(Option<VendorPrefix>),
	
	/// -moz- only
	ui_invalid(Option<VendorPrefix>),
	
	/// -moz- only
	ui_valid(Option<VendorPrefix>),
	
	/// -moz- only
	user_disabled(Option<VendorPrefix>),
	
	/// -moz- only
	window_inactive(Option<VendorPrefix>),
	
	/// -webkit- only, with potential Mozilla support coming.
	autofill(Option<VendorPrefix>),
}

impl ToCss for NonTreeStructuralPseudoClass
{
	//noinspection SpellCheckingInspection
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
			if let &Some(ref vendorPrefix) = vendorPrefix
			{
				vendorPrefix.to_css(dest)?;
			}
			dest.write_str(classWithoutColon)
		}
		
		#[inline(always)]
		fn write_with_vendor_prefix_value<W: fmt::Write, T: ToCss>(dest: &mut W, vendorPrefix: &Option<VendorPrefix>, classWithoutColon: &str, value: &T) -> fmt::Result
		{
			dest.write_char(':')?;
			if let &Some(ref vendorPrefix) = vendorPrefix
			{
				vendorPrefix.to_css(dest)?;
			}
			dest.write_str(classWithoutColon)?;
			dest.write_char('(')?;
			value.to_css(dest)?;
			dest.write_char(')')
		}
		
		use self::NonTreeStructuralPseudoClass::*;
		use self::VendorPrefix::*;
		
		match *self
		{
			active => write(dest, ":active"),
			
			any(ref vendorPrefix, ref value) => write_with_vendor_prefix_value(dest, vendorPrefix, "any", value),
			
			any_link(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "any-link"),
			
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
				let name = if let &Some(ref vendorPrefix) = vendorPrefix
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
				dest.write_str(name)
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
			
			
			// -servo- only
			
			case_sensitive_type_attr(ref vendorPrefix, ref value) => write_with_vendor_prefix_value(dest, vendorPrefix, "case-sensitive-type-attr", value),
			
			non_zero_border(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "non-zero-border"),
			
			
			// -moz- only
			
			broken(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "broken"),
			
			drag_over(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "drag-over"),
			
			first_node(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "first-node"),
			
			focusring(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "focusring"),
			
			full_screen_ancestor(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "full-screen-ancestor"),
			
			handler_blocked(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "handler-blocked"),
			
			handler_crashed(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "handler-crashed"),
			
			handler_disabled(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "handler-disabled"),
			
			last_node(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "last-node"),
			
			list_bullet(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "list-bullet"),
			
			list_number(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "list-number"),
			
			loading(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "loading"),
			
			locale_dir(ref vendorPrefix, ref value) => write_with_vendor_prefix_value(dest, vendorPrefix, "locale-dir", value),
			
			lwtheme(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "lwtheme"),
			
			lwtheme_brighttext(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "lwtheme-brighttext"),
			
			lwtheme_darktext(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "lwtheme-darktext"),
			
			native_anonymous(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "native-anonymous"),
			
			only_whitespace(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "only-whitespace"),
			
			submit_invalid(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "submit-invalid"),
			
			suppressed(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "suppressed"),
			
			system_metric(ref vendorPrefix, ref value) => write_with_vendor_prefix_value(dest, vendorPrefix, "system-metric", value),
			
			tree_cell(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "tree-cell"),
			
			tree_cell_text(ref vendorPrefix, ref value) => write_with_vendor_prefix_value(dest, vendorPrefix, "tree-cell-text", value),
			
			tree_checkbox(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "tree-checkbox"),
			
			tree_column(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "tree-column"),
			
			tree_drop_feedback(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "tree-drop-feedback"),
			
			tree_image(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "tree-image"),
			
			tree_indentation(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "tree-indentation"),
			
			tree_line(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "tree-line"),
			
			tree_progressmeter(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "tree-progressmeter"),
			
			tree_row(ref vendorPrefix, ref value) => write_with_vendor_prefix_value(dest, vendorPrefix, "tree-row", value),
			
			tree_separator(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "tree-separator"),
			
			tree_twisty(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "tree-twisty"),
			
			ui_invalid(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "ui-invalid"),
			
			ui_valid(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "ui-valid"),
			
			user_disabled(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "user-disabled"),
			
			window_inactive(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "window-inactive"),
			
			autofill(ref vendorPrefix) => write_with_vendor_prefix(dest, vendorPrefix, "autofill"),
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
	
	/// https://drafts.csswg.org/selectors-4/#useraction-pseudos
	///
	/// We intentionally skip the link-related ones.
	pub fn is_safe_user_action_state(&self) -> bool
	{
		use self::NonTreeStructuralPseudoClass::*;
		
		match *self
		{
			active => true,
			focus => true,
			hover => true,
			_ => false,
		}
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub(crate) fn parse_without_arguments<'i>(name: CowRcStr<'i>) -> Result<Self, ParseError<'i, SelectorParseError<'i, CustomParseError<'i>>>>
	{
		use self::NonTreeStructuralPseudoClass::*;
		use self::VendorPrefix::*;
		
		match_ignore_ascii_case!
		{
			&name,
			
			"active" => Ok(active),
			
			"any-link" => Ok(any_link(None)),
			
			"-moz-any-link" => Ok(any_link(Some(moz))),
			
			"-webkit-any-link" => Ok(any_link(Some(webkit))),
			
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
			
			"optional" => Ok(optional),
			
			"out-of-range" => Ok(out_of_range),
			
			"placeholder-shown" => Ok(placeholder_shown(None)),
			
			"-moz-placeholder-shown" => Ok(placeholder_shown(Some(moz))),
			
			// See https://developer.mozilla.org/en-US/docs/Web/CSS/:-moz-placeholder
			"-moz-placeholder" => Ok(placeholder_shown(None)),
			
			"read-only" => Ok(read_only(None)),
			
			"-moz-read-only" => Ok(read_only(Some(moz))),
			
			"read-write" => Ok(read_write(None)),
			
			"-moz-read-write" => Ok(read_write(Some(moz))),
			
			"required" => Ok(required),
			
			"right" => Ok(right),
			
			"scope" => Err(ParseError::Custom(SelectorParseError::Custom(CustomParseError::NonTreeStructuralPseudoClassScopeIsObsoleteAsOfFirefox55))),
			
			"target" => Ok(target),
			
			"valid" => Ok(valid),
			
			"visited" => Ok(visited),
			
			
			// -servo-only
			
			"-servo-non-zero-border" => Ok(non_zero_border(Some(servo))),
			
			
			// -moz- only

			"-moz-broken" => Ok(broken(Some(moz))),
			
			"-moz-drag-over" => Ok(drag_over(Some(moz))),
			
			"-moz-first-node" => Ok(first_node(Some(moz))),
			
			"-moz-focusring" => Ok(focusring(Some(moz))),
			
			"-moz-full-screen-ancestor" => Ok(full_screen_ancestor(Some(moz))),
			
			"-moz-handler-blocked" => Ok(handler_blocked(Some(moz))),
			
			"-moz-handler-crashed" => Ok(handler_crashed(Some(moz))),
			
			"-moz-handler-disabled" => Ok(handler_disabled(Some(moz))),
			
			"-moz-last-node" => Ok(last_node(Some(moz))),
			
			"-moz-list-bullet" => Ok(list_bullet(Some(moz))),
			
			"-moz-list-number" => Ok(list_number(Some(moz))),
			
			"-moz-loading" => Ok(loading(Some(moz))),
			
			"-moz-lwtheme" => Ok(lwtheme(Some(moz))),
			
			"-moz-lwtheme-brighttext" => Ok(lwtheme_brighttext(Some(moz))),
			
			"-moz-lwtheme-darktext" => Ok(lwtheme_darktext(Some(moz))),
			
			"-moz-native-anonymous" => Ok(native_anonymous(Some(moz))),
			
			"-moz-only-whitespace" => Ok(only_whitespace(Some(moz))),
			
			"-moz-submit-invalid" => Ok(submit_invalid(Some(moz))),
			
			"-moz-suppressed" => Ok(suppressed(Some(moz))),
			
			"-moz-tree-cell" => Ok(tree_cell(Some(moz))),
			
			"-moz-tree-checkbox" => Ok(tree_checkbox(Some(moz))),
			
			"-moz-tree-column" => Ok(tree_column(Some(moz))),
			
			"-moz-tree-drop-feedback" => Ok(tree_drop_feedback(Some(moz))),
			
			"-moz-tree-image" => Ok(tree_image(Some(moz))),
			
			"-moz-tree-indentation" => Ok(tree_indentation(Some(moz))),
			
			"-moz-tree-line" => Ok(tree_line(Some(moz))),
			
			"-moz-tree-progressmeter" => Ok(tree_progressmeter(Some(moz))),
			
			"-moz-tree-separator" => Ok(tree_separator(Some(moz))),
			
			"-moz-tree-twisty" => Ok(tree_twisty(Some(moz))),
			
			"-moz-ui-invalid" => Ok(ui_invalid(Some(moz))),
			
			"-moz-ui-valid" => Ok(ui_valid(Some(moz))),
			
			"-moz-user-disabled" => Ok(user_disabled(Some(moz))),
			
			"-moz-window-inactive" => Ok(window_inactive(Some(moz))),
			
			
			// -webkit- only, with potential Mozilla support coming
			
			"-webkit-autofill" => Ok(autofill(Some(webkit))),
			"-moz-autofill" => Ok(autofill(Some(moz))),
			
			
			_ => Err(ParseError::Custom(SelectorParseError::UnsupportedPseudoClassOrElement(name.clone()))),
		}
	}
	
	#[inline(always)]
	pub(crate) fn parse_with_arguments<'i, 't>(name: CowRcStr<'i>, input: &mut Parser<'i, 't>, ourSelectorParser: &OurSelectorParser) -> Result<Self, ParseError<'i, SelectorParseError<'i, CustomParseError<'i>>>>
	{
		use self::NonTreeStructuralPseudoClass::*;
		use self::VendorPrefix::*;
		
		match_ignore_ascii_case!
		{
			&name,
			
			"any" => Ok(any(None, Self::parse_any(input, ourSelectorParser)?)),
			
			"-moz-any" => Ok(any(Some(moz), Self::parse_any(input, ourSelectorParser)?)),
			
			"-webkit-any" => Ok(any(Some(webkit), Self::parse_any(input, ourSelectorParser)?)),
			
			"dir" => Ok(dir(None, Self::parse_text_directionality(input)?)),
			
			"-moz-dir" => Ok(dir(Some(moz), Self::parse_text_directionality(input)?)),
			
			"lang" => Ok(lang(Self::parse_lang(input)?)),
			
			
			// -servo- only
			
			"-servo-case-sensitive-type-attr" => Ok(case_sensitive_type_attr(Some(servo), Atom::from(input.expect_ident()?))),
			
			
			// -moz- only
			
			"-moz-locale-dir" => Ok(locale_dir(Some(moz), Self::parse_text_directionality(input)?)),
			
			"-moz-system-metric" => Ok(system_metric(Some(moz), Self::parse_system_metric(input)?)),
			
			"-moz-tree-cell-text" => Ok(tree_cell_text(Some(moz), Self::parse_tree_hover(input)?)),
			
			"-moz-tree-row" => Ok(tree_row(Some(moz), Self::parse_tree_hover(input)?)),
			
			
			_ => Err(ParseError::Custom(SelectorParseError::UnsupportedPseudoClassOrElement(name.clone()))),
		}
	}
	
	#[inline(always)]
	fn parse_any<'i, 't>(input: &mut Parser<'i, 't>, ourSelectorParser: &OurSelectorParser) -> Result<DeduplicatedSelectors, ParseError<'i, SelectorParseError<'i, CustomParseError<'i>>>>
	{
		use ::cssparser::ParseError::*;
		
		ourSelectorParser.parse_internal(input, OurSelectorExt::is_false_if_any_selector_is_simple_and_only_uses_the_descendant_combinator).map_err(|error|
		{
			match error
			{
				Basic(error) => Basic(error),
				Custom(customParseError) => Custom(SelectorParseError::Custom(customParseError))
			}
		})
	}
	
	#[inline(always)]
	fn parse_text_directionality<'i, 't>(input: &mut Parser<'i, 't>) -> Result<TextDirectionality, ParseError<'i, SelectorParseError<'i, CustomParseError<'i>>>>
	{
		use ::cssparser::ParseError::*;
		
		TextDirectionality::parse(input).map_err(|error|
		{
			match error
			{
				Basic(error) => Basic(error),
				Custom(customParseError) => Custom(SelectorParseError::Custom(customParseError))
			}
		})
	}
	
	#[inline(always)]
	fn parse_system_metric<'i, 't>(input: &mut Parser<'i, 't>) -> Result<SystemMetric, ParseError<'i, SelectorParseError<'i, CustomParseError<'i>>>>
	{
		use ::cssparser::ParseError::*;
		
		SystemMetric::parse(input).map_err(|error|
		{
			match error
			{
				Basic(error) => Basic(error),
				Custom(customParseError) => Custom(SelectorParseError::Custom(customParseError))
			}
		})
	}
	
	#[inline(always)]
	fn parse_tree_hover<'i, 't>(input: &mut Parser<'i, 't>) -> Result<TreeHover, ParseError<'i, SelectorParseError<'i, CustomParseError<'i>>>>
	{
		use ::cssparser::ParseError::*;
		
		TreeHover::parse(input).map_err(|error|
		{
			match error
			{
				Basic(error) => Basic(error),
				Custom(customParseError) => Custom(SelectorParseError::Custom(customParseError))
			}
		})
	}
	
	#[inline(always)]
	fn parse_lang<'i, 't>(input: &mut Parser<'i, 't>) -> Result<LanguageRanges, ParseError<'i, SelectorParseError<'i, CustomParseError<'i>>>>
	{
		// the :lang() pseudo-class represents an element that is in one of the languages listed in its argument. It accepts a comma-separated list of one or more language ranges as its argument. Each language range in :lang() must be a valid CSS <ident> or <string>. (Language ranges containing asterisks, for example, must be quoted as strings.)
		input.parse_comma_separated(|input| Ok(LanguageRange(Atom::from(input.expect_ident_or_string()?.as_ref())))).map(LanguageRanges)
	}
}
