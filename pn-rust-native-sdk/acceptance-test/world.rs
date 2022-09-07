use pn_rust_native_sdk::*;
use cucumber::{WorldInit};
use std::{convert::Infallible};
use async_trait::async_trait;

#[derive(Debug)]
pub struct Settings {
    pub check_contract_expectations: bool,
    pub contract_server: String,
}

#[derive(Debug, WorldInit)]
pub struct World {
    pub settings: Settings,
    pub default_config: pubnub::PubnubConfig,
    pub demo_keyset: pubnub::PubnubKeyset,
    pub pubnub: Option<pubnub::Pubnub>,
    pub keyset: Option<pubnub::PubnubKeyset>,
}

impl World {
    pub fn get_pubnub(&mut self, config: Option<pubnub::PubnubConfig>) -> &Option<pubnub::Pubnub> {

        if let Some(current_config) = config {
            println!("{:#?}", current_config);

            self.stop_pubnub();
            self.pubnub = Some(pubnub::Pubnub::new(current_config));
        }

        return &self.pubnub;
    }

    pub fn stop_pubnub(&mut self) {
        if let Some(pubnub_instance) = &self.pubnub {
            pubnub_instance.stop();
        }

        self.pubnub = None;
    }
}

#[async_trait(?Send)]
impl cucumber::World for World {
    type Error = Infallible;

    async fn new() -> Result<Self, Self::Error> {
        Ok(Self {
            settings: Settings{
                check_contract_expectations: true,
                contract_server: String::from("localhost:8090"),
            },
            default_config: pubnub::PubnubConfig {
                publish_key: String::from(""),
                subscribe_key: String::from(""),
                secret_key: String::from(""),
                origin: String::from("localhost:8090"),
                ssl: false,
                suppress_leave_events: true,
                log_verbosity: false,
            },
            demo_keyset: pubnub::PubnubKeyset {
                publish_key: String::from("demo"),
                subscribe_key: String::from("demo"),
                secret_key: String::from(""),
            },
            pubnub: Option::None,
            keyset: Option::None,
        }) 
    }
}
