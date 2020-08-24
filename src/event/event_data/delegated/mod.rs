use super::super::sections::DelegatedEventSeal;
use crate::error::Error;
use crate::state::IdentifierState;
pub mod inception;
pub mod rotation;

pub fn incept_delegated(seal: &DelegatedEventSeal) -> IdentifierState {
    IdentifierState {
        prefix: seal.prefix,
        sn: 0,
        last: seal.event_digest,
        ..IdentifierState::default()
    }
}
