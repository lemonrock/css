// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


macro_rules! declare_viewport_descriptor
{
    ( $( $variant_name: expr => $variant: ident($data: ident), )+ ) =>
    {
         declare_viewport_descriptor_inner!([] [ $( $variant_name => $variant($data), )+ ] 0);
    };
}

macro_rules! declare_viewport_descriptor_inner
{
    (
        [ $( $assigned_variant_name: expr =>
             $assigned_variant: ident($assigned_data: ident) = $assigned_discriminant: expr, )* ]
        [
            $next_variant_name: expr => $next_variant: ident($next_data: ident),
            $( $variant_name: expr => $variant: ident($data: ident), )*
        ]
        $next_discriminant: expr
    ) => {
        declare_viewport_descriptor_inner!
        {
            [
                $( $assigned_variant_name => $assigned_variant($assigned_data) = $assigned_discriminant, )*
                $next_variant_name => $next_variant($next_data) = $next_discriminant,
            ]
            [ $( $variant_name => $variant($data), )* ]
            $next_discriminant + 1
        }
    };

    (
        [ $( $assigned_variant_name: expr =>
             $assigned_variant: ident($assigned_data: ident) = $assigned_discriminant: expr, )* ]
        [ ]
        $number_of_variants: expr
    ) => {
        #[derive(Clone, Debug, PartialEq)]
        #[allow(missing_docs)]
        pub enum ViewportDescriptor
        {
            $(
                $assigned_variant($assigned_data),
            )+
        }

        impl ViewportDescriptor
        {
            #[allow(missing_docs)]
            pub fn discriminant_value(&self) -> usize
            {
                match *self
                {
                    $(
                        ViewportDescriptor::$assigned_variant(..) => $assigned_discriminant,
                    )*
                }
            }
        }

        impl ToCss for ViewportDescriptor
        {
            fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
            {
                match *self
                {
                    $(
                        ViewportDescriptor::$assigned_variant(ref descriptor_value) =>
                        {
                            dest.write_str($assigned_variant_name)?;
                            dest.write_char(':')?;
                            descriptor_value.to_css(dest)?;
                        },
                    )*
                }
                dest.write_char(';')
            }
        }
    };
}

declare_viewport_descriptor!
{
    "min-width" => MinWidth(ViewportLength),
    "max-width" => MaxWidth(ViewportLength),

    "min-height" => MinHeight(ViewportLength),
    "max-height" => MaxHeight(ViewportLength),

    "zoom" => Zoom(Zoom),
    "min-zoom" => MinZoom(Zoom),
    "max-zoom" => MaxZoom(Zoom),

    "user-zoom" => UserZoom(UserZoom),
    "orientation" => Orientation(ViewportOrientation),
}
