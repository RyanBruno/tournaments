use reqwest::Client;

#[derive(Clone)]
pub struct ClientContext {
  pub client: Client,
}
