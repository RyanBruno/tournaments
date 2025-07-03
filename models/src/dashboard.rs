use serde::{Deserialize, Serialize};
use rkyv::{
    Archive,
    Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
};
use crate::Event;

#[derive(Archive, Serialize, Deserialize, Clone, RkyvDeserialize, RkyvSerialize, PartialEq)]
pub struct DashboardView {
  pub announcement: String,
  pub name: String,
  pub events: Vec<Event>,
}