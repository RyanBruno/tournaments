use std::error::Error;
use crate::{
  Command,
  Patch,
  KVStore,
  Event,
  EventPatch,
  EntityId
};
use rkyv::{
    Archive, Deserialize, Serialize,
};

#[derive(Archive, Serialize, Deserialize, Clone)]
pub struct DashboardView {
  pub announcement: String,
  pub name: String,
  pub events: Vec<Event>,
}

#[derive(Default, Clone, Archive, Serialize, Deserialize)]
pub enum DashboardCommand {
  #[default]
  Noop,
  /* Events */
  CreateEvent(Event),
  UpdateEvent((String, EntityId, EventPatch)),

  /* Users */
  /* Announcment */
}

#[derive(Default, Clone, Archive, Serialize, Deserialize)]
pub enum DashboardModel {
  #[default]
  Noop,
  DashboardView(DashboardView),
  Event(Event),
}

impl Patch<DashboardModel> for DashboardCommand {
  fn apply_to(self, target: &mut DashboardModel) -> () {
    match (self, target) {
      (DashboardCommand::CreateEvent(event), DashboardModel::DashboardView(view)) => {
        // Add Event to dashboard view
        view.events.push(event.clone());
      },
      (DashboardCommand::UpdateEvent((_tenant_id, _id, patch)), DashboardModel::Event(event)) => {
        // Apply patch to event
        patch.apply_to(event);
      },
      (DashboardCommand::UpdateEvent((_tenant_id, id, patch)), DashboardModel::DashboardView(view)) => {
        // Apply patch to event
        patch.apply_to(view.events.iter_mut().find(|e| e.id == id).unwrap());
      },
      _ => todo!(),
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
      _ => todo!(),
    }
    Ok(())
  }
}