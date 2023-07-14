use ambient_api::{
    animation::{PlayClipFromUrlNode, AnimationPlayer},
    asset, 
    concepts::make_transformable,
    entity::{add_component, self}, 
    components::core::{
        transform::{translation, local_to_world, rotation, local_to_parent}, physics::{character_controller_height, character_controller_radius, physics_controlled, dynamic}, app::name, ecs::{parent, children}, prefab::prefab_from_url, animation::apply_animation_player
    },
    prelude::{
        Vec2,
        vec3, Quat, Entity, EntityId
    }, main, 
};

const INIT_POS: f32 = std::f32::consts::FRAC_PI_2;

#[main]
pub fn main() {
    create_ranged_creep(Vec3{x:2., y:2., z:-0.8});


}

fn create_ranged_creep(init_pos: Vec3) -> EntityId{
    let ranged_idle = PlayClipFromUrlNode::new(
        asset::url("assets/model/Yeti.fbx/CharacterArmature/Idle.anim").unwrap(),
    );
    let ranged_walk = PlayClipFromUrlNode::new(
        asset::url("assets/anim/Yeti.fbx/CharacterArmature/Run.anim").unwrap(),
    );
    let ranged_attack = PlayClipFromUrlNode::new(
        asset::url("assets/anim/Yeti.fbx/CharacterArmature/Punch.anim").unwrap(),
    );
    let ranged_death: PlayClipFromUrlNode = PlayClipFromUrlNode::new(
        asset::url("assets/model/Yeti.fbx/CharacterArmature/Death.anim").unwrap(),
    );

    let idle_player = AnimationPlayer::new(&ranged_idle);
    let walk_player = AnimationPlayer::new(&ranged_walk);
    let attack_player = AnimationPlayer::new(&ranged_attack);
    let death_player = AnimationPlayer::new(&ranged_death);


    let model = Entity::new()
        .with_merge(make_transformable())
        .with(translation(), vec3(init_pos.x, init_pos.y, init_pos.z))
        .with(character_controller_height(), 0.5)
        .with(character_controller_radius(), 0.5)
        .with_default(physics_controlled())
        .with_default(local_to_world())
        .with(rotation(), Quat::from_rotation_z(-INIT_POS))
        .with(name(), "Ranged Creep".to_string())
        .spawn();

    let anim_model = Entity::new()
        .with_merge(make_transformable())
        .with_default(dynamic())
        .with(parent(), model)
        .with(
            prefab_from_url(),
            asset::url("assets/model/Yeti.fbx").unwrap(),
        )
        .with_default(local_to_parent())
        .with_default(local_to_world())
        .with(translation(), vec3(0.0, 0.0, 0.8))
        .spawn();

    add_component(anim_model, apply_animation_player(), idle_player.0);
    entity::add_component(anim_model, components::anim_state(), vec![1.0, 0.0]);

    entity::add_component(model, children(), vec![anim_model]);
    entity::add_component(model, components::anim_model(), anim_model);
    entity::add_component(model, components::target_pos(), Vec2{x:init_pos.x, y:init_pos.y});

    model
}