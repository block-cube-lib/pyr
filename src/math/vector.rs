use num::Num;

pub trait VectorElement: Num + Clone + Copy + std::fmt::Debug + Default {}
impl<T> VectorElement for T where T: Num + Clone + Copy + std::fmt::Debug + Default {}

pub trait Vector: Clone + Copy + Default {
    type ElementType: VectorElement;
    const DIMENSION: usize;
}

#[macro_use]
mod ops;

pub mod prelude;

mod vector1;
mod vector2;
mod vector3;
mod vector4;

pub use self::vector1::Vector1;
pub use self::vector2::Vector2;
pub use self::vector3::Vector3;
pub use self::vector4::Vector4;

//============================================================
// vector conversion
//============================================================
impl<T: VectorElement> Vector1<T> {
    /// Convert to Vector2
    /// ```
    /// # use pyrite::math::{Vector1, Vector2};
    /// let v1 = Vector1 { x: 1 };
    /// let v2 = v1.with_y(2);
    /// assert_eq!(v2, Vector2 { x: 1, y: 2 });
    /// ```
    pub fn with_y(&self, y: T) -> Vector2<T> {
        Vector2 { x: self.x, y }
    }

    /// Convert to Vector3
    /// ```
    /// # use pyrite::math::{Vector1, Vector3};
    /// let v1 = Vector1 { x: 1 };
    /// let v3 = v1.with_yz(2, 3);
    /// assert_eq!(v3, Vector3 { x:1, y: 2, z: 3 });
    /// ```
    pub fn with_yz(&self, y: T, z: T) -> Vector3<T> {
        Vector3 { x: self.x, y, z }
    }

    /// Convert to Vector4
    /// ```
    /// # use pyrite::math::{Vector1, Vector4};
    /// let v1 = Vector1 { x: 1 };
    /// let v4 = v1.with_yzw(2, 3, 4);
    /// assert_eq!(v4, Vector4 { x: 1, y: 2, z: 3, w: 4 });
    /// ```
    pub fn with_yzw(&self, y: T, z: T, w: T) -> Vector4<T> {
        Vector4 { x: self.x, y, z, w }
    }
}

/// Comvert to Vector2 from (Vector1, T)
/// ```
/// # use pyrite::math::{Vector1, Vector2};
/// let v1 = Vector1 { x: 1 };
/// let v2 = Vector2::from((v1, 2));
/// assert_eq!(v2, Vector2 { x: 1, y: 2 });
/// ```
impl<T: VectorElement> From<(Vector1<T>, T)> for Vector2<T> {
    fn from(t: (Vector1<T>, T)) -> Self {
        Vector2 { x: t.0.x, y: t.1 }
    }
}

/// Comvert to Vector3 from (Vector1, T, T)
/// ```
/// # use pyrite::math::{Vector1, Vector3};
/// let v1 = Vector1 { x: 1 };
/// let v3 = Vector3::from((v1, 2, 3));
/// assert_eq!(v3, Vector3 { x: 1, y: 2, z: 3});
/// ```
impl<T: VectorElement> From<(Vector1<T>, T, T)> for Vector3<T> {
    fn from(t: (Vector1<T>, T, T)) -> Self {
        Vector3 {
            x: t.0.x,
            y: t.1,
            z: t.2,
        }
    }
}

/// Comvert to Vector4 from (Vector1, T, T, T)
/// ```
/// # use pyrite::math::{Vector1, Vector4};
/// let v1 = Vector1 { x: 1 };
/// let v4 = Vector4::from((v1, 2, 3, 4));
/// assert_eq!(v4, Vector4 { x: 1, y: 2, z: 3, w: 4});
/// ```
impl<T: VectorElement> From<(Vector1<T>, T, T, T)> for Vector4<T> {
    fn from(t: (Vector1<T>, T, T, T)) -> Self {
        Vector4 {
            x: t.0.x,
            y: t.1,
            z: t.2,
            w: t.3,
        }
    }
}

impl<T: VectorElement> Vector2<T> {
    /// Convert to Vector1
    /// ```
    /// # use pyrite::math::{Vector1, Vector2};
    /// let v2 = Vector2 { x: 1, y: 2 };
    /// let v1 = v2.x();
    /// assert_eq!(v1, Vector1 { x: 1 });
    /// ```
    pub fn x(&self) -> Vector1<T> {
        Vector1 { x: self.x }
    }

