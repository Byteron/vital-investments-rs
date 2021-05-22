use bevy::prelude::*;

pub struct Date(pub i32);
pub struct DateTimer(pub Timer);
pub struct DateTickEvent(pub i32);

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
