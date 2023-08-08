use ambient_api::{
    animation::{AnimationPlayer, BlendNode, PlayClipFromUrlNode},
    components::core::{
        animation::apply_animation_player,
        app::{main_scene, name},
        camera::aspect_ratio_from_window,
        ecs::{children, parent},
        physics::{
            angular_velocity, character_controller_height, character_controller_radius,
            cube_collider, dynamic, linear_velocity, physics_controlled, plane_collider,
        },
        player::player,
        prefab::prefab_from_url,
        primitives::{cube, quad},
        rendering::color,
        transform::{local_to_parent, local_to_world, lookat_target, rotation, scale, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    entity::add_component,
    prelude::*,
};

const INIT_POS: f32 = std::f32::consts::FRAC_PI_2;

macro_rules! idle_animation_state { () => { vec![1.0, 0.0, 0.0] }; }
macro_rules! walk_animation_state { () => { vec![0.0, 1.0, 0.0] }; }
macro_rules! attack_animation_state { () => { vec![0.0, 0.0, 1.0] }; }

#[main]
pub fn main() {
    messages::ChooseRole::subscribe(|source, msg| {
        let idle = PlayClipFromUrlNode::new(
            asset::url("assets/anim/Idle.fbx/animations/mixamo.com.anim").unwrap(),
        );
        let walk = PlayClipFromUrlNode::new(
            asset::url("assets/anim/Walking.fbx/animations/mixamo.com.anim").unwrap(),
        );
        let attack = PlayClipFromUrlNode::new(
            asset::url("assets/anim/Standing Torch Melee Attack 01.fbx/animations/mixamo.com.anim")
                .unwrap(),
        );
        let idle_player = AnimationPlayer::new(&idle);
        let walk_player = AnimationPlayer::new(&walk);
        let attack_player = AnimationPlayer::new(&attack);

        // this is waiting for the ui server module to send a message
        println!("{:?} chose role {:?} in player module", source, msg.role);

        let player_id = msg.player_id;
        let role = msg.role;

        let minus = match role % 2 {
            0 => 1.0,
            _ => -1.0,
        };
        let init_pos = match role {
            0 => {
                // Mars Hero A
                // set model, init pos
                vec2(15.0, 15.0) - random::<Vec2>() * 3.0
            }
            1 => {
                // Jupyter Hero A
                vec2(-15.0, -15.0) + random::<Vec2>() * 3.0
            }
            _ => unimplemented!(),
        };

        let cam = Entity::new()
            .with_merge(make_perspective_infinite_reverse_camera())
            .with(aspect_ratio_from_window(), EntityId::resources())
            .with_default(main_scene())
            .with(
                translation(),
                vec3(init_pos.x + 10.0 * minus, init_pos.y, 15.0),
            )
            .with(lookat_target(), vec3(init_pos.x, init_pos.y, 0.))
            .spawn();

        let model = Entity::new()
            .with_merge(make_transformable())
            .with(translation(), vec3(init_pos.x, init_pos.y, 3.0))
            .with(character_controller_height(), 2.0)
            .with(character_controller_radius(), 0.5)
            .with_default(physics_controlled())
            .with_default(local_to_world())
            .with(rotation(), Quat::from_rotation_z(-INIT_POS))
            .with(name(), "Hero".to_string())
            .spawn();
        let anim_model = Entity::new()
            .with_merge(make_transformable())
            .with_default(dynamic())
            .with(parent(), model)
            .with(
                prefab_from_url(),
                asset::url("assets/model/Ganfaul M Aure.fbx").unwrap(),
            )
            .with_default(local_to_parent())
            .with_default(local_to_world())
            .with(translation(), vec3(0.0, 0.0, 0.8))
            .spawn();
        add_component(anim_model, apply_animation_player(), idle_player.0);
        entity::add_component(anim_model, components::anim_state(), idle_animation_state!());

        entity::add_component(model, children(), vec![anim_model]);
        entity::add_component(player_id, components::role(), role);
        entity::add_component(player_id, components::hero_model(), model);
        entity::add_component(player_id, components::cam(), cam);
        // entity::add_component(player_id, components::ground(), ground);
        entity::add_component(player_id, components::anim_model(), anim_model);
        entity::add_component(player_id, components::target_pos(), init_pos);
        query((player(), components::hero_model())).each_frame({
            move |list| {

                for (player_id, (_, model)) in list {
                    let anim_model =
                        entity::get_component(player_id, components::anim_model()).unwrap();
                    let anim_state =
                        entity::get_component(anim_model, components::anim_state()).unwrap();

                    if anim_state == attack_animation_state!() {
                        continue;
                    }
                    let current_pos = entity::get_component(model, translation()).unwrap();
                    let target_pos =
                        entity::get_component(player_id, components::target_pos()).unwrap();
                    let diff = target_pos - current_pos.xy();

                    if diff.length() < 1.0 {
                        // if current_pos.xy() == vec2(0.0, 0.0) {
                        physics::move_character(model, vec3(0., 0., -0.1), 0.01, delta_time());
                        // }
                        if entity::get_component(anim_model, components::anim_state()).unwrap()
                            != attack_animation_state!()
                        {
                            entity::set_component(
                                anim_model,
                                apply_animation_player(),
                                idle_player.0,
                            );
                            entity::set_component(
                                anim_model,
                                components::anim_state(),
                                idle_animation_state!(),
                            );
                        };
                        continue;
                    }

                    let target_direction = diff;
                    let initial_direction: Vec2 = Vec2::new(1.0, 0.0);
                    let dot = initial_direction.dot(target_direction);
                    let det = initial_direction.x * target_direction.y
                        - initial_direction.y * target_direction.x;
                    let angle = det.atan2(dot);
                    let rot: Quat = Quat::from_rotation_z(angle - INIT_POS);
                    entity::set_component(model, rotation(), rot);

                    let speed = 0.05;
                    let displace = diff.normalize_or_zero() * speed;

                    if anim_state != walk_animation_state!() {
                        entity::set_component(anim_model, apply_animation_player(), walk_player.0);
                        entity::set_component(
                            anim_model,
                            components::anim_state(),
                            walk_animation_state!(),
                        );
                    }
                    let collision = physics::move_character(
                        model,
                        vec3(displace.x, displace.y, -0.1),
                        0.01,
                        delta_time(),
                    );

                    if collision.side {
                        entity::set_component(
                            player_id,
                            components::target_pos(),
                            current_pos.xy(),
                        );
                        entity::set_component(anim_model, apply_animation_player(), idle_player.0);
                        entity::set_component(
                            anim_model,
                            components::anim_state(),
                            vec![1.0, 0.0, 0.0],
                        );
                    } else {
                        let cam = entity::get_component(player_id, components::cam()).unwrap();
                        // messages::PlayerWalk::new(model, cam, player_id)
                        // .send_client_broadcast_unreliable();
                    }
                }
            }
        });
    });
    messages::MouseLeftClick::subscribe(|_, msg| {
        let result = physics::raycast_first(msg.ray_origin, msg.ray_dir);
        if let Some(hit) = result {
            if entity::has_component(hit.entity, plane_collider()) {
                let target_pos = vec2(hit.position.x, hit.position.y);
                entity::set_component(msg.player_id, components::target_pos(), target_pos);
                let crossx = make_transformable()
                    .with_default(quad())
                    .with(scale(), vec3(0.05, 0.3, 0.1))
                    .with(color(), vec4(0.2, 0.5, 0.2, 1.0))
                    .with(translation(), vec3(target_pos.x, target_pos.y, 0.2))
                    .spawn();
                let crossy = make_transformable()
                    .with_default(quad())
                    .with(scale(), vec3(0.3, 0.05, 0.1))
                    .with(color(), vec4(0.2, 0.5, 0.2, 1.0))
                    .with(translation(), vec3(target_pos.x, target_pos.y, 0.2))
                    .spawn();
                run_async(async move {
                    sleep(0.2).await;
                    entity::despawn(crossx);
                    entity::despawn(crossy);
                })
            }
        }
    });

    messages::CamMove::subscribe(|_, msg| {
        // println!("cam move {:?}", msg);
        let c = entity::get_component(msg.player_id, components::cam()).unwrap();
        let role = entity::get_component(msg.player_id, components::role()).unwrap();
        let minus = match role % 2 {
            0 => 1.0,
            _ => -1.0,
        };
        let x = msg.direction.x;
        let y = msg.direction.y;

        let lookpos = entity::get_component(c, lookat_target()).unwrap();
        let new_lookpos = lookpos + vec3(y, x, 0.0) * -0.1 * minus;
        entity::set_component(c, lookat_target(), new_lookpos);
        let pos = entity::get_component(c, translation()).unwrap();
        entity::set_component(c, translation(), pos + vec3(y, x, 0.0) * -0.1 * minus);
    });
}
