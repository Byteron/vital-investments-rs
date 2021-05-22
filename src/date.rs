use bevy::prelude::*;

pub struct Date(pub i32);
pub struct DateTimer(pub Timer);
pub struct DateTickEvent(pub i32);

pub struct DateText;

pub fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexStart,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "Date: ",
                TextStyle {
                    font: assets.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(DateText);
}

pub fn update_text(
    date: Res<Date>,
    mut reader: EventReader<DateTickEvent>,
    mut query: Query<&mut Text, With<DateText>>,
) {
    for _ in reader.iter() {
        let day = 1 + date.0 % 30;
        let month = 1 + (date.0 / 30) % 12;
        let year = 2020 + date.0 / 360;

        for mut text in query.iter_mut() {
            text.sections.get_mut(0).unwrap().value = format!("Date: {}.{}.{}", day, month, year);
        }
    }
}

pub fn tick(
    time: Res<Time>,
    mut date: ResMut<Date>,
    mut timer: ResMut<DateTimer>,
    mut writer: EventWriter<DateTickEvent>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        date.0 += 1;
        writer.send(DateTickEvent(date.0));
    }
}
