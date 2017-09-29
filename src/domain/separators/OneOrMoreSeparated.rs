// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// Marker trait on T to automatically implement ToCss for Vec<T> when T's are separated by some delimiter `delim`.
pub trait OneOrMoreSeparated
{
	/// Associated type indicating which separator is used.
	type S: Separator;
}

impl<T> ToCss for Vec<T> where T: ToCss + OneOrMoreSeparated
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		let mut iter = self.iter();
		iter.next().unwrap().to_css(dest)?;
		for item in iter
		{
			dest.write_str(<T as OneOrMoreSeparated>::S::separator())?;
			item.to_css(dest)?;
		}
		Ok(())
	}
}
