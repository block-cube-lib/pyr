macro_rules! impl_xyzw_accessor {
    (min_vector_dimension 1; 1, $function_name: ident, $($index: expr) +) => {
        impl_xyzw_accessor!(dimension 1; 1, $function_name, $($index)+);
        impl_xyzw_accessor!(dimension 2; 1, $function_name, $($index)+);
        impl_xyzw_accessor!(dimension 3; 1, $function_name, $($index)+);
        impl_xyzw_accessor!(dimension 4; 1, $function_name, $($index)+);
    };
    (min_vector_dimension 2; 1, $function_name: ident, $($index: expr),+) => {
        impl_xyzw_accessor!(dimension 2; 1, $function_name, $($index)+);
        impl_xyzw_accessor!(dimension 3; 1, $function_name, $($index)+);
        impl_xyzw_accessor!(dimension 4; 1, $function_name, $($index)+);
    };
    (min_vector_dimension 3; 1, $function_name: ident, $($index: expr),+) => {
        impl_xyzw_accessor!(dimension 3; 1, $function_name, $($index)+);
        impl_xyzw_accessor!(dimension 4; 1, $function_name, $($index)+);
    };
    (min_vector_dimension 4; 1, $function_name: ident, $($index: expr),+) => {
        impl_xyzw_accessor!(dimension 4; 1, $function_name, $($index)+);
    };
    (min_vector_dimension 1; $return_vector_dimension: expr, $function_name: ident, $($index: expr),+) => {
        impl_xyzw_accessor!(dimension 1; $return_vector_dimension, $function_name, $($index),+);
        impl_xyzw_accessor!(dimension 2; $return_vector_dimension, $function_name, $($index),+);
        impl_xyzw_accessor!(dimension 3; $return_vector_dimension, $function_name, $($index),+);
        impl_xyzw_accessor!(dimension 4; $return_vector_dimension, $function_name, $($index),+);
    };
    (min_vector_dimension 2; $return_vector_dimension: expr, $function_name: ident, $($index: expr),+) => {
        impl_xyzw_accessor!(dimension 2; $return_vector_dimension, $function_name, $($index),+);
        impl_xyzw_accessor!(dimension 3; $return_vector_dimension, $function_name, $($index),+);
        impl_xyzw_accessor!(dimension 4; $return_vector_dimension, $function_name, $($index),+);
    };
    (min_vector_dimension 3; $return_vector_dimension: expr, $function_name: ident, $($index: expr),+) => {
        impl_xyzw_accessor!(dimension 3; $return_vector_dimension, $function_name, $($index),+);
        impl_xyzw_accessor!(dimension 4; $return_vector_dimension, $function_name, $($index),+);
    };
    (min_vector_dimension 4; $return_vector_dimension: expr, $function_name: ident, $($index: expr),+) => {
        impl_xyzw_accessor!(dimension 4; $return_vector_dimension, $function_name, $($index),+);
    };

    (dimension $vector_dimension: expr; 1, $function_name: ident, $($index: expr)+) => {
        impl<T: VectorElement> Vector<T, $vector_dimension> {
            pub fn $function_name(&self) -> T {
                self.elements[$($index)+]
            }
        }
    };
    (dimension $vector_dimension: expr; $return_vector_dimension: expr, $function_name: ident, $($index: expr),+) => {
        impl<T: VectorElement> Vector<T, $vector_dimension> {
            pub fn $function_name(&self) -> Vector<T, $return_vector_dimension> {
                [
                    $( self.elements[$index] ),+
                ].to_vector()
            }
        }
    };
}

