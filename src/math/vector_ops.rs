macro_rules! ops_construct {
    ($struct_name:ident, $a: expr, $b: expr, $op: tt; $($element: tt), *) =>
    {
        $struct_name{
            $(
                $element: $a.$element $op $b.$element,
            )*
        }
    };
    ($struct_name:ident, $a: expr, $b: expr, $op: tt, scalar; $($element: tt), *) =>
    {
        $struct_name{
            $(
                $element: $a.$element $op $b,
            )*
        }
    };
}

macro_rules! operator {
    ($struct_name: ident, $trait: ident, $func: ident, $op: tt; $($element: tt), *) => {
        impl ops::$trait for $struct_name
        {
            type Output = $struct_name;
            fn $func(self, rhs: Self) -> Self::Output {
                ops_construct!($struct_name, self, rhs, $op; $($element), *)
            }
        }
    };
    ($struct_name: ident, $trait: ident, $func: ident, $op: tt, assign; $($element: tt), *) => {
        impl ops::$trait for $struct_name
        {
            fn $func(&mut self, rhs: Self) {
                *self = ops_construct!($struct_name, self, rhs, $op; $($element), *);
            }
        }
    };
    ($struct_name: ident, $trait: ident, $func: ident, $op: tt, scalar; $($element: tt), *) => {
        impl ops::$trait<f32> for $struct_name
        {
            type Output = $struct_name;
            fn $func(self, rhs: f32) -> Self::Output {
                ops_construct!($struct_name, self, rhs, $op, scalar; $($element), *)
            }
        }
    };
    ($struct_name: ident, $trait: ident, $func: ident, $op: tt, scalar_assign; $($element: tt), *) => {
        impl ops::$trait<f32> for $struct_name
        {
            fn $func(&mut self, rhs: f32) {
                *self = ops_construct!($struct_name, self, rhs, $op, scalar; $($element), *);
            }
        }
    };
    ($struct_name: ident, $trait: ident, $func: ident, $op: tt, scalar_vector; $($element: tt), *) => {
        impl ops::$trait<$struct_name> for f32
        {
            type Output = $struct_name;
            fn $func(self, rhs: $struct_name) -> Self::Output {
                ops_construct!($struct_name, rhs, self, $op, scalar; $($element), *)
            }
        }
    };
}

macro_rules! vector_operators {
    ($struct_name:ident; $($element: tt, $index: tt); *) => {
        operator!($struct_name, Add, add, +; $($element), *);
        operator!($struct_name, Sub, sub, -; $($element), *);
        operator!($struct_name, Mul, mul, *; $($element), *);
        operator!($struct_name, Div, div, /; $($element), *);
        operator!($struct_name, AddAssign, add_assign, +, assign; $($element), *);
        operator!($struct_name, SubAssign, sub_assign, -, assign; $($element), *);
        operator!($struct_name, MulAssign, mul_assign, *, assign; $($element), *);
        operator!($struct_name, DivAssign, div_assign, /, assign; $($element), *);
        operator!($struct_name, Mul, mul, *, scalar; $($element), *);
        operator!($struct_name, Div, div, /, scalar; $($element), *);
        operator!($struct_name, MulAssign, mul_assign, *, scalar_assign; $($element), *);
        operator!($struct_name, DivAssign, div_assign, /, scalar_assign; $($element), *);
        operator!($struct_name, Mul, mul, *, scalar_vector; $($element), *);

        impl ops::Index<usize> for $struct_name {
            type Output = f32;
            fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
                match index {
                    $(
                        $index => &self.$element,
                    )*
                    _ => panic!("out of range"),
                }
            }
        }

        impl ops::IndexMut<usize> for $struct_name {
            fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut f32 {
                match index {
                    $(
                        $index => &mut self.$element,
                    )*
                    _ => panic!("out of range"),
                }
            }
        }

        impl ops::Neg for $struct_name {
            type Output = $struct_name;

            fn neg(self) -> Self::Output {
                $struct_name {
                $(
                    $element: -self.$element,
                )*
                }
            }
        }
    };
}

