// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


pub trait Length: Sized
+ Copy + Clone
+ PartialEq<Self> + Eq + PartialOrd + Ord + Hash
+ ToCss
+ Default
+ Add<Self, Output=Self> + AddAssign<Self>
+ Sub<Self, Output=Self> + SubAssign<Self>
+ Mul<Self, Output=Self> + MulAssign<Self>
+ Div<Self, Output=Self> + DivAssign<Self>
+ Rem<Self, Output=Self> + RemAssign<Self>
+ Neg<Output=Self>
+ Debug
+ CssNumberNewType + Unit
{
	type Number: CssNumber;
}
