
#[derive(Debug, Clone)]
pub struct PubnubConfig {
  pub publish_key: String,
  pub subscribe_key: String,
  pub secret_key: String,
  pub origin: String,
  pub ssl: bool,
  pub suppress_leave_events: bool,
  pub log_verbosity: bool
}


#[derive(Debug, Clone)]
pub struct PubnubKeyset {
  pub publish_key: String,
  pub subscribe_key: String,
  pub secret_key: String
}

#[derive(Debug)]
pub struct Pubnub {
  config: PubnubConfig
  
}

impl Pubnub {

  pub fn new(config: PubnubConfig) -> Self {
    return Self { config }
  }

  pub fn stop(&self) {
    // stop the subscription
  }

  pub fn init() -> i32 {
    return 4;
  }

  pub fn get_config(&self) -> PubnubConfig {
    return self.config.clone();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
      let result = Pubnub::init();
      assert_eq!(result, 4);
  }
}