#[cfg(test)]
macro_rules! vector_ops_test {
    ($struct_name:ident; $func:ident; $op: tt; $($element: tt, $index: tt); *) => {
        #[test]
        fn $func() {
            let a = $struct_name{
                $(
                    $element: 1.3 * (1.0 + 0.1 * ($index as f32)),
                )*
            };
            let b = $struct_name{
                $(
                    $element: 1.7 * (1.0 + 0.1 * ($index as f32)),
                )*
            };
            assert_eq!(
                a $op b,
                $struct_name {
                    $(
                        $element: a.$element $op b.$element,
                    )*
                });
        }
    };
    ($struct_name:ident; $func:ident; $op: tt; assign; $($element: tt, $index: tt); *) => {
        #[test]
        fn $func() {
            let mut v1 = $struct_name{
                $(
                    $element: 1.3 * (1.0 + 0.1 * ($index as f32)),
                )*
            };
            let mut v2 = v1.clone();

            let x = $struct_name{
                $(
                    $element: 1.7 * (1.0 + 0.1 * ($index as f32)),
                )*
            };

            v1 $op x;
            $(
                v2.$element $op x.$element;
            )*
            assert_eq!(v1, v2);
        }
    };
    ($struct_name:ident; $func:ident; $op: tt; scalar; $($element: tt, $index: tt); *) => {
        #[test]
        fn $func() {
            let v = $struct_name{
                $(
                    $element: 1.3 * (1.0 + 0.1 * ($index as f32)),
                )*
            };
            let s = 3.2;
            let result = $struct_name {
                $(
                    $element: v.$element $op s,
                )*
            };
            assert_eq!(v $op s, result);
        }
    };
}

#[cfg(test)]
macro_rules! vector_operators_test {
    ($struct_name:ident; $($element:tt, $index:tt);*) => {
        vector_ops_test!($struct_name; add; +; $($element, $index); *);
        vector_ops_test!($struct_name; sub; -; $($element, $index); *);
        vector_ops_test!($struct_name; mul; *; $($element, $index); *);
        vector_ops_test!($struct_name; div; /; $($element, $index); *);
        vector_ops_test!($struct_name; add_assign; +=; assign; $($element, $index); *);
        vector_ops_test!($struct_name; sub_assign; -=; assign; $($element, $index); *);
        vector_ops_test!($struct_name; mul_assign; *=; assign; $($element, $index); *);
        vector_ops_test!($struct_name; div_assign; /=; assign; $($element, $index); *);
        vector_ops_test!($struct_name; mul_scalar; *; scalar; $($element, $index); *);
        vector_ops_test!($struct_name; div_scalar; /; scalar; $($element, $index); *);

        #[test]
        fn scalar_mul() {
            let v = $struct_name{
                $(
                    $element: 1.3 * (1.0 + 0.1 * ($index as f32)),
                )*
            };
            let s = 3.2;
            assert_eq!(s * v, v * s);
        }

        #[test]
        fn index() {
            let v = $struct_name{
                $(
                    $element: 1.3 * (1.0 + 0.1 * ($index as f32)),
                )*
            };
            $(
                assert_eq!(v[$index], v.$element);
            )*
        }

        #[test]
        #[should_panic]
        fn index_panic() {
            let v = $struct_name{
                $(
                    $element: 0.0,
                )*
            };
            $(
                let _ = v[$index + 1];
            )*
        }

        #[test]
        fn index_mut() {
            let mut v = $struct_name{
                $(
                    $element: 0.0,
                )*
            };
            $(
                let x = 1.3 * (1.0 + 0.1 * ($index as f32));
                v[$index] = x;
                assert_eq!(v[$index], x);
            )*
        }

        #[test]
        #[should_panic]
        fn index_mut_panic() {
            let mut v = $struct_name{
                $(
                    $element: 0.0,
                )*
            };
            $(
                v[$index + 1] = 1.0;
            )*
        }

        #[test]
        fn neg() {
            let a = $struct_name{
                $(
                    $element: ($index as f32 + 0.9) * 1.1,
                )*
            };
            let b = -a;
            $(
                assert_eq!(-a.$element, b.$element);
            )*;
        }
    };
}
