use ambient_api::components::core::{
    app::window_logical_size,
    layout::{docking_bottom, fit_horizontal_none, fit_vertical_none, height, width},
    rect::{
        background_color, border_color, border_radius, border_thickness, line_from, line_to,
        line_width,
    },
    transform::{lookat_target, translation},
};
use ambient_api::prelude::*;

#[main]
pub fn main() {
    App.el().spawn_interactive();
    // messages::MakeMap::subscribe(|_, data| {
    //     let id = data.player_id;
    //     let role = data.role;
    //     println!("Map: role: {}", role);
    //     if id == player::get_local() {
    //         App { role }.el().spawn_interactive()
    //     }
    // });
}

#[element_component]
fn App(hooks: &mut Hooks) -> Element {
    let (show, set_show) = hooks.use_state(false);
    let (role, set_role) = hooks.use_state(None);
    let (player_pos, set_player_pos) = hooks.use_state(vec2(0.0, 0.0));
    let (camera_pos, set_camera_pos) = hooks.use_state(vec2(0.0, 0.0));
    // let screen_size = hooks.use_query(window_logical_size());
    hooks.use_frame(move |w| {
        let local = player::get_local();
        let role = w.get(local, components::role());
        if let Ok(r) = role {
            set_role(Some(r));
            set_show(true);
        } else {
            return;
        }
        let model = w.get(local, components::hero_model()).unwrap();
        let pos = w.get(model, translation()).unwrap();
        let pos = pos.xy();
        let cam = w.get(local, components::cam()).unwrap();
        let cam_look = w.get(cam, lookat_target()).unwrap();
        let cam_look = cam_look.xy();
        match role.unwrap() % 2 {
            1 => {
                let home = vec2(-15.0, -15.0);
                let x = pos.y - home.x;
                let y = pos.x - home.y;
                let camx = cam_look.y - home.x;
                let camy = cam_look.x - home.y;
                set_camera_pos(vec2(camx / 30.0 * 70.0, 70.0 - camy / 30.0 * 70.0));
                set_player_pos(vec2(x / 30.0 * 70.0, 70.0 - y / 30.0 * 70.0));
            }
            _ => {
                let home = vec2(15.0, 15.0);
                let x = (home.x - pos.y) / 30.0 * 70.0;
                let y = (home.y - pos.x) / 30.0 * 70.0;
                let camx = (home.x - cam_look.y) / 30.0 * 70.0;
                let camy = (home.y - cam_look.x) / 30.0 * 70.0;
                set_camera_pos(vec2(camx, 70.0 - camy));
                set_player_pos(vec2(x, 70.0 - y));
            }
        };
    });

    let cam_rect = Rectangle::el()
        .with(width(), 30.)
        .with(height(), 30.)
        .with(background_color(), vec4(0.8, 0.8, 0.8, 0.2))
        .with(
            translation(),
            camera_pos.extend(-0.05) - vec3(15.0, 15.0, 0.0),
        );

    let player_rect = Rectangle::el()
        .with(width(), 5.)
        .with(height(), 5.)
        .with(background_color(), vec4(0.8, 0.8, 0.8, 1.))
        .with(translation(), player_pos.extend(-0.1));

    // .with_margin_even(5.0);

    let (red_pos, blue_pos) = {
        if let Some(role) = role {
            match role % 2 {
                1 => (vec3(70.0, 5.0, -0.1), vec3(5.0, 70.0, -0.1)),
                _ => (vec3(5.0, 70.0, -0.1), vec3(70.0, 5.0, -0.1)),
            }
        } else {
            (vec3(0.0, 0.0, -0.1), vec3(0.0, 0.0, -0.1))
        }
    };

    let red_dot = Rectangle::el()
        .with(width(), 5.)
        .with(height(), 5.)
        .with(background_color(), vec4(1., 0., 0., 1.))
        .with(translation(), red_pos)
        .with_margin_even(5.0);

    let blue_dot = Rectangle::el()
        .with(width(), 5.)
        .with(height(), 5.)
        .with(background_color(), vec4(0., 0., 1., 1.))
        .with(translation(), blue_pos)
        .with_margin_even(5.0);

    let map = Rectangle::el()
        .with(width(), 80.)
        .with(height(), 80.)
        .with(background_color(), vec4(0.5, 0.5, 0.5, 1.))
        .with_margin_even(10.0)
        .children(vec![red_dot, blue_dot, player_rect, cam_rect]);
    let canvas = WindowSized::el([Dock::el([FlowRow::el([map])
        .with_background(vec4(1., 1., 1., 0.02))
        .with_default(fit_vertical_none())
        .with_default(fit_horizontal_none())
        .with_default(docking_bottom())
        .with(height(), 100.)
        .with_margin_even(10.)])
    .with_background(vec4(0., 0., 0., 0.00))
    .with_padding_even(STREET)]);
    // .with(translation(), vec3(0.0, 0.0, 0.0));
    if show {
        canvas
        // Element::new()
    } else {
        Element::new()
    }
}
