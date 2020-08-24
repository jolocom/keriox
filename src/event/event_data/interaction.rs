use super::super::sections::DelegatedEventSeal;
use crate::error::Error;
use crate::state::IdentifierState;
use crate::{prefix::SelfAddressingPrefix, state::EventSemantics};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractionEvent {
    #[serde(rename = "dig")]
    pub previous_event_hash: SelfAddressingPrefix,

    pub data: Vec<DelegatedEventSeal>,
}

impl EventSemantics for InteractionEvent {
    fn apply_to(&self, state: IdentifierState) -> Result<IdentifierState, Error> {
        Ok(IdentifierState {
            open_seals: [state.open_seals, self.data].concat(),
            ..state
        })
    }
}