    /// Convert to Vector3
    /// ```
    /// # use pyrite::math::{Vector2, Vector3};
    /// let v2 = Vector2 { x: 1, y: 2 };
    /// let v3 = v2.with_z(3);
    /// assert_eq!(v3, Vector3 { x: 1, y: 2, z: 3 });
    /// ```
    pub fn with_z(&self, z: T) -> Vector3<T> {
        Vector3 {
            x: self.x,
            y: self.y,
            z,
        }
    }

    /// Convert to Vector4
    /// ```
    /// # use pyrite::math::{Vector2, Vector4};
    /// let v2 = Vector2 { x: 1, y: 2 };
    /// let v4 = v2.with_zw(3, 4);
    /// assert_eq!(v4, Vector4 { x: 1, y: 2, z: 3, w: 4 });
    /// ```
    pub fn with_zw(&self, z: T, w: T) -> Vector4<T> {
        Vector4 {
            x: self.x,
            y: self.y,
            z,
            w,
        }
    }
}

/// Comvert to Vector3 from (Vector2, T)
/// ```
/// # use pyrite::math::{Vector2, Vector3};
/// let v2 = Vector2 { x: 1, y: 2 };
/// let v3 = Vector3::from((v2, 3));
/// assert_eq!(v3, Vector3 { x: 1, y: 2, z: 3});
/// ```
impl<T: VectorElement> From<(Vector2<T>, T)> for Vector3<T> {
    fn from(t: (Vector2<T>, T)) -> Self {
        Vector3 {
            x: t.0.x,
            y: t.0.y,
            z: t.1,
        }
    }
}

/// Comvert to Vector4 from (Vector2, T, T)
/// ```
/// # use pyrite::math::{Vector2, Vector4};
/// let v2 = Vector2 { x: 1, y: 2 };
/// let v4 = Vector4::from((v2, 3, 4));
/// assert_eq!(v4, Vector4 { x: 1, y: 2, z: 3, w: 4});
/// ```
impl<T: VectorElement> From<(Vector2<T>, T, T)> for Vector4<T> {
    fn from(t: (Vector2<T>, T, T)) -> Self {
        Vector4 {
            x: t.0.x,
            y: t.0.y,
            z: t.1,
            w: t.2,
        }
    }
}

impl<T: VectorElement> Vector3<T> {
    /// Convert to Vector1
    /// ```
    /// # use pyrite::math::{Vector1, Vector3};
    /// let v3 = Vector3 { x: 1, y: 2, z: 3 };
    /// let v1 = v3.x();
    /// assert_eq!(v1, Vector1 { x: 1 });
    /// ```
    pub fn x(&self) -> Vector1<T> {
        Vector1 { x: self.x }
    }

    /// Convert to Vector2
    pub fn xy(&self) -> Vector2<T> {
        Vector2 {
            x: self.x,
            y: self.y,
        }
    }

    /// Convert to Vector4
    /// ```
    /// # use pyrite::math::{Vector3, Vector4};
    /// let v3 = Vector3 { x: 1, y: 2, z: 3 };
    /// let v4 = v3.with_w(4);
    /// assert_eq!(v4, Vector4 { x: 1, y: 2, z: 3, w: 4 });
    /// ```
    pub fn with_w(&self, w: T) -> Vector4<T> {
        Vector4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w,
        }
    }
}

/// Comvert to Vector4 from (Vector3, T)
/// ```
/// # use pyrite::math::{Vector3, Vector4};
/// let v3 = Vector3 { x: 1, y: 2, z: 3 };
/// let v4 = Vector4::from((v3, 4));
/// assert_eq!(v4, Vector4 { x: 1, y: 2, z: 3, w: 4});
/// ```
impl<T: VectorElement> From<(Vector3<T>, T)> for Vector4<T> {
    fn from(t: (Vector3<T>, T)) -> Self {
        Vector4 {
            x: t.0.x,
            y: t.0.y,
            z: t.0.z,
            w: t.1,
        }
    }
}

impl<T: VectorElement> Vector4<T> {
    /// Vector4 to Vector1
    /// ```
    /// # use pyrite::math::{Vector1, Vector4};
    /// let v4 = Vector4 { x: 1, y: 2, z: 3, w: 4 };
    /// let v1 = v4.x();
    /// assert_eq!(v1, Vector1 { x: 1 });
    /// ```
    pub fn x(&self) -> Vector1<T> {
        Vector1 { x: self.x }
    }

