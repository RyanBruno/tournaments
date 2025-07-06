use std::ops::Add;

use serde::{Deserialize as SarDeserialize, Serialize as SarSerialize};
use rkyv::{
    Archive,
    Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
};

use crate::Event;

#[derive(Archive, RkyvDeserialize, RkyvSerialize, SarSerialize, SarDeserialize, Debug, Clone, Default, PartialEq)]
pub struct DashboardData {
  pub announcement: String,
  pub name: String,
  pub events: Vec<Event>,
}

impl Add<Vec<Event>> for DashboardData {
  type Output = DashboardView;
  fn add(self, rhs: Vec<Event>) -> DashboardView {
    DashboardView { announcement: self.announcement, name: self.name, events: self.events, active_events: rhs }
  }
}
#[derive(Archive, RkyvDeserialize, RkyvSerialize, SarSerialize, SarDeserialize, Debug, Clone, Default, PartialEq)]
pub struct DashboardView {
  pub announcement: String,
  pub name: String,
  pub events: Vec<Event>,
  pub active_events: Vec<Event>,
}