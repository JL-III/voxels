use bevy::prelude::*;

use crate::app_state::state::AppState;

use super::events::CommandDispatchEvent;

#[derive(Component)]
pub struct CommandInterface {}

pub fn spawn_command_interface(mut commands: Commands) {
    let _command_interface_entity = build_command_interface(&mut commands);
}

pub fn despawn_command_interface(
    mut commands: Commands,
    command_interface_query: Query<Entity, With<CommandInterface>>,
) {
    if let Ok(command_interface_entity) = command_interface_query.get_single() {
        commands
            .entity(command_interface_entity)
            .despawn_recursive();
    }
}

pub fn update_command_interface(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut event_reader_char: EventReader<ReceivedCharacter>,
    mut command_dispatch_event_writer: EventWriter<CommandDispatchEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    mut string: Local<String>,
    mut command_interface_query: Query<&mut Text, With<CommandInterface>>,
) {
    if let Some(key) = keyboard_input.get_just_pressed().next() {
        match key {
            KeyCode::Return | KeyCode::NumpadEnter => {
                command_dispatch_event_writer.send(CommandDispatchEvent {
                    command: string.to_string(),
                });
                next_app_state.set(AppState::Game);
                string.clear();
            }
            KeyCode::Back => {
                string.pop();
            }
            KeyCode::Escape => {
                string.clear();
            }
            _ => {}
        }
    }
    for ev in event_reader_char.read() {
        // ignore control (special) characters
        if !ev.char.is_control() {
            string.push(ev.char);
        }
    }
    for mut text in &mut command_interface_query {
        text.sections[0].value = string.to_string();
    }
}

pub fn build_command_interface(commands: &mut Commands) -> Entity {
    let command_interface_entity = commands
        .spawn((
            TextBundle {
                background_color: BackgroundColor(Color::GRAY),
                style: Style {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(10.0),
                    ..default()
                },
                text: Text {
                    sections: vec![TextSection::new(
                        "commands",
                        TextStyle {
                            font_size: 32.0,
                            ..default()
                        },
                    )],
                    ..default()
                },
                ..Default::default()
            },
            CommandInterface {},
        ))
        .id();
    command_interface_entity
}