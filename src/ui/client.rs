use ambient_api::components::core::app::window_logical_size;
use ambient_api::prelude::*;
#[main]
pub fn main() {
    // let now = Arc::new(Mutex::new(time()));
    App.el().spawn_interactive();

    ambient_api::messages::Frame::subscribe(move |_| {
        let player_id = player::get_local();

        let (delta, input) = input::get_delta();
        let window_size =
            entity::get_component(entity::resources(), window_logical_size()).unwrap();
        let mouse_pos = input.mouse_position;
        let cam_opt = entity::get_component(player_id, components::cam());
        if let Some(cam) = cam_opt {
            let mut cam_move = vec2(0.0, 0.0);

            if mouse_pos.x < window_size.x as f32 / 4.0 {
                cam_move.x = -1.0;
            } else if mouse_pos.x > window_size.x as f32 / 4.0 * 3.0 {
                cam_move.x = 1.0;
            }

            if mouse_pos.y < window_size.y as f32 / 4.0 {
                cam_move.y = 1.0;
            } else if mouse_pos.y > window_size.y as f32 / 4.0 * 3.0 {
                cam_move.y = -1.0;
            }

            if cam_move != Vec2::ZERO {
                messages::CamMove::new(cam_move, player_id).send_server_unreliable();
            }
        } else {
            return;
        }

        if delta.mouse_buttons.contains(&MouseButton::Left) {
            // wanna move the character!
            let cam = entity::get_component(player_id, components::cam()).unwrap();
            let ray = camera::screen_position_to_world_ray(cam, mouse_pos);
            messages::MouseLeftClick {
                ray_origin: ray.origin,
                ray_dir: ray.dir,
                player_id,
            }
            .send_server_unreliable();
        }
    });
}

#[element_component]
fn App(hooks: &mut Hooks) -> Element {
    let (screen, set_screen) = hooks.use_state(None);
    let (show, set_show) = hooks.use_state(true);

    let f = FocusRoot::el([PageScreen::el([
        ScreenContainer(screen).el(),
        // FlowColumn::el([
        Text::el("Select team:").with_margin_even(10.),
        Button::new("Join Team Mars", {
            let set_screen = set_screen.clone();
            let set_show = set_show.clone();
            let show = show.clone();
            move |_| {
                set_screen(Some(TeamMars::el(
                    cb({
                        let set_screen = set_screen.clone();
                        move || {
                            set_screen(None);
                        }
                    }),
                    cb({
                        let set_show = set_show.clone();
                        let show = show.clone();
                        move |r| {
                            let player_id = player::get_local();
                            messages::ChooseRole::new(player_id, r).send_server_reliable();
                            set_show(!show);
                        }
                    }),
                )))
            }
        })
        .el()
        .with_margin_even(10.),
        Button::new("Join Team Jupiter", move |_| {
            set_screen(Some(TeamJupiter::el(
                cb({
                    let set_screen = set_screen.clone();
                    move || {
                        set_screen(None);
                    }
                }),
                cb({
                    let set_show = set_show.clone();
                    let show = show.clone();
                    move |r| {
                        let player_id = player::get_local();
                        messages::ChooseRole::new(player_id, r).send_server_reliable();
                        // entity::add_component(player_id, components::role(), r);
                        set_show(!show);
                    }
                }),
            )))
        })
        .el()
        .with_margin_even(10.),
        // ]),
    ])]);
    if show {
        f
    } else {
        Element::new()
    }
}

#[element_component]
fn TeamMars(
    hooks: &mut Hooks,
    on_back: Cb<dyn Fn() + Sync + Send>,
    decision: Cb<dyn Fn(u32) + Sync + Send>,
) -> Element {
    let (screen, set_screen) = hooks.use_state(None);
    let (id, _) = hooks.use_state_with(|_| random::<u32>());
    PageScreen::el([
        ScreenContainer(screen).el(),
        // Text::el(format!("SubScreen {id}")),
        Button::new("Back", move |_| on_back())
            .el()
            .with_margin_even(10.),
        Button::new("Mars Hero 1", {
            // let set_screen = set_screen.clone();
            move |world| {
                decision(0);
            }
        })
        .el()
        .with_margin_even(10.),
    ])
}

#[element_component]
fn TeamJupiter(
    hooks: &mut Hooks,
    on_back: Cb<dyn Fn() + Sync + Send>,
    decision: Cb<dyn Fn(u32) + Sync + Send>,
) -> Element {
    let (screen, set_screen) = hooks.use_state(None);
    // let (id, _) = hooks.use_state_with(|_| random::<u32>());
    PageScreen::el([
        ScreenContainer(screen).el(),
        // Text::el(format!("SubScreen {id}")),
        Button::new("Back", move |_| on_back())
            .el()
            .with_margin_even(10.),
        Button::new("Jupiter Hero A", {
            // let set_screen = set_screen.clone();
            move |_| decision(1)
        })
        .el()
        .with_margin_even(10.),
    ])
}
