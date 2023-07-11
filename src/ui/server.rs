use ambient_api::prelude::*;

#[main]
pub fn main() {
    messages::ChooseRole::subscribe(|source, msg| {
        println!("{:?} chose role {:?}", source, msg.role);
        messages::ChooseRole { ..msg }.send_local_broadcast(false);
        // we get the info, then we need to broadcast it to other server modules
    });

    messages::MouseLeftClick::subscribe(|_, msg| {
        messages::MouseLeftClick { ..msg }.send_local_broadcast(false);
    });

    messages::CamMove::subscribe(|_, msg| {
        messages::CamMove { ..msg }.send_local_broadcast(false);
    });
}
