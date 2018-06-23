use math::Vector3;

pub struct Sphere {
    pub origin: Vector3,
    pub radius: f32,
}

pub struct AABB {
    pub min: Vector3,
    pub max: Vector3,
}
