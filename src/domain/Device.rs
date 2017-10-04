// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


pub trait Device
{
	fn media_type(&self) -> MediaType;
	
	fn documentMatchesUrl(&self, urlMatchingFunction: &UrlMatchingFunction) -> bool;
	
	fn viewportWidthInAppUnitsMatches(&self, width: &Range<CalculablePropertyValue<LengthUnit<CssSignedNumber>>>) -> bool;
	
	fn viewportHeightInAppUnitsMatches(&self, width: &Range<CalculablePropertyValue<LengthUnit<CssSignedNumber>>>) -> bool;
	
	fn viewportAspectRatioMatches(&self, width: &Range<Ratio>) -> bool;
	
	fn orientationMatches(&self, orientation: MediaOrientation) -> bool;
	
	fn viewportResolutionMatches(&self, width: &Range<MediaResolution>) -> bool;
}
