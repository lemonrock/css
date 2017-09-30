// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// Number of app units per pixel
pub const AppUnitsPerPX: CssFloat = 60.;

/// Number of app units per inch
pub const AppUnitsPerIN: CssFloat = AppUnitsPerPX * 96.;

/// Number of app units per centimeter
pub const AppUnitsPerCM: CssFloat = AppUnitsPerIN / 2.54;

/// Number of app units per millimeter
pub const AppUnitsPerMM: CssFloat = AppUnitsPerIN / 25.4;

/// Number of app units per quarter
pub const AppUnitsPerQ: CssFloat = AppUnitsPerMM / 4.;

/// Number of app units per point
pub const AppUnitsPerPT: CssFloat = AppUnitsPerIN / 72.;

/// Number of app units per pica
pub const AppUnitsPerPC: CssFloat = AppUnitsPerPT * 12.;
