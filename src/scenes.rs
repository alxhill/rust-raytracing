use crate::surfaces::Normal;
use crate::{AABox, Light, Object, Phong, Plane, Point3D, RGBColor, Scene, Sphere, Vector3D};
use std::sync::Arc;

impl Scene {
    pub fn default_scene_one() -> Scene {
        let mut scene = Scene::new();

        let red_mat = Arc::new(Phong::reflective(0.25, 0.65, 0.6, 0.5, 20.0, RGBColor::RED));
        let yellow_mat = Arc::new(Phong::new(0.2, 0.8, 0.0, 1.0, RGBColor::YELLOW));
        let grey_mat = Arc::new(Phong::new(0.5, 0.5, 0.0, 1.0, RGBColor::GREY));
        let green_mat = Arc::new(Phong::reflective(0.2, 0.4, 0.0, 0.8, 1.0, RGBColor::GREEN));
        let reflective_mat = Arc::new(Phong::reflective(0.0, 0.0, 0.0, 1.2, 4.0, RGBColor::WHITE));
        let normal_mat = Arc::new(Normal::default());

        scene.add_object(Object::sphere(
            Sphere::new(Point3D::zero(), 30.0),
            reflective_mat,
        ));
        scene.add_object(Object::sphere(
            Sphere::new(Point3D { x: 0.0, y: 60.0, z: -1.0 }, 20.0),
            normal_mat,
        ));
        scene.add_object(Object::sphere(
            Sphere::new(Point3D { x: -40.0, y: 25.0, z: -2.0 }, 15.0),
            red_mat.clone(),
        ));
        scene.add_object(Object::sphere(
            Sphere::new(Point3D { x: 40.0, y: 15.0, z: -2.0 }, 15.0),
            green_mat,
        ));
        scene.add_object(Object::sphere(
            Sphere::new(Point3D { x: 40.0, y: 30.0, z: -2.0 }, 20.0),
            yellow_mat,
        ));
        scene.add_object(Object::aa_box(
            AABox::new(Point3D { x: 10.0, y: 10.0, z: 10.0 }, Point3D { x: 50.0, y: 50.0, z: 50.0 }),
            red_mat.clone(),
        ));

        // floor and back
        scene.add_object(Object::plane(
            Plane::new(Point3D { x: 0.0, y: -50.0, z: 0.0 }, Vector3D { x: 0.0, y: 1.0, z: 0.0 }),
            grey_mat.clone(),
        ));
        scene.add_object(Object::plane(
            Plane::new(Point3D { x: 0.0, y: 0.0, z: 150.0 }, Vector3D { x: 0.0, y: 0.0, z: -1.0 }),
            grey_mat,
        ));

        scene.add_light(Light::point_light(Point3D { x: 0.0, y: 100.0, z: 0.0 }, 2.0));
        scene.add_light(Light::point_light(Point3D { x: 0.0, y: 0.0, z: -50.0 }, 2.0));

        scene
    }
}
