use std::error::Error;
use std::rc::Rc;
use std::cell::{RefCell, Ref};

use crate::{CQRSStore, Command, KVStore};
use models::{Bracket, ScoreUpdate, Patch};
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Default, Clone, Archive, Serialize, Deserialize)]
pub enum TournamentCommand {
    #[default]
    Noop,
    CreateBracket(Bracket),
    UpdateScore((String, ScoreUpdate)), // bracket_id, update
}

#[derive(Default, Clone, Archive, Serialize, Deserialize)]
pub enum TournamentModel {
    #[default]
    Noop,
    Bracket(Bracket),
}

impl Patch<TournamentModel> for TournamentCommand {
    fn apply_to(self, target: &mut TournamentModel) {
        match self {
            TournamentCommand::CreateBracket(bracket) => {
                *target = TournamentModel::Bracket(bracket);
            }
            TournamentCommand::UpdateScore((bracket_id, update)) => {
                if let TournamentModel::Bracket(bracket) = target {
                    if bracket.id == bracket_id {
                        if let Some(m) = bracket.matches.iter_mut().find(|m| m.id == update.match_id) {
                            update.apply_to(m);
                        }
                    }
                }
            }
            TournamentCommand::Noop => {}
        }
    }
}

impl Command<TournamentModel, TournamentCommand> for TournamentCommand {
    fn fold(&self, kv_store: &mut KVStore<TournamentModel, TournamentCommand>) -> Result<(), Box<dyn Error>> {
        match self {
            TournamentCommand::CreateBracket(bracket) => {
                kv_store.create(bracket.id.clone(), TournamentModel::Bracket(bracket.clone()))?;
            }
            TournamentCommand::UpdateScore((id, _update)) => {
                kv_store.update(id.clone(), self.clone())?;
            }
            TournamentCommand::Noop => {}
        }
        Ok(())
    }
}

pub type TournamentStoreInner = CQRSStore<TournamentCommand, TournamentModel, TournamentCommand>;
#[derive(Clone)]
pub struct TournamentStore {
    inner: Rc<RefCell<TournamentStoreInner>>,
}

impl TournamentStore {
    pub fn new(store: TournamentStoreInner) -> Self {
        Self { inner: Rc::new(RefCell::new(store)) }
    }

    pub fn command(&mut self, command: &TournamentCommand) -> Result<(), Box<dyn Error>> {
        self.inner.borrow_mut().command(command)
    }

    pub fn borrow_inner(&self) -> Ref<TournamentStoreInner> {
        self.inner.borrow()
    }

    pub fn fold(&mut self) -> Result<(), Box<dyn Error>> {
        self.inner.borrow_mut().fold()
    }
}

pub fn tournament_store() -> Result<TournamentStore, Box<dyn Error>> {
    Ok(TournamentStore::new(
        TournamentStoreInner::new(
            "data/tournament/transactions/".into(),
            KVStore::new(
                "data/tournament/snapshots/".into(),
                "data/tournament/events/".into(),
                4,
            )?
        )
    ))
}
