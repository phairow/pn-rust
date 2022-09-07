use cucumber::{then, when};
use crate::World;

#[derive(Debug)]
struct SamplePublishmessage {
   title: String,
   text: String,
}

#[when(regex = r"I publish a message")]
async fn publish_message(w: &mut World) {
   let mut config = w.default_config.clone();

   config.publish_key = w.demo_keyset.publish_key.clone();
   config.subscribe_key = w.demo_keyset.subscribe_key.clone();
   config.secret_key = w.demo_keyset.secret_key.clone();

   let pubnub = w.get_pubnub(Some(config));

   let message = "hello there";

   // pubnub.publish(SamplePublishmessage{
   //    title: String::from("Sample message title"),
   //    text: String::from("Sample message text")
   // });

   println!("I publish a message");
   println!("{:#?}", pubnub);
}

#[then(regex = r"I receive successful response")]
async fn success_response(w: &mut World) {
   let pubnub = w.get_pubnub(None);

   println!("I receive successful response");
   println!("{:#?}", pubnub);
}
