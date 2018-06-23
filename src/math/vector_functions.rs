macro_rules! vector_functions {
    ($struct_name:ident; $($element: tt, $index: tt); *) => {
        impl $struct_name {
            pub fn new($($element: f32), *) -> Self {
                $struct_name {
                    $(
                        $element: $element,
                    )*
                }
            }

            pub fn zero() -> Self {
                $struct_name {
                    $(
                        $element: 0.0,
                    )*
                }
            }

            pub fn one() -> Self {
                $struct_name {
                    $(
                        $element: 1.0,
                    )*
                }
            }

            pub fn length_squared(&self) -> f32 {
                $(
                    self.$element.powi(2) +
                )*
                0.0
            }

            pub fn length(&self) -> f32 {
                self.length_squared().sqrt()
            }

            pub fn distance_squared(&self, other: Self) -> f32 {
                (*self - other).length_squared()
            }

            pub fn distance(&self, other: Self) -> f32 {
                (*self - other).length()
            }

            pub fn dot(&self, other: Self) -> f32 {
                $(
                    self.$element * other.$element +
                )*
                0.0
            }

            pub fn normalized(&self) -> Self {
                let len = self.length();
                if len < std::f32::EPSILON {
                    *self
                } else {
                    *self / self.length()
                }
            }

            pub fn normalize(&mut self) {
                *self = self.normalized();
            }
        }
    };
}

#[cfg(test)]
macro_rules! vector_functions_test {
    ($struct_name:ident; $($element: tt, $index: tt); *) => {
        macro_rules! create_vector {
            () => {
                create_vector!([1.1, 2.4, 3.5, 6.7]);
            };
            (2) => {
                [
                    create_vector!(),
                    create_vector!([3.5, 6.3, 1.3, 5.7])
                ];
            };
            ($value: tt) => {
                $struct_name {
                    $(
                        $element: $value[$index],
                    )*
                }
            };
        }

        #[test]
        fn new() {
            let a = $struct_name::new($(($index + 1) as f32), *);
            let b = $struct_name {
                $(
                    $element: ($index + 1) as f32,
                )*
            };
            assert_eq!(a, b);
        }

        #[test]
        fn zero() {
            let v = $struct_name {
                $(
                    $element: 0.0,
                )*
            };
            assert_eq!($struct_name::zero(), v);
        }

        #[test]
        fn one() {
            let v = $struct_name {
                $(
                    $element: 1.0,
                )*
            };
            assert_eq!($struct_name::one(), v);
        }

        #[test]
        fn length_squared() {
            let v = create_vector!();
            let len_sq = $(v.$element.powi(2) + )* 0.0;
            assert_eq!(v.length_squared(), len_sq);
        }

        #[test]
        fn length() {
            let v = create_vector!();
            assert_eq!(v.length(), v.length_squared().sqrt());
        }

        #[test]
        fn distance_squared() {
            let [a, b] = create_vector!(2);
            assert_eq!(a.distance_squared(b), (b - a).length_squared());
        }

        #[test]
        fn distance() {
            let [a, b] = create_vector!(2);
            assert_eq!(a.distance(b), (b - a).length());
        }

        #[test]
        fn dot() {
            let [a, b] = create_vector!(2);
            let d = $((a.$element * b.$element) +)* 0.0;
            assert_eq!(a.dot(b), d);
        }

        #[test]
        fn normalized() {
            let v = create_vector!();
            let n = v.normalized();
            let len = v.length();
            assert!((1.0 - n.length()).abs() < ::std::f32::EPSILON);
            $(
                assert_eq!(n[$index], v[$index] / len);
            )*
        }

        #[test]
        fn normalize() {
            let mut v = create_vector!();
            let n = v.normalized();
            v.normalize();
            assert_eq!(v, n);
        }
    };
}