impl_xyzw_accessor!(min_vector_dimension 1; 1, x, 0);
impl_xyzw_accessor!(min_vector_dimension 1; 2, xx, 0, 0);
impl_xyzw_accessor!(min_vector_dimension 1; 3, xxx, 0, 0, 0);
impl_xyzw_accessor!(min_vector_dimension 1; 4, xxxx, 0, 0, 0, 0);
impl_xyzw_accessor!(min_vector_dimension 2; 1, y, 1);
impl_xyzw_accessor!(min_vector_dimension 2; 2, xy, 0, 1);
impl_xyzw_accessor!(min_vector_dimension 2; 2, yx, 1, 0);
impl_xyzw_accessor!(min_vector_dimension 2; 2, yy, 1, 1);
impl_xyzw_accessor!(min_vector_dimension 2; 3, xxy, 0, 0, 1);
impl_xyzw_accessor!(min_vector_dimension 2; 3, xyx, 0, 1, 0);
impl_xyzw_accessor!(min_vector_dimension 2; 3, xyy, 0, 1, 1);
impl_xyzw_accessor!(min_vector_dimension 2; 3, yxx, 1, 0, 0);
impl_xyzw_accessor!(min_vector_dimension 2; 3, yxy, 1, 0, 1);
impl_xyzw_accessor!(min_vector_dimension 2; 3, yyx, 1, 1, 0);
impl_xyzw_accessor!(min_vector_dimension 2; 3, yyy, 1, 1, 1);
impl_xyzw_accessor!(min_vector_dimension 2; 4, xxxy, 0, 0, 0, 1);
impl_xyzw_accessor!(min_vector_dimension 2; 4, xxyx, 0, 0, 1, 0);
impl_xyzw_accessor!(min_vector_dimension 2; 4, xxyy, 0, 0, 1, 1);
impl_xyzw_accessor!(min_vector_dimension 2; 4, xyxx, 0, 1, 0, 0);
impl_xyzw_accessor!(min_vector_dimension 2; 4, xyxy, 0, 1, 0, 1);
impl_xyzw_accessor!(min_vector_dimension 2; 4, xyyx, 0, 1, 1, 0);
impl_xyzw_accessor!(min_vector_dimension 2; 4, xyyy, 0, 1, 1, 1);
impl_xyzw_accessor!(min_vector_dimension 2; 4, yxxx, 1, 0, 0, 0);
impl_xyzw_accessor!(min_vector_dimension 2; 4, yxxy, 1, 0, 0, 1);
impl_xyzw_accessor!(min_vector_dimension 2; 4, yxyx, 1, 0, 1, 0);
impl_xyzw_accessor!(min_vector_dimension 2; 4, yxyy, 1, 0, 1, 1);
impl_xyzw_accessor!(min_vector_dimension 2; 4, yyxx, 1, 1, 0, 0);
impl_xyzw_accessor!(min_vector_dimension 2; 4, yyxy, 1, 1, 0, 1);
impl_xyzw_accessor!(min_vector_dimension 2; 4, yyyx, 1, 1, 1, 0);
impl_xyzw_accessor!(min_vector_dimension 2; 4, yyyy, 1, 1, 1, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 1, z, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 2, xz, 0, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 2, yz, 1, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 2, zx, 2, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 2, zy, 2, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 2, zz, 2, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 3, xxz, 0, 0, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 3, xyz, 0, 1, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 3, xzx, 0, 2, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 3, xzy, 0, 2, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 3, xzz, 0, 2, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 3, yxz, 1, 0, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 3, yyz, 1, 1, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 3, yzx, 1, 2, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 3, yzy, 1, 2, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 3, yzz, 1, 2, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 3, zxx, 2, 0, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 3, zxy, 2, 0, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 3, zxz, 2, 0, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 3, zyx, 2, 1, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 3, zyy, 2, 1, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 3, zyz, 2, 1, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 3, zzx, 2, 2, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 3, zzy, 2, 2, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 3, zzz, 2, 2, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, xxxz, 0, 0, 0, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, xxyz, 0, 0, 1, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, xxzx, 0, 0, 2, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 4, xxzy, 0, 0, 2, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 4, xxzz, 0, 0, 2, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, xyxz, 0, 1, 0, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, xyyz, 0, 1, 1, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, xyzx, 0, 1, 2, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 4, xyzy, 0, 1, 2, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 4, xyzz, 0, 1, 2, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, xzxx, 0, 2, 0, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 4, xzxy, 0, 2, 0, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 4, xzxz, 0, 2, 0, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, xzyx, 0, 2, 1, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 4, xzyy, 0, 2, 1, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 4, xzyz, 0, 2, 1, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, xzzx, 0, 2, 2, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 4, xzzy, 0, 2, 2, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 4, xzzz, 0, 2, 2, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, yxxz, 1, 0, 0, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, yxyz, 1, 0, 1, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, yxzx, 1, 0, 2, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 4, yxzy, 1, 0, 2, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 4, yxzz, 1, 0, 2, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, yyxz, 1, 1, 0, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, yyyz, 1, 1, 1, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, yyzx, 1, 1, 2, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 4, yyzy, 1, 1, 2, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 4, yyzz, 1, 1, 2, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, yzxx, 1, 2, 0, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 4, yzxy, 1, 2, 0, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 4, yzxz, 1, 2, 0, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, yzyx, 1, 2, 1, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 4, yzyy, 1, 2, 1, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 4, yzyz, 1, 2, 1, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, yzzx, 1, 2, 2, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 4, yzzy, 1, 2, 2, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 4, yzzz, 1, 2, 2, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zxxx, 2, 0, 0, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zxxy, 2, 0, 0, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zxxz, 2, 0, 0, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zxyx, 2, 0, 1, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zxyy, 2, 0, 1, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zxyz, 2, 0, 1, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zxzx, 2, 0, 2, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zxzy, 2, 0, 2, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zxzz, 2, 0, 2, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zyxx, 2, 1, 0, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zyxy, 2, 1, 0, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zyxz, 2, 1, 0, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zyyx, 2, 1, 1, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zyyy, 2, 1, 1, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zyyz, 2, 1, 1, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zyzx, 2, 1, 2, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zyzy, 2, 1, 2, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zyzz, 2, 1, 2, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zzxx, 2, 2, 0, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zzxy, 2, 2, 0, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zzxz, 2, 2, 0, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zzyx, 2, 2, 1, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zzyy, 2, 2, 1, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zzyz, 2, 2, 1, 2);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zzzx, 2, 2, 2, 0);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zzzy, 2, 2, 2, 1);
impl_xyzw_accessor!(min_vector_dimension 3; 4, zzzz, 2, 2, 2, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 1, w, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 2, ww, 3, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 2, wx, 3, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 2, wy, 3, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 2, wz, 3, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 2, xw, 0, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 2, yw, 1, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 2, zw, 2, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 3, www, 3, 3, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 3, wwx, 3, 3, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 3, wwy, 3, 3, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 3, wwz, 3, 3, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 3, wxw, 3, 0, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 3, wxx, 3, 0, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 3, wxy, 3, 0, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 3, wxz, 3, 0, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 3, wyw, 3, 1, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 3, wyx, 3, 1, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 3, wyy, 3, 1, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 3, wyz, 3, 1, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 3, wzw, 3, 2, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 3, wzx, 3, 2, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 3, wzy, 3, 2, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 3, wzz, 3, 2, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 3, xww, 0, 3, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 3, xwx, 0, 3, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 3, xwy, 0, 3, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 3, xwz, 0, 3, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 3, xxw, 0, 0, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 3, xyw, 0, 1, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 3, xzw, 0, 2, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 3, yww, 1, 3, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 3, ywx, 1, 3, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 3, ywy, 1, 3, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 3, ywz, 1, 3, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 3, yxw, 1, 0, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 3, yyw, 1, 1, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 3, yzw, 1, 2, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 3, zww, 2, 3, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 3, zwx, 2, 3, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 3, zwy, 2, 3, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 3, zwz, 2, 3, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 3, zxw, 2, 0, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 3, zyw, 2, 1, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 3, zzw, 2, 2, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wwww, 3, 3, 3, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wwwx, 3, 3, 3, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wwwy, 3, 3, 3, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wwwz, 3, 3, 3, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wwxw, 3, 3, 0, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wwxx, 3, 3, 0, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wwxy, 3, 3, 0, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wwxz, 3, 3, 0, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wwyw, 3, 3, 1, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wwyx, 3, 3, 1, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wwyy, 3, 3, 1, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wwyz, 3, 3, 1, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wwzw, 3, 3, 2, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wwzx, 3, 3, 2, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wwzy, 3, 3, 2, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wwzz, 3, 3, 2, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wxww, 3, 0, 3, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wxwx, 3, 0, 3, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wxwy, 3, 0, 3, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wxwz, 3, 0, 3, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wxxw, 3, 0, 0, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wxxx, 3, 0, 0, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wxxy, 3, 0, 0, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wxxz, 3, 0, 0, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wxyw, 3, 0, 1, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wxyx, 3, 0, 1, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wxyy, 3, 0, 1, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wxyz, 3, 0, 1, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wxzw, 3, 0, 2, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wxzx, 3, 0, 2, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wxzy, 3, 0, 2, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wxzz, 3, 0, 2, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wyww, 3, 1, 3, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wywx, 3, 1, 3, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wywy, 3, 1, 3, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wywz, 3, 1, 3, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wyxw, 3, 1, 0, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wyxx, 3, 1, 0, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wyxy, 3, 1, 0, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wyxz, 3, 1, 0, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wyyw, 3, 1, 1, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wyyx, 3, 1, 1, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wyyy, 3, 1, 1, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wyyz, 3, 1, 1, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wyzw, 3, 1, 2, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wyzx, 3, 1, 2, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wyzy, 3, 1, 2, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wyzz, 3, 1, 2, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wzww, 3, 2, 3, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wzwx, 3, 2, 3, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wzwy, 3, 2, 3, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wzwz, 3, 2, 3, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wzxw, 3, 2, 0, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wzxx, 3, 2, 0, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wzxy, 3, 2, 0, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wzxz, 3, 2, 0, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wzyw, 3, 2, 1, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wzyx, 3, 2, 1, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wzyy, 3, 2, 1, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wzyz, 3, 2, 1, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wzzw, 3, 2, 2, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wzzx, 3, 2, 2, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wzzy, 3, 2, 2, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, wzzz, 3, 2, 2, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xwww, 0, 3, 3, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xwwx, 0, 3, 3, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xwwy, 0, 3, 3, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xwwz, 0, 3, 3, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xwxw, 0, 3, 0, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xwxx, 0, 3, 0, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xwxy, 0, 3, 0, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xwxz, 0, 3, 0, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xwyw, 0, 3, 1, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xwyx, 0, 3, 1, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xwyy, 0, 3, 1, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xwyz, 0, 3, 1, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xwzw, 0, 3, 2, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xwzx, 0, 3, 2, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xwzy, 0, 3, 2, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xwzz, 0, 3, 2, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xxww, 0, 0, 3, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xxwx, 0, 0, 3, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xxwy, 0, 0, 3, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xxwz, 0, 0, 3, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xxxw, 0, 0, 0, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xxyw, 0, 0, 1, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xxzw, 0, 0, 2, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xyww, 0, 1, 3, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xywx, 0, 1, 3, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xywy, 0, 1, 3, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xywz, 0, 1, 3, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xyxw, 0, 1, 0, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xyyw, 0, 1, 1, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xyzw, 0, 1, 2, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xzww, 0, 2, 3, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xzwx, 0, 2, 3, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xzwy, 0, 2, 3, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xzwz, 0, 2, 3, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xzxw, 0, 2, 0, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xzyw, 0, 2, 1, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, xzzw, 0, 2, 2, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, ywww, 1, 3, 3, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, ywwx, 1, 3, 3, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, ywwy, 1, 3, 3, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, ywwz, 1, 3, 3, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, ywxw, 1, 3, 0, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, ywxx, 1, 3, 0, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, ywxy, 1, 3, 0, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, ywxz, 1, 3, 0, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, ywyw, 1, 3, 1, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, ywyx, 1, 3, 1, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, ywyy, 1, 3, 1, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, ywyz, 1, 3, 1, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, ywzw, 1, 3, 2, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, ywzx, 1, 3, 2, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, ywzy, 1, 3, 2, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, ywzz, 1, 3, 2, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, yxww, 1, 0, 3, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, yxwx, 1, 0, 3, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, yxwy, 1, 0, 3, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, yxwz, 1, 0, 3, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, yxxw, 1, 0, 0, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, yxyw, 1, 0, 1, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, yxzw, 1, 0, 2, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, yyww, 1, 1, 3, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, yywx, 1, 1, 3, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, yywy, 1, 1, 3, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, yywz, 1, 1, 3, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, yyxw, 1, 1, 0, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, yyyw, 1, 1, 1, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, yyzw, 1, 1, 2, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, yzww, 1, 2, 3, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, yzwx, 1, 2, 3, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, yzwy, 1, 2, 3, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, yzwz, 1, 2, 3, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, yzxw, 1, 2, 0, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, yzyw, 1, 2, 1, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, yzzw, 1, 2, 2, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zwww, 2, 3, 3, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zwwx, 2, 3, 3, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zwwy, 2, 3, 3, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zwwz, 2, 3, 3, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zwxw, 2, 3, 0, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zwxx, 2, 3, 0, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zwxy, 2, 3, 0, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zwxz, 2, 3, 0, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zwyw, 2, 3, 1, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zwyx, 2, 3, 1, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zwyy, 2, 3, 1, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zwyz, 2, 3, 1, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zwzw, 2, 3, 2, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zwzx, 2, 3, 2, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zwzy, 2, 3, 2, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zwzz, 2, 3, 2, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zxww, 2, 0, 3, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zxwx, 2, 0, 3, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zxwy, 2, 0, 3, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zxwz, 2, 0, 3, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zxxw, 2, 0, 0, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zxyw, 2, 0, 1, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zxzw, 2, 0, 2, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zyww, 2, 1, 3, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zywx, 2, 1, 3, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zywy, 2, 1, 3, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zywz, 2, 1, 3, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zyxw, 2, 1, 0, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zyyw, 2, 1, 1, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zyzw, 2, 1, 2, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zzww, 2, 2, 3, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zzwx, 2, 2, 3, 0);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zzwy, 2, 2, 3, 1);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zzwz, 2, 2, 3, 2);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zzxw, 2, 2, 0, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zzyw, 2, 2, 1, 3);
impl_xyzw_accessor!(min_vector_dimension 4; 4, zzzw, 2, 2, 2, 3);

