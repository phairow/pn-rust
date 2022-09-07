use cucumber::{given};
use crate::World;

#[given(regex = r"the demo keyset")]
async fn demo_keyset(w: &mut World) {
    w.keyset = Some(w.demo_keyset.clone());

    println!("the demo keyset");
    println!("{:#?}", w.keyset);
}