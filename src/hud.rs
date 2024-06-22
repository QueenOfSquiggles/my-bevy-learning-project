use bevy::prelude::*;
use style_description::StyleDescription;

use crate::ui_builder::*;

// This plugin is being tested with isolation by only including itself and `DefaultPlugins`
// eg:
// ```
// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_plugins(hud::HUDPlugin)
//         .run();
// }
// ```

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_hud)
            .add_systems(Update, handle_buttons);
    }
}

#[derive(Default, Clone, Copy)]
enum ButtonID {
    #[default]
    Play,
    Options,
    Credits,
    Quit,
}

impl ButtonID {
    fn get_name(&self) -> &str {
        match self {
            ButtonID::Play => "Play",
            ButtonID::Options => "Options",
            ButtonID::Credits => "Credits",
            ButtonID::Quit => "Quit",
        }
    }
}

#[derive(Component)]
struct ButtonIdentity {
    id: ButtonID,
}

fn create_hud(mut commands: Commands, assets: ResMut<AssetServer>) {
    // let button_image = assets.load("textures/button_square_depth_gloss.png");

    let slicer = TextureSlicer {
        border: // BorderRect::square(4.),
        BorderRect {
            left: 4.,
            right: 4.,
            top: 5.,
            bottom: 8.,
        },
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    commands.spawn(Camera2dBundle::default());
    let layout = UiBuilder::start_layout()
        .node("root", &[])
        .start_children()
        .node("menu", &["column"])
        .start_children()
        .node("btn_group", &["column"])
        .start_children()
        .node("btn_play", &["btn"])
        .node("btn_options", &["btn"])
        .node("btn_credits", &["btn"])
        .node("btn_quit", &["btn"]);
    info!("GUI Layout  {:?}", layout);

    let mut style_builder = layout.finish_layout();
    let style = style_builder
        .by_name(
            "root",
            StyleDescription {
                width: Some(Val::Percent(100.)),
                height: Some(Val::Percent(100.)),
                ..default()
            },
        )
        .by_tag(
            &["column"],
            StyleDescription {
                flex_direction: Some(FlexDirection::Column),
                align_items: Some(AlignItems::Center),
                justify_content: Some(JustifyContent::Center),
                ..default()
            },
        )
        .by_tag(&["btn"], StyleDescription { ..default() });
    // info!("GUI Styled  {:?}", style);

    // commands
    //     .spawn(NodeBundle {
    //         style: Style {
    //             // display: Display::Flex,
    //             width: Val::Percent(45.),
    //             height: Val::Percent(100.),
    //             align_self: AlignSelf::FlexEnd,
    //             margin: UiRect::left(Val::Percent(15.)),
    //             padding: UiRect::all(Val::Percent(3.)),
    //             flex_direction: FlexDirection::Column,
    //             justify_content: JustifyContent::FlexEnd,
    //             align_items: AlignItems::Center,
    //             ..default()
    //         },
    //         background_color: Color::WHITE.into(),
    //         ..default()
    //     })
    //     .with_children(|parent| {
    //         for (width, height, id) in [
    //             (320., 64., ButtonID::Play),
    //             (320., 64., ButtonID::Options),
    //             (320., 64., ButtonID::Credits),
    //             (320., 64., ButtonID::Quit),
    //         ] {
    //             parent
    //                 .spawn((
    //                     ButtonBundle {
    //                         style: Style {
    //                             width: Val::Px(width),
    //                             height: Val::Px(height),
    //                             justify_content: JustifyContent::Center,
    //                             align_items: AlignItems::Center,
    //                             margin: UiRect::all(Val::Px(3.)),
    //                             ..default()
    //                         },

    //                         image: UiImage::new(button_image.clone()),
    //                         ..default()
    //                     },
    //                     ImageScaleMode::Sliced(slicer.clone()),
    //                     ButtonIdentity { id },
    //                 ))
    //                 .with_children(|build| {
    //                     build.spawn(TextBundle::from_section(
    //                         id.get_name(),
    //                         TextStyle {
    //                             font_size: 16.0,
    //                             color: Color::BLACK,
    //                             ..default()
    //                         },
    //                     ));
    //                 });
    //         }
    //     });
}

#[allow(clippy::type_complexity)]
fn handle_buttons(
    interaction_query: Query<(&Interaction, &ButtonIdentity), (Changed<Interaction>, With<Button>)>,
) {
    for (i, id) in interaction_query.iter() {
        if *i == Interaction::Pressed {
            let name = match id.id {
                ButtonID::Play => "Play Button",
                ButtonID::Options => "Options Button",
                ButtonID::Credits => "Credits Button",
                ButtonID::Quit => "Quit Button",
            };
            println!("{name} was pressed!")
        }
    }
}
