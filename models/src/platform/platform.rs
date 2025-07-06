use serde::{Deserialize as SarDeserialize, Serialize as SarSerialize};
use rkyv::{
    Archive,
    Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
};
use crate::{Patch, EntityId};

#[derive(Archive, RkyvDeserialize, RkyvSerialize, SarSerialize, SarDeserialize, Debug, Clone, Default, PartialEq)]
pub struct Platform {
  pub tenant_id: EntityId,
  pub community_name: String,
  pub community_description: String,
  pub platform_url: String,
}
#[derive(Archive, RkyvDeserialize, RkyvSerialize, SarSerialize, SarDeserialize, Debug, Clone, Default, PartialEq)]
pub struct PlatformPatch {
  pub tenant_id: EntityId,
  pub community_name: Option<String>,
  pub community_description: Option<String>,
  pub platform_url: Option<String>
}

impl Patch<Platform> for PlatformPatch {
  fn apply_to(self, target: &mut Platform) -> () {
    if let Some(community_name) = self.community_name {
      target.community_name = community_name;
    }
    if let Some(community_description) = self.community_description {
      target.community_description = community_description;
    }
    if let Some(platform_url) = self.platform_url {
      target.platform_url = platform_url;
    }
  }
}