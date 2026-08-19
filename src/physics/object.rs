use crate::{
    math::{point::Point, vector::Vector},
    physics::{
        intersect::Intersection, material::Material, plane::Plane, ray::Ray, sphere::Sphere,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

impl Object {
    pub fn intersect(&self, ray: Ray) -> Vec<Intersection<'_>> {
        match self {
            Object::Sphere(sphere) => {
                let inverse = sphere.transform.inverse().unwrap();

                let local_ray = ray.transform(&inverse);

                sphere
                    .local_intersect(local_ray)
                    .into_iter()
                    .map(|t| Intersection::new(t, self))
                    .collect()
            }

            Object::Plane(plane) => {
                let inverse = plane.transform.inverse().unwrap();

                let local_ray = ray.transform(&inverse);

                plane
                    .local_intersect(local_ray)
                    .into_iter()
                    .map(|t| Intersection::new(t, self))
                    .collect()
            }
        }
    }
    pub fn is_plane(&self, plane: &Plane) -> bool {
        match self {
            Object::Plane(p) => p == plane,
            _ => false,
        }
    }
    pub fn normal_at(&self, world_point: Point) -> Vector {
        match self {
            Object::Sphere(sphere) => sphere.normal_at(world_point),
            Object::Plane(plane) => plane.local_normal_at(world_point),
        }
    }
    pub fn material(&self) -> &Material {
        match self {
            Object::Sphere(sphere) => &sphere.material,
            Object::Plane(plane) => &plane.material,
        }
    }
    pub fn material_mut(&mut self) -> &mut Material {
        match self {
            Object::Sphere(sphere) => &mut sphere.material,
            Object::Plane(plane) => &mut plane.material,
        }
    }
}
