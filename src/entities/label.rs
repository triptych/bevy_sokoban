use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn create_labels(
    commands: &mut Commands,
    gameplay: &Res<Gameplay>,
    asset_server: &Res<AssetServer>,
) {
    let font_handle = asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap();
    let font_size = 20.0;

    commands
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: gameplay.state.to_string(),
                font: font_handle,
                style: TextStyle {
                    font_size,
                    color: Color::BLACK,
                },
            },
            ..Default::default()
        })
        .with(Label {
            label_type: LabelType::GameplayState,
        })
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(font_size),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                value: gameplay.moves_count.to_string(),
                font: font_handle,
                style: TextStyle {
                    font_size,
                    color: Color::BLACK,
                },
            },
            ..Default::default()
        })
        .with(Label {
            label_type: LabelType::MovesCount,
        })
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(font_size * 2.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                value: "FPS: ".to_string(),
                font: font_handle,
                style: TextStyle {
                    font_size,
                    color: Color::BLACK,
                },
            },
            ..Default::default()
        })
        .with(Label {
            label_type: LabelType::FPS,
        });
}
