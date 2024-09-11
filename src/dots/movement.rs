use bevy::prelude::*;
use crate::Player;
use crate::constants::{SPEED};

#[derive(Component)]
pub struct MovementState {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
}

// run this on FixedUpdate
pub fn position_update_system(
    time: Res<Time>,
    mut q_movement: Query<&mut MovementState>,
) {
    for mut state in &mut q_movement {
        // Reborrow `state` to mutably access both of its fields
        let state = &mut *state;

        // Compute the new position.
        // (`delta_seconds` always returns the fixed timestep
        // duration, if this system is added to `FixedUpdate`)
        state.position += state.velocity * time.delta_seconds();
    }
}

// run this on Update
pub fn transform_movement_extrapolate_velocity(
    fixed_time: Res<Time<Fixed>>,
    mut q_movement: Query<(
        &mut Transform,
        &MovementState,
    )>,
) {
    for (mut xf, state) in &mut q_movement {
        let a = fixed_time.overstep_fraction();
        let future_position = state.position + state.velocity * fixed_time.delta_seconds();
        xf.translation = state.position.lerp(future_position, a);
    }
}


pub fn input_system(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut objects: Query<&mut MovementState, With<Player>>,
) {
    for mut state in &mut objects {
        let state = &mut *state;
        let delta = SPEED * time.delta_seconds();

        let last_velocity = state.velocity.clone();

        for key in keys.get_pressed() {
            match key {
                KeyCode::ArrowUp => state.velocity.y += delta,
                KeyCode::ArrowDown => state.velocity.y -= delta,
                KeyCode::ArrowLeft => state.velocity.x -= delta,
                KeyCode::ArrowRight => state.velocity.x += delta,
                _ => {}
            };
        }

        state.acceleration = (last_velocity - state.velocity) * delta;
        // 
        // 
        // if keys.pressed(KeyCode::ArrowUp) {
        //     state.velocity.y += delta;
        // }
        // 
        // if keys.pressed(KeyCode::ArrowDown) {
        //     state.velocity.y -= delta;
        // }
        // 
        // if !(keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::ArrowDown)) {
        //     state.acceleration.y = 0.0;
        // }
        // 
        // if keys.pressed(KeyCode::ArrowRight) {
        //     state.velocity.x += delta;
        // }
        // if keys.pressed(KeyCode::ArrowLeft) {
        //     state.velocity.x -= delta;
        // }
    }
}

