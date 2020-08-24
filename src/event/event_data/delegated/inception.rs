use super::super::super::sections::{DelegatingLocationSeal, InceptionWitnessConfig, KeyConfig};
use crate::error::Error;
use crate::state::signatory::Signatory;
use crate::state::EventSemantics;
use crate::state::IdentifierState;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DelegatedInceptionEvent {
    #[serde(flatten)]
    pub key_config: KeyConfig,

    #[serde(flatten)]
    pub witness_config: InceptionWitnessConfig,

    #[serde(rename = "seal")]
    pub delegation_location: DelegatingLocationSeal,
}

impl EventSemantics for DelegatedInceptionEvent {
    fn apply_to(&self, state: IdentifierState) -> Result<IdentifierState, Error> {
        Ok(IdentifierState {
            current: Signatory {
                threshold: self.key_config.threshold,
                signers: self.key_config.public_keys.clone(),
            },
            next: self.key_config.threshold_key_digest.clone(),
            witnesses: self.witness_config.initial_witnesses.clone(),
            tally: self.witness_config.tally,
            ..state
        })
    }
}
