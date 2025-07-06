use std::error::Error;
use crate::{
  CQRSStore, Command, KVStore
};
use models::{
  Patch,
  Platform,
  PlatformPatch,
  PlatformUser,
  PlatformUserPatch,
};
use rkyv::{
    Archive, Deserialize, Serialize,
};

use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Ref;


#[derive(Default, Clone, Archive, Serialize, Deserialize)]
pub enum PlatformCommand {
  #[default]
  Noop,
  /* User */
  CreateUser(PlatformUser),
  UpdateUser((String, PlatformUserPatch)),
  /* User */
  CreatePlatform(Platform),
  UpdatePlatform(PlatformPatch),
}

#[derive(Default, Clone, Archive, Serialize, Deserialize)]
pub enum PlatformModel {
  #[default]
  Noop,
  User(PlatformUser),
  Platform(Platform),
}

impl Patch<PlatformModel> for PlatformCommand {
  fn apply_to(self, target: &mut PlatformModel) -> () {
    match (self, target) {
      (PlatformCommand::UpdatePlatform(patch), PlatformModel::Platform(platform)) => {
        // Apply patch to platform
        patch.apply_to(platform);
      },
      (PlatformCommand::UpdateUser((_email, patch)), PlatformModel::User(user)) => {
        // Apply patch to user
        patch.apply_to(user);
      },
      _ => {},
    }
  }
}

impl Command<PlatformModel, PlatformCommand> for PlatformCommand {
  fn fold(&self, kv_store: &mut KVStore<PlatformModel, PlatformCommand>) -> Result<(), Box<dyn Error>> {
    match self {
      PlatformCommand::CreateUser(user) => {
        /* Create User */
        kv_store.create(format!("user-{}", user.email),PlatformModel::User(user.clone()))?;
      },
      PlatformCommand::UpdateUser((email, _user)) => {
        /* Update User */
        kv_store.update(format!("user-{}", email), self.clone())?;
      },
      PlatformCommand::CreatePlatform(platform) => {
        /* Create Platform */
        kv_store.create(format!("platform-{}", platform.tenant_id.clone()), PlatformModel::Platform(platform.clone()))?;
      },
      PlatformCommand::UpdatePlatform(platform) => {
        /* Update Platform */
        kv_store.update(format!("platform-{}", platform.tenant_id.clone()), self.clone())?;
      },
      PlatformCommand::Noop => {},
    }
    Ok(())
  }
}

pub type PlatformStoreInner = CQRSStore<PlatformCommand, PlatformModel, PlatformCommand>;
#[derive(Clone)]
pub struct PlatformStore {
  inner: Rc<RefCell<PlatformStoreInner>>,
}

impl PlatformStore {
  pub fn new(store: PlatformStoreInner) -> Self {
    Self { inner: Rc::new(RefCell::new(store)) }
  }

  pub fn command(&mut self, command: &PlatformCommand) -> Result<(), Box<dyn Error>> {
    self.inner.borrow_mut().command(command)
  }

  pub fn borrow_inner(&self) -> Ref<PlatformStoreInner> {
    self.inner.borrow()
  }

  pub fn fold(&mut self) -> Result<(), Box<dyn Error>> {
    self.inner.borrow_mut().fold()
  }
}

pub fn platform_store() -> Result<PlatformStore, Box<dyn Error>> {
  Ok(PlatformStore::new(
  PlatformStoreInner::new(
    "data/platform/transactions/".into(),
    KVStore::new(
      "data/platform/snapshots/".into(),
      "data/platform/events/".into(),
      10,
    )?
  )))
}