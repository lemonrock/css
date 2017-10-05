// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


pub trait Device
{
	fn documentMatchesUrl(&self, urlMatchingFunction: &UrlMatchingFunction) -> bool;
	
	fn media_type(&self) -> MediaType;
	
	fn viewportWidthInAppUnitsMatches(&self, width: &Range<CalculablePropertyValue<LengthUnit<CssSignedNumber>>>) -> bool;
	
	fn viewportHeightInAppUnitsMatches(&self, height: &Range<CalculablePropertyValue<LengthUnit<CssSignedNumber>>>) -> bool;
	
	fn viewportAspectRatioMatches(&self, ratio: &Range<Ratio>) -> bool;
	
	fn orientationMatches(&self, orientation: MediaOrientation) -> bool;
	
	fn viewportResolutionMatches(&self, resolution: &Range<MediaResolution>) -> bool;
	
	fn scanMatches(&self, scan: &MediaScan) -> bool;
	
	fn gridMatches(&self, grid: &MediaGrid) -> bool;
	
	fn overflowBlockMatches(&self, overflowBlock: &MediaOverflowBlock) -> bool;
	
	fn overflowInlineMatches(&self, overflowInline: &MediaOverflowInline) -> bool;
	
	fn colorBitDepthMatches(&self, colorBitDepth: &Range<ColorBitDepth>) -> bool;
	
	fn colorIndexMatches(&self, colorIndex: &Range<MediaColorIndex>) -> bool;
	
	fn monochromeBitDepthMatches(&self, monochromeBitDepth: &Range<MonochromeBitDepth>) -> bool;
	
	fn colorGamutMatches(&self, colorGamut: &MediaColorGamut) -> bool;
	
	fn pointerMatches(&self, pointer: &MediaPointer) -> bool;
	
	fn hoverMatches(&self, hover: &MediaHover) -> bool;
	
	fn anyPointerMatches(&self, pointer: &MediaPointer) -> bool;
	
	fn anyHoverMatches(&self, hover: &MediaHover) -> bool;
	
	fn transform3DMatches(&self, transform3D: &MediaTransform3D) -> bool;
}
