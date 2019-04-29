//! An example showing how to create a dispatcher inside of a State.

use amethyst::{
    ecs::{Dispatcher, DispatcherBuilder, System},
    prelude::*,
    shrev::EventChannel,
    Error,
};

use std::marker::PhantomData;

struct StateA;

pub struct TestSystem;
impl<'s> System<'s> for TestSystem {
    type SystemData = ();

    fn run(&mut self, data: Self::SystemData) {
        println!("TestSystem::run");
    }
}

impl SimpleState for StateA {
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        println!("StateA::update()");
        // Shows how to push a `Trans` through the event queue.
        // If you do use TransQueue, you will be forced to use the 'static lifetime on your states.
        data.world
            .write_resource::<EventChannel<TransEvent<GameData<'static, 'static>, StateEvent>>>()
            .single_write(Box::new(|| Trans::Push(Box::new(
                StateB::new(
                    DispatcherBuilder::new()
                        .with(TestSystem, "test_system", &[])
                        .build()
                )
            )
            )));

        // You can also use normal Trans at the same time!
        // Those will be executed before the ones in the EventChannel
        // Trans::Push(Box::new(StateB::default()))

        Trans::None
    }
}

/// StateB isn't Send + Sync
struct StateB<'a> {
    dispatcher: Dispatcher<'static, 'static>,
    _phantom: &'a PhantomData<()>,
}

impl<'a> StateB<'a> {
    fn new(dispatcher: Dispatcher<'static, 'static>) -> Self {
        StateB {
            dispatcher,
            _phantom: &PhantomData,
        }
    }
}

impl<'a> SimpleState for StateB<'a> {
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        println!("StateB::update()");
        self.dispatcher.dispatch(&mut data.world.res);
        Trans::Quit
    }
}

fn main() -> Result<(), Error> {
    amethyst::start_logger(Default::default());
    let mut game = Application::build("./", StateA)?.build(GameDataBuilder::default())?;
    game.run();
    Ok(())
}
