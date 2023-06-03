pub use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct BlueBall;

#[derive(Resource)]
pub struct GameInfo {
    pub is_won: bool,
    pub wins: i32,
    pub collected: i32,
}
#[derive(Component)]
pub struct Reboot;

#[derive(Component)]
pub struct TextChanges;

#[derive(Component)]
pub struct Cursor;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    Menu,
    NewGame,
    InGame,
    Paused,
    #[default]
    Splash,
}

#[derive(Component)]
pub struct FPSTimer {
    pub elapsed: f32,
}

pub fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}

#[derive(Resource)]
pub struct Jump {
    pub jumping: bool,
    pub elapsed: f32,
    pub avalible: bool,
}

#[derive(Component)]
pub struct Cube;

#[derive(Resource)]
pub struct Pause {
    pub paused: bool,
}


#[derive(Component)]
pub struct Visible {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}


#[derive(Component)]
pub struct Renderable;


// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}


// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Fov(pub u32);


// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    Settings,
    SettingsDisplay,
    SettingsSound,
    SettingsFov,
    #[default]
    Disabled,
}


// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
pub struct HitTimer(pub Timer);

impl Default for HitTimer {
    fn default() -> Self {
        HitTimer(Timer::from_seconds(0.5, TimerMode::Once))
    }
}

#[derive(Resource)]
pub struct Hit {
    pub hit: bool,
}