#[cfg(test)]
mod xyzw_accessor_test {
    use super::*;
    type Vector1<T> = Vector<T, 1>;
    type Vector2<T> = Vector<T, 2>;
    type Vector3<T> = Vector<T, 3>;
    type Vector4<T> = Vector<T, 4>;

    #[test]
    fn x() {
        let v = Vector1::new(1);
        assert_eq!(v.x(), 1);

        let v = Vector2::new(1, 2);
        assert_eq!(v.x(), 1);

        let v = Vector3::new(1, 2, 3);
        assert_eq!(v.x(), 1);

        let v = Vector4::new(1, 2, 3, 4);
        assert_eq!(v.x(), 1);
    }

    #[test]
    fn xx() {
        let v = Vector1::new(1);
        assert_eq!(v.xx(), [1, 1].into());

        let v = Vector2::new(1, 2);
        assert_eq!(v.xx(), [1, 1].into());

        let v = Vector3::new(1, 2, 3);
        assert_eq!(v.xx(), [1, 1].into());

        let v = Vector4::new(1, 2, 3, 4);
        assert_eq!(v.xx(), [1, 1].into());
    }

    #[test]
    fn xy() {
        let v = Vector2::new(1, 2);
        assert_eq!(v.xy(), [1, 2].into());

        let v = Vector3::new(1, 2, 3);
        assert_eq!(v.xy(), [1, 2].into());

        let v = Vector4::new(1, 2, 3, 4);
        assert_eq!(v.xy(), [1, 2].into());
    }

