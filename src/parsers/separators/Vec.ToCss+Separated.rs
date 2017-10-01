// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


impl<T: ToCss + Separated> ToCss for Vec<T>
{
	fn to_css<W: Write>(&self, dest: &mut W) -> fmt::Result
	{
		if self.is_empty()
		{
			return Ok(());
		}
		
		let mut iterator = self.iter();
		iterator.next().unwrap().to_css(dest)?;
		
		for item in iterator
		{
			dest.write_str(<T as Separated>::Delimiter::separator())?;
			item.to_css(dest)?;
		}
		
		Ok(())
	}
}
