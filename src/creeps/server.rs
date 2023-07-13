use ambient_api::{
    animation::PlayClipFromUrlNode,
    asset, 
    concepts::make_transformable,
    entity::add_component, 
    components::core::{
        transform::{translation, local_to_world, rotation}, physics::{character_controller_height, character_controller_radius, physics_controlled}, app::name
    },
    prelude::{
        Vec2,
        vec3, Quat, Entity, EntityId
    }, main, 
};

const INIT_POS: f32 = std::f32::consts::FRAC_PI_2;

#[main]
pub fn main() {
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


}

fn create_ranged_creep(init_pos: Vec2) -> EntityId{
    Entity::new()
        .with_merge(make_transformable())
        .with(translation(), vec3(init_pos.x, init_pos.y, 3.0))
        .with(character_controller_height(), 1.0)
        .with(character_controller_radius(), 0.5)
        .with_default(physics_controlled())
        .with_default(local_to_world())
        .with(rotation(), Quat::from_rotation_z(-INIT_POS))
        .with(name(), "Ranged Creep".to_string())
        .spawn()
}