use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        physics::{
            angular_velocity, cube_collider, dynamic, linear_velocity, physics_controlled,
            plane_collider, sphere_collider,
        },
        primitives::{cube, quad, sphere_radius},
        rendering::{cast_shadows, color, fog_density, light_diffuse, sky, sun, water},
        transform::{lookat_target, rotation, scale, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_sphere, make_transformable},
    prelude::*,
};

#[main]
pub fn main() {
    let mars_ball = Entity::new()
        .with_merge(make_sphere())
        .with_default(cast_shadows())
        .with(sphere_radius(), 1.)
        .with(sphere_collider(), 1.0)
        .with(translation(), vec3(15., 15., 1.))
        .with(color(), vec4(1.0, 0.0, 0.1, 1.))
        .with(components::health(), 100)
        .with(components::base_side(), 0)
        .spawn();

    let jupyter_ball = Entity::new()
        .with_merge(make_sphere())
        .with_default(cast_shadows())
        .with(sphere_collider(), 1.0)
        .with(sphere_radius(), 1.)
        .with(translation(), vec3(-15., -15., 1.))
        .with(color(), vec4(0., 0., 1., 1.))
        .with(components::health(), 100)
        .with(components::base_side(), 1)
        .spawn();

    query((components::health(), components::base_side())).each_frame(|list| {
        for (base_id, (health, side)) in list {
            if health <= 0 {
                println!("base {} destroyed", base_id);
                // messages::BlowSound::new(base_id).send_client_broadcast_unreliable();
                let c = if side == 0 {
                    vec4(1.0, 0.0, 0.1, 1.)
                } else {
                    vec4(0., 0., 1., 1.)
                };
                let pos = entity::get_component(base_id, translation()).unwrap();
                entity::despawn(base_id);
                run_async(async move {
                    // loop {

                    for _ in 0..20 {
                        // sleep(0.01).await;
                        let pos =
                            pos + vec3(random::<f32>(), random::<f32>(), random::<f32>()) * 0.3;
                        let size = vec3(0.3, 0.3, 0.1);
                        let rot = Quat::from_rotation_y(random::<f32>() * 3.14)
                            * Quat::from_rotation_x(random::<f32>() * 3.14);
                        let id = Entity::new()
                            .with_merge(make_transformable())
                            .with_default(cube())
                            .with(rotation(), rot)
                            .with_default(physics_controlled())
                            .with_default(cast_shadows())
                            .with(linear_velocity(), vec3(random(), random(), 15.0))
                            // .with(angular_velocity(), random::<Vec3>() * 1.0)
                            .with(cube_collider(), Vec3::ONE)
                            .with(dynamic(), true)
                            .with(scale(), random::<Vec3>() * size * 2.0)
                            .with(translation(), pos)
                            .with(color(), c)
                            .spawn();
                    }
                    // }
                });
            }
        }
    });

    Entity::new()
        .with_merge(make_transformable())
        .with_default(quad())
        .with(scale(), Vec3::ONE * 30.)
        .with_default(plane_collider())
        // .with(color(), vec4(1., 0., 0., 1.))
        .with(translation(), vec3(0., 0., 0.01))
        .spawn();

    Entity::new()
        .with_merge(make_transformable())
        .with_default(water())
        .with(scale(), Vec3::ONE * 2000.)
        .spawn();

    Entity::new()
        .with_merge(make_transformable())
        .with_default(sky())
        .spawn();

    // Entity::new()
    //     .with_merge(make_sphere())
    //     .with_default(cast_shadows())
    //     .with(sphere_radius(), 1.)
    //     .with(translation(), vec3(0., 0., 1.))
    //     .with(color(), vec4(1., 1., 1., 1.))
    //     .spawn();

    let sun = Entity::new()
        .with_merge(make_transformable())
        .with_default(sun())
        .with(rotation(), quat(0.0, 0.8875973, 0.0, -0.46063244))
        // .with(rotation(), Quat::from_rotation_y(2.6))
        .with_default(main_scene())
        .with(light_diffuse(), Vec3::ONE)
        .with(fog_density(), 0.)
        .spawn();
}
