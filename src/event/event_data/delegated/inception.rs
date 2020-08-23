use crate::state::EventSemantics;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DelegatedInceptionEvent {}

impl EventSemantics for DelegatedInceptionEvent {}
