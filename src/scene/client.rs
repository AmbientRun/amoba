use ambient_api::prelude::*;

#[main]
pub fn main() {
    // messages::BlowSound::subscribe(|source, msg| {
    //     let emitter = msg.emitter;
    //     let player_id = player::get_local();
    //     // if let Some(cam) = entity::get_component(player_id, components::cam()) {
    //     spatial_audio::set_emitter(emitter);
    //     spatial_audio::set_listener(player_id);
    //     spatial_audio::play_sound_on_entity(
    //         asset::url("assets/explo_body_compressed_01.ogg").unwrap(),
    //         emitter,
    //     );
    //     // }
    // });
}
