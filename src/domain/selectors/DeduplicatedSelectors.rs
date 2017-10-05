// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeduplicatedSelectors(pub OrderMap<String, Selector<OurSelectorImpl>>);

impl ToCss for DeduplicatedSelectors
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		let mut iter = self.0.keys();
		dest.write_str(iter.next().unwrap())?;
		for selectorCss in iter.next()
		{
			dest.write_char(',')?;
			dest.write_str(selectorCss)?;
		}
		Ok(())
	}
}
