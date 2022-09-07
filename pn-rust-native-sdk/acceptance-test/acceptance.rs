use tokio;
use cucumber::{WorldInit};
mod world;
use world::World;

mod hooks;

mod steps;
#[allow(unused_imports)]
use steps::*;

#[tokio::main]
async fn main() {
    let runner = World::cucumber()
    .before(|_feature, _rule, _scenario, _world| {
        return Box::pin(hooks::before(_scenario, _world));
    })
    .after(|_feature, _rule, _scenario, _world| {
        return Box::pin(hooks::after(_scenario, _world));
    });

    runner.run_and_exit("acceptance-test/features").await;
}