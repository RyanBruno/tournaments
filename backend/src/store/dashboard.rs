use std::error::Error;
use crate::{
  CQRSStore, Command, KVStore
};
use models::{
  Event,
  EventPatch,
  Patch,
  EntityId,
  DashboardData,
  DashboardUser, DashboardUserPatch
};
use rkyv::{
    Archive, Deserialize, Serialize,
};

use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Ref;

#[derive(Default, Clone, Archive, Serialize, Deserialize)]
pub enum DashboardCommand {
  #[default]
  Noop,
  /* Events */
  CreateEvent(Event),
  UpdateEvent((String, EntityId, EventPatch)),
  StartEvent((String, EntityId)),
  EndEvent((String, EntityId)),

  /* User */
  CreateUser(DashboardUser),
  UpdateUser((String, DashboardUserPatch)),
  /* Dashboard Info */
  SetAnnouncement((EntityId, String)),
  SetName((EntityId, String)),
}

#[derive(Default, Clone, Archive, Serialize, Deserialize)]
pub enum DashboardModel {
  #[default]
  Noop,
  DashboardData(DashboardData),
  Event(Event),
  User(DashboardUser),
}

impl Patch<DashboardModel> for DashboardCommand {
  fn apply_to(self, target: &mut DashboardModel) -> () {
    match (self, target) {
      (DashboardCommand::CreateEvent(event), DashboardModel::DashboardData(view)) => {
        // Add Event to dashboard view
        view.events.retain(|e| e.id != event.id);
        view.events.push(event.clone());
      },
      (DashboardCommand::CreateEvent(event), target) => {
        // Create a new dashboard view with the event
        *target = DashboardModel::DashboardData(DashboardData {
          announcement: String::new(),
          name: String::new(),
          events: vec![event.clone()],
        });
      },
      (DashboardCommand::UpdateEvent((_tenant_id, _id, patch)), DashboardModel::Event(event)) => {
        // Apply patch to event
        patch.apply_to(event);
      },
      (DashboardCommand::UpdateEvent((_tenant_id, id, patch)), DashboardModel::DashboardData(view)) => {
        // Apply patch to event
        patch.apply_to(view.events.iter_mut().find(|e| e.id == id).unwrap());
      },
      (DashboardCommand::StartEvent((_, id)), DashboardModel::Event(event)) => {
        if event.id == id { event.active = true; }
      },
      (DashboardCommand::EndEvent((_, id)), DashboardModel::Event(event)) => {
        if event.id == id { event.active = false; }
      },
      (DashboardCommand::StartEvent((_, id)), DashboardModel::DashboardData(view)) => {
        if let Some(e) = view.events.iter_mut().find(|e| e.id == id) {
          e.active = true;
        }
      },
      (DashboardCommand::EndEvent((_, id)), DashboardModel::DashboardData(view)) => {
        if let Some(e) = view.events.iter_mut().find(|e| e.id == id) {
          e.active = false;
        }
      },
      (DashboardCommand::SetAnnouncement((_tenant_id, announcement)), DashboardModel::DashboardData(view)) => {
        // Set announcement in dashboard view
        view.announcement = announcement;
      },
      (DashboardCommand::SetName((_tenant_id, name)), DashboardModel::DashboardData(view)) => {
        // Set name in dashboard view
        view.name = name;
      },
      _ => {},
    }
  }
}

impl Command<DashboardModel, DashboardCommand> for DashboardCommand {
  fn fold(&self, kv_store: &mut KVStore<DashboardModel, DashboardCommand>) -> Result<(), Box<dyn Error>> {
    match self {
      DashboardCommand::CreateEvent(event) => {
        /* Create Event */
        kv_store.create(event.id.clone(), DashboardModel::Event(event.clone()))?;
        /* Update Tenant's Dashboard */
        kv_store.update(event.tenant_id.clone(), self.clone())?;
      },
      DashboardCommand::UpdateEvent((tenant_id, id, _patch)) => {
        /* Update Event */
        kv_store.update(id.clone(), self.clone())?;
        /* Update Tenant's Dashboard */
        kv_store.update(tenant_id.clone(), self.clone())?;
      },
      DashboardCommand::StartEvent((tenant_id, id)) | DashboardCommand::EndEvent((tenant_id, id)) => {
        kv_store.update(id.clone(), self.clone())?;
        kv_store.update(tenant_id.clone(), self.clone())?;
      },
      DashboardCommand::SetAnnouncement((tenant_id, _announcement)) => {
        kv_store.update(tenant_id.clone(), self.clone())?;
      },
      DashboardCommand::SetName((tenant_id, _name)) => {
        kv_store.update(tenant_id.clone(), self.clone())?;
      },
      DashboardCommand::CreateUser(user) => {
        /* Create User */
        kv_store.create(format!("user-{}", user.email),DashboardModel::User(user.clone()))?;
      },
      DashboardCommand::UpdateUser((email, _user)) => {
        /* Update User */
        kv_store.update(format!("user-{}", email), self.clone())?;
      },
      DashboardCommand::Noop => {},
    }
    Ok(())
  }
}

pub type DashboardStoreInner = CQRSStore<DashboardCommand, DashboardModel, DashboardCommand>;
#[derive(Clone)]
pub struct DashboardStore {
  inner: Rc<RefCell<DashboardStoreInner>>,
}

impl DashboardStore {
  pub fn new(store: DashboardStoreInner) -> Self {
    Self { inner: Rc::new(RefCell::new(store)) }
  }

  pub fn command(&mut self, command: &DashboardCommand) -> Result<(), Box<dyn Error>> {
    self.inner.borrow_mut().command(command)
  }

  pub fn borrow_inner(&self) -> Ref<DashboardStoreInner> {
    self.inner.borrow()
  }

  pub fn fold(&mut self) -> Result<(), Box<dyn Error>> {
    self.inner.borrow_mut().fold()
  }
}

pub fn dashboard_store() -> Result<DashboardStore, Box<dyn Error>> {
  Ok(DashboardStore::new(
  DashboardStoreInner::new(
    "data/dashboard/transactions/".into(),
    KVStore::new(
      "data/dashboard/snapshots/".into(),
      "data/dashboard/events/".into(),
      10,
    )?
  )))
}