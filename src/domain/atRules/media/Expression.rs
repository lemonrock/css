// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// A single expression as per http://dev.w3.org/csswg/mediaqueries-3/#media1
#[derive(Clone, Debug, PartialEq)]
pub struct Expression(pub ExpressionKind);

impl ToCss for Expression
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		use self::ExpressionKind::*;
		use self::Range::*;
		
		#[inline(always)]
		fn write<W: fmt::Write, T: ToCss>(dest: &mut W, name: &str, value: &T) -> fmt::Result
		{
			dest.write_char('(')?;
			dest.write_str(name)?;
			dest.write_char(':')?;
			value.to_css(dest)?;
			dest.write_char(')')
		}
		
		match self.0
		{
			Width(AtLeast(ref value)) => write(dest, "min-width", value),
			
			Width(AtMost(ref value)) => write(dest, "max-width", value),
			
			Width(Exact(ref value)) => write(dest, "width", value),
			
			Height(AtLeast(ref value)) => write(dest, "min-height", value),
			
			Height(AtMost(ref value)) => write(dest, "max-height", value),
			
			Height(Exact(ref value)) => write(dest, "height", value),
			
			AspectRatio(AtLeast(ref value)) => write(dest, "min-aspect-ratio", value),
			
			AspectRatio(AtMost(ref value)) => write(dest, "max-aspect-ratio", value),
			
			AspectRatio(Exact(ref value)) => write(dest, "aspect-ratio", value),
			
			Orientation(ref value) => write(dest, "orientation", value),
			
			Resolution(AtLeast(ref value)) => write(dest, "min-resolution", value),
			
			Resolution(AtMost(ref value)) => write(dest, "max-resolution", value),
			
			Resolution(Exact(ref value)) => write(dest, "resolution", value),
			
			Scan(ref value) => write(dest, "scan", value),
			
			Grid(ref value) => write(dest, "grid", value),
			
			Update(ref value) => write(dest, "update", value),
			
			OverflowBlock(ref value) => write(dest, "overflow-block", value),
			
			OverflowInline(ref value) => write(dest, "overflow-inline", value),
			
			Color(AtLeast(ref value)) => write(dest, "min-color", value),
			
			Color(AtMost(ref value)) => write(dest, "max-color", value),
			
			Color(Exact(ref value)) => write(dest, "color", value),
			
			Color(AtLeast(ref value)) => write(dest, "min-color-index", value),
			
			Color(AtMost(ref value)) => write(dest, "max-color-index", value),
			
			Color(Exact(ref value)) => write(dest, "color-index", value),
			
			Monochrome(AtLeast(ref value)) => write(dest, "min-monochrome", value),
			
			Monochrome(AtMost(ref value)) => write(dest, "max-monochrome", value),
			
			Monochrome(Exact(ref value)) => write(dest, "monochrome", value),
			
			ColorGamut(ref value) => write(dest, "color-gamut", value),
			
			Pointer(ref value) => write(dest, "pointer", value),
			
			Hover(ref value) => write(dest, "hover", value),
			
			AnyPointer(ref value) => write(dest, "any-pointer", value),
			
			AnyHover(ref value) => write(dest, "any-hover", value),
			
			Transform3D(ref value) => write(dest, "-webkit-transform-3d", value),
		}
	}
}

impl Expression
{
	pub(crate) fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		use self::ExpressionKind::*;
		use self::Range::*;
		
		input.expect_parenthesis_block()?;
		input.parse_nested_block(|input|
		{
			let name = input.expect_ident_cloned()?;
			input.expect_colon()?;
			
			Ok
			(
				Expression
				(
					match_ignore_ascii_case!
					{
						&name,
						
						"min-width" => Width(AtLeast(LengthUnit::parse_non_negative(context, input)?)),
						
						"max-width" => Width(AtMost(LengthUnit::parse_non_negative(context, input)?)),
						
						"width" => Width(Exact(LengthUnit::parse_non_negative(context, input)?)),
						
						"min-height" => Height(AtLeast(LengthUnit::parse_non_negative(context, input)?)),
						
						"max-height" => Height(AtMost(LengthUnit::parse_non_negative(context, input)?)),
						
						"height" => Height(Exact(LengthUnit::parse_non_negative(context, input)?)),
						
						"min-aspect-ratio" => AspectRatio(AtLeast(Ratio::parse(context, input)?)),
						
						"max-aspect-ratio" => AspectRatio(AtMost(Ratio::parse(context, input)?)),
						
						"aspect-ratio" => AspectRatio(Exact(Ratio::parse(context, input)?)),
						
						"orientation" => Orientation(MediaOrientation::parse(input)?),
						
						"min-resolution" => Resolution(AtLeast(MediaResolution::parse(context, input)?)),
						
						"max-resolution" => Resolution(AtLeast(MediaResolution::parse(context, input)?)),
						
						"resolution" => Resolution(Exact(MediaResolution::parse(context, input)?)),
						
						"-webkit-min-device-pixel-ratio" => Resolution(AtLeast(MediaResolution::parseWebKit(input)?)),
						
						"-webkit-max-device-pixel-ratio" => Resolution(AtMost(MediaResolution::parseWebKit(input)?)),
						
						"-webkit-device-pixel-ratio" => Resolution(Exact(MediaResolution::parseWebKit(input)?)),
						
						"scan" => Scan(MediaScan::parse(input)?),
						
						"grid" => Grid(MediaGrid::parse(context, input)?),
						
						"update" => Update(MediaUpdate::parse(input)?),
						
						"overflow-block" => OverflowBlock(MediaOverflowBlock::parse(input)?),
						
						"overflow-inline" => OverflowInline(MediaOverflowInline::parse(input)?),
						
						"min-color" => Color(AtLeast(ColorBitDepth::parse(context, input)?)),
						
						"max-color" => Color(AtMost(ColorBitDepth::parse(context, input)?)),
						
						"color" => Color(Exact(ColorBitDepth::parse(context, input)?)),
						
						"min-color-index" => ColorIndex(AtLeast(MediaColorIndex::parse(context, input)?)),
						
						"max-color-index" => ColorIndex(AtMost(MediaColorIndex::parse(context, input)?)),
						
						"color-index" => ColorIndex(Exact(MediaColorIndex::parse(context, input)?)),
						
						"color-gamut" => ColorGamut(MediaColorGamut::parse(input)?),
						
						"pointer" => Pointer(MediaPointer::parse(input)?),
						
						"hover" => Hover(MediaHover::parse(input)?),
						
						"any-pointer" => AnyPointer(MediaPointer::parse(input)?),
						
						"any-hover" => AnyHover(MediaHover::parse(input)?),
						
						"-webkit-transform-3d" => Transform3D(MediaTransform3D::parse(context, input)?),
						
						"min-device-width" | "min-device-width" | "device-width" | "min-device-height" | "min-device-height" | "device-height" | "min-device-aspect-ratio" | "min-device-aspect-ratio" | "device-aspect-ratio" => return Err(ParseError::Custom(CustomParseError::DeprecatedMediaQueryExpression(name.clone()))),
						
						_ => return Err(ParseError::Custom(CustomParseError::UnsupportedMediaQueryExpression(name.clone())))
					}
				)
			)
		})
	}
	
	/// Evaluate this expression and return whether it matches the current device.
	pub fn matches<D: Device>(&self, device: &D) -> bool
	{
		use self::ExpressionKind::*;
		
		match self.0
		{
			Width(ref range) => device.viewportWidthInAppUnitsMatches(range),
			Height(ref range) => device.viewportHeightInAppUnitsMatches(range),
			AspectRatio(ref range) => device.viewportAspectRatioMatches(range),
			Orientation(orientation) => device.orientationMatches(orientation),
			Resolution(ref range) => device.viewportResolutionMatches(range),
			_ => unimplemented!("Please implement matches for remaining behaviours")
		}
	}
}