    #[test]
    fn yx() {
        let v = Vector2::new(1, 2);
        assert_eq!(v.yx(), [2, 1].into());

        let v = Vector3::new(1, 2, 3);
        assert_eq!(v.yx(), [2, 1].into());

        let v = Vector4::new(1, 2, 3, 4);
        assert_eq!(v.yx(), [2, 1].into());
    }

    #[test]
    fn xyz() {
        let v = Vector3::new(1, 2, 3);
        assert_eq!(v.xyz(), [1, 2, 3].into());

        let v = Vector4::new(1, 2, 3, 4);
        assert_eq!(v.xyz(), [1, 2, 3].into());
    }

    #[test]
    fn xzy() {
        let v = Vector3::new(1, 2, 3);
        assert_eq!(v.xzy(), [1, 3, 2].into());

        let v = Vector4::new(1, 2, 3, 4);
        assert_eq!(v.xzy(), [1, 3, 2].into());
    }

    #[test]
    fn xyzw() {
        let v = Vector4::new(1, 2, 3, 4);
        assert_eq!(v.xyzw(), [1, 2, 3, 4].into());
    }

    #[test]
    fn wzyx() {
        let v = Vector4::new(1, 2, 3, 4);
        assert_eq!(v.wzyx(), [4, 3, 2, 1].into());
    }
}
