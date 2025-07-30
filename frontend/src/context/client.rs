use reqwest::Client;

#[derive(Clone)]
pub struct ClientContext {
  pub client: Client,
  pub token: Option<String>,
}

impl ClientContext {
    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }
}
