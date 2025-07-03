use serde::{Deserialize, Serialize};
use rkyv::{
    Archive,
    Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
};
use crate::Patch;

#[derive(Archive, RkyvDeserialize, RkyvSerialize, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Event {
  pub tenant_id: String,
  pub id: String,
  pub name: String,
  pub location: String,
  pub date: String,
  pub image: String,
  pub banner: Option<String>,
  pub upsell: Option<String>,
}

#[derive(Archive, RkyvDeserialize, RkyvSerialize, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct EventPatch {
  pub id: Option<String>,
  pub name: Option<String>,
  pub location: Option<String>,
  pub date: Option<String>,
  pub image: Option<String>,
  pub banner: Option<Option<String>>,
  pub upsell: Option<Option<String>>,
}

impl Patch<Event> for EventPatch {
  fn apply_to(self, target: &mut Event) {
    if let Some(id) = self.id {
      target.id = id;
    }
    if let Some(name) = self.name {
      target.name = name;
    }
    if let Some(location) = self.location {
      target.location = location;
    }
    if let Some(date) = self.date {
      target.date = date;
    }
    if let Some(image) = self.image {
      target.image = image;
    }
    if let Some(banner) = self.banner {
      target.banner = banner;
    }
    if let Some(upsell) = self.upsell {
      target.upsell = upsell;
    }
  }
}