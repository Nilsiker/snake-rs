use bevy::prelude::*;

#[derive(Event)]
pub struct TickEvent;

#[derive(Resource)]
struct TickTimer(Timer);

pub struct TickPlugin {
    ms: usize,
}
impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TickEvent>()
            .insert_resource(TickTimer(Timer::from_seconds(
                self.ms as f32 / 1000f32,
                TimerMode::Repeating,
            )))
            .add_systems(Update, tick);
    }
}

impl TickPlugin {
    pub fn new(ms: usize) -> Self {
        Self { ms }
    }
}

fn tick(mut tick_events: EventWriter<TickEvent>, mut timer: ResMut<TickTimer>, time: Res<Time>) {
    if timer.0.finished() {
        timer.0.reset();
        tick_events.send(TickEvent);
    } else {
        timer.0.tick(time.delta());
    }
}
