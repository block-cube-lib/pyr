use crate::Vec3;
use crate::scalar::FloatScalar;
use thiserror::Error;

/// Error type for Ray creation errors.
#[derive(Debug, Error)]
pub enum RayCreationError {
    /// Error when the direction vector is zero.
    #[error("Direction vector cannot be zero.")]
    ZeroDirectionVector,
}

/// A ray in 3D space, defined by an origin point and a direction vector.
pub struct Ray<T: FloatScalar> {
    /// The origin point of the ray.
    origin: Vec3<T>,
    /// The direction vector of the ray. This vector is normalized.
    dir: Vec3<T>,
}

impl<T: FloatScalar> Ray<T> {
    /// Creates a new `Ray` with the given origin and direction.
    /// The direction vector will be normalized. If the direction vector is zero, an error will
    pub fn new(origin: Vec3<T>, dir: Vec3<T>) -> Result<Self, RayCreationError> {
        let dir = dir
            .try_normalized()
            .ok_or(RayCreationError::ZeroDirectionVector)?;
        Ok(Self { origin, dir })
    }

    /// Creates a new `Ray` without checking if the direction vector is zero and normalizing it.
    /// This is unsafe because a zero direction vector will lead to undefined behavior.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the direction vector is not zero and is normalized.
    pub unsafe fn new_unchecked(origin: Vec3<T>, dir: Vec3<T>) -> Self {
        Self { origin, dir }
    }

    /// Returns a reference to the origin of the ray.
    pub fn origin(&self) -> &Vec3<T> {
        &self.origin
    }

    /// Returns a reference to the direction of the ray.
    pub fn dir(&self) -> &Vec3<T> {
        &self.dir
    }

    /// Sets the origin of the ray to a new value.
    pub fn set_origin(&mut self, new_origin: Vec3<T>) {
        self.origin = new_origin;
    }

    /// Sets the direction of the ray to a new value.
    /// The new direction vector will be normalized. If the new direction vector is zero, an error
    /// will be returned.
    pub fn set_dir(&mut self, new_dir: Vec3<T>) -> Result<(), RayCreationError> {
        let normalized_dir = new_dir
            .try_normalized()
            .ok_or(RayCreationError::ZeroDirectionVector)?;
        self.dir = normalized_dir;
        Ok(())
    }
}