    /// Vector4 to Vector2
    /// ```
    /// # use pyrite::math::{Vector2, Vector4};
    /// let v4 = Vector4 { x: 1, y: 2, z: 3, w: 4 };
    /// let v2 = v4.xy();
    /// assert_eq!(v2, Vector2 { x: 1, y: 2 });
    /// ```
    pub fn xy(&self) -> Vector2<T> {
        Vector2 {
            x: self.x,
            y: self.y,
        }
    }

    /// Vector4 to Vector3
    /// ```
    /// # use pyrite::math::{Vector3, Vector4};
    /// let v4 = Vector4 { x: 1, y: 2, z: 3, w: 4 };
    /// let v3 = v4.xyz();
    /// assert_eq!(v3, Vector3 { x: 1, y: 2, z: 3 });
    /// ```
    pub fn xyz(&self) -> Vector3<T> {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

/// Convert to Vector2 from Vector1.
/// ```
/// # use pyrite::math::{Vector1, Vector2};
/// let v1 = Vector1 { x: 1 };
/// let v2 = Vector2::from(v1);
/// assert_eq!(v2, Vector2 { x: 1, y: 0 });
/// ```
impl<T: VectorElement> From<Vector1<T>> for Vector2<T> {
    fn from(v: Vector1<T>) -> Vector2<T> {
        Vector2::new(v.x, T::zero())
    }
}

/// Convert to Vector3 from Vector1.
/// ```
/// # use pyrite::math::{Vector1, Vector3};
/// let v1 = Vector1 { x: 1 };
/// let v3 = Vector3::from(v1);
/// assert_eq!(v3, Vector3 { x: 1, y: 0, z: 0 });
/// ```
impl<T: VectorElement> From<Vector1<T>> for Vector3<T> {
    fn from(v: Vector1<T>) -> Vector3<T> {
        Vector3::new(v.x, T::zero(), T::zero())
    }
}

/// Convert to Vector4 from Vector1.
/// ```
/// # use pyrite::math::{Vector1, Vector4};
/// let v1 = Vector1::new(1);
/// let v4 = Vector4::from(v1);
/// assert_eq!(v4, Vector4 { x: 1, y: 0, z: 0, w: 0 });
/// ```
impl<T: VectorElement> From<Vector1<T>> for Vector4<T> {
    fn from(v: Vector1<T>) -> Vector4<T> {
        Vector4::new(v.x, T::zero(), T::zero(), T::zero())
    }
}

/// Convert to Vector3 from Vector2.
/// ```
/// # use pyrite::math::{Vector2, Vector3};
/// let v2 = Vector2 { x: 1, y: 2 };
/// let v3 = Vector3::from(v2);
/// assert_eq!(v3, Vector3 { x: 1, y: 2, z: 0 });
/// ```
impl<T: VectorElement> From<Vector2<T>> for Vector3<T> {
    fn from(v: Vector2<T>) -> Vector3<T> {
        Vector3::new(v.x, v.y, T::zero())
    }
}

/// Convert to Vector4 from Vector2.
/// ```
/// # use pyrite::math::{Vector2, Vector4};
/// let v2 = Vector2 { x: 1, y: 2 };
/// let v4 = Vector4::from(v2);
/// assert_eq!(v4, Vector4 { x: 1, y: 2, z: 0, w: 0 });
/// ```
impl<T: VectorElement> From<Vector2<T>> for Vector4<T> {
    fn from(v: Vector2<T>) -> Vector4<T> {
        Vector4::new(v.x, v.y, T::zero(), T::zero())
    }
}

/// Convert to Vector4 from Vector3.
/// ```
/// # use pyrite::math::{Vector3, Vector4};
/// let v3 = Vector3 { x: 1, y: 2, z: 3 };
/// let v4 = Vector4::from(v3);
/// assert_eq!(v4, Vector4 { x: 1, y: 2, z: 3, w: 0 });
/// ```
impl<T: VectorElement> From<Vector3<T>> for Vector4<T> {
    fn from(v: Vector3<T>) -> Vector4<T> {
        Vector4::new(v.x, v.y, v.z, T::zero())
    }
}
