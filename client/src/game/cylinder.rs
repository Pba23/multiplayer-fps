use crate::game::vector3d::Vector3D;


#[derive(Clone)]
pub struct Ray {
    pub origin: Vector3D,
    pub direction: Vector3D,
}
impl Ray {
    pub fn new(origin: Vector3D, direction: Vector3D) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn point_at(&self, t: f32) -> Vector3D {
        self.origin + self.direction * t
    }
    
}
pub struct Intersection {
    pub t: f32,
    pub point: Vector3D,
    pub normal: Vector3D,
}
pub trait Object: Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn normal(&self, point: &Vector3D) -> Vector3D;
}

pub struct Cylinder {
    pub center: Vector3D,
    pub axis: Vector3D,
    pub radius: f32,
    pub height: f32,
}

impl Cylinder {
    pub fn new(center: Vector3D, axis: Vector3D, radius: f32, height: f32) -> Self {
        Self {
            center,
            axis: axis.normalize(),
            radius,
            height,
        }
    }
}

impl Object for Cylinder {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let oc = ray.origin - self.center;
        let axis_dot_dir = ray.direction.dot(&self.axis);
        let axis_dot_oc = oc.dot(&self.axis);

        // Calcul des coefficients quadratiques pour la surface latérale
        let a = ray.direction.dot(&ray.direction) - axis_dot_dir.powi(2);
        let b = 2.0 * (ray.direction.dot(&oc) - axis_dot_dir * axis_dot_oc);
        let c = oc.dot(&oc) - axis_dot_oc.powi(2) - self.radius.powi(2);

        let discriminant = b * b - 4.0 * a * c;

        let mut closest_t = f32::INFINITY;
        let mut closest_point = Vector3D::new(0.0, 0.0, 0.0);
        let mut closest_normal = Vector3D::new(0.0, 0.0, 0.0);

        // Vérification de l'intersection avec la surface latérale
        if discriminant >= 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t1 = (-b - sqrt_discriminant) / (2.0 * a);
            let t2 = (-b + sqrt_discriminant) / (2.0 * a);

            for &t in &[t1, t2] {
                if t > 0.0 && t < closest_t {
                    let point = ray.point_at(t);
                    let height_check = (point - self.center).dot(&self.axis);
                    if height_check >= 0.0 && height_check <= self.height {
                        closest_t = t;
                        closest_point = point;
                        closest_normal = (point - (self.center + self.axis * height_check)).normalize();
                    }
                }
            }
        }

        // Vérification de l'intersection avec les bases
        let base_centers = [self.center, self.center + self.axis * self.height];
        let base_normals = [-self.axis, self.axis];

        for (&base_center, &base_normal) in base_centers.iter().zip(base_normals.iter()) {
            let t = (base_center - ray.origin).dot(&base_normal) / ray.direction.dot(&base_normal);
            if t > 0.0 && t < closest_t {
                let point = ray.point_at(t);
                if (point - base_center).magnitude() <= self.radius {
                    closest_t = t;
                    closest_point = point;
                    closest_normal = base_normal;
                }
            }
        }

        if closest_t != f32::INFINITY {
            Some(Intersection { t: closest_t, point: closest_point, normal: closest_normal })
        } else {
            None
        }
    }

    fn normal(&self, point: &Vector3D) -> Vector3D {
        let height_check = (*point - self.center).dot(&self.axis);
        if height_check.abs() < 1e-6 {
            return -self.axis;
        } else if (height_check - self.height).abs() < 1e-6 {
            return self.axis;
        }
        (*point - (self.center + self.axis * height_check)).normalize()
    }
}