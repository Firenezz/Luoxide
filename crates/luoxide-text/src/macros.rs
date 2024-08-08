
macro_rules! operator {
    (impl $op:ident for $struct_name:ident by fn $function:ident = $operator:tt) => {
        impl $op<$struct_name> for $struct_name {
            type Output = $struct_name;

            #[inline]
            fn $function(self, rhs: $struct_name) -> Self::Output {
                $struct_name { raw: self.raw $operator rhs.raw }
            }
        }

        impl $op<&$struct_name> for $struct_name {
            type Output = $struct_name;

            #[inline]
            fn $function(self, rhs: &$struct_name) -> Self::Output {
                self $operator *rhs
            }
        }

        impl<T> $op<T> for &$struct_name
        where
            $struct_name: $op<T, Output = $struct_name>
        {
            type Output = $struct_name;

            #[inline]
            fn $function(self, rhs: T) -> Self::Output {
                *self $operator rhs
            }
        }
    };

    (impl $op:ident for $struct_name:ident by fn $function:ident = $operator:tt $( $field:ident ),*) => {
        impl $op<$struct_name> for $struct_name {
            type Output = $struct_name;

            #[inline]
            fn $function(self, rhs: $struct_name) -> Self::Output {
                $struct_name {
                    $($field: self.$field $operator rhs.$field),*
                }
            }
        }

        impl $op<&$struct_name> for $struct_name {
            type Output = $struct_name;

            #[inline]
            fn $function(self, rhs: &$struct_name) -> Self::Output {
                self $operator *rhs
            }
        }

        impl<T> $op<T> for &$struct_name
        where
            $struct_name: $op<T, Output = $struct_name>
        {
            type Output = $struct_name;

            #[inline]
            fn $function(self, rhs: T) -> Self::Output {
                *self $operator rhs
            }
        }
    };
}