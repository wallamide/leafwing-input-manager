use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_startup_system(spawn_player)
        .add_system(say_hello)
        .run();
}

#[derive(Actionlike, Clone, Debug)]
enum Action {
    Hello,
}

impl Action {
    fn default_binding(&self) -> UserInput {
        match self {
            Action::Hello => KeyCode::Space.into(),
        }
    }

    fn input_map() -> InputMap<Action> {
        let mut input_map = InputMap::default();

        for action in Action::variants() {
            input_map.insert(action.default_binding(), action);
        }

        input_map
    }

    fn starting_cooldown(&self) -> Cooldown {
        match self {
            Action::Hello => Cooldown::from_secs(5.0),
        }
    }

    fn cooldowns() -> Cooldowns<Action> {
        let mut cooldowns = Cooldowns::default();

        for action in Action::variants() {
            cooldowns.set(action.starting_cooldown(), action);
        }

        cooldowns
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn_bundle(InputManagerBundle::<Action> {
        action_state: ActionState::default(),
        input_map: Action::input_map(),
        cooldowns: Action::cooldowns(),
    });
}

fn say_hello(mut query: Query<(&ActionState<Action>, &mut Cooldowns<Action>)>) {
    let (action_state, mut cooldowns) = query.single_mut();

    if action_state.pressed(Action::Hello) && cooldowns.ready(Action::Hello) {
        info!("Hello!");
        dbg!(cooldowns.clone());
        cooldowns.trigger(Action::Hello);
        dbg!(cooldowns);
    }
}
