mod tables;

use tables::{IdentifierId, SledEventTree, SledEventTreeVec};
use std::path::Path;
use chrono::{DateTime, Local};
use sled::Db;
use crate::{
    error::Error,
    derivation::attached_signature_code::get_sig_count,
    event::Event,
    prefix::{
        AttachedSignaturePrefix, BasicPrefix, IdentifierPrefix, Prefix, SelfAddressingPrefix,
        SelfSigningPrefix,
    },
};
use super::EventDatabase;

pub struct SledEventDatabase {
    // "iids" tree
    // this thing is expensive, but everything else is cheeeeeep
    identifiers: SledEventTree<IdentifierId>,
    // "evts" tree
    events: SledEventTreeVec<Event>,
    // "dtss" tree
    datetime_stamps: SledEventTreeVec<DateTime<Local>>,
    // "sigs" tree
    signatures: SledEventTreeVec<AttachedSignaturePrefix>,
    // "rcts" tree
    receipts_nt: SledEventTreeVec<???>,
    // "ures" tree
    escrowed_receipts_nt: SledEventTreeVec<>,
    // "vrcs" tree
    receipts_t: SledEventTreeVec<>,
    // "vres" tree
    escrowed_receipts_t: SledEventTreeVec<>,
    // "kels" tree
    key_event_logs: SledEventTreeVec<SelfAddressingPrefix>,
    // "pses" tree
    partially_signed_events: SledEventTreeVec<???>,
    // "ooes" tree
    out_of_order_events: SledEventTreeVec<>,
    // "ldes" tree
    likely_duplicious_events: SledEventTreeVec<>,
    // "dels" tree
    diplicitous_events: SledEventTreeVec<>,
}


impl SledEventDatabase {
    pub fn new<'a, P>(path: P) 
        -> Result<Self, Error> 
    where P: Into<&'a Path> {
        let db = sled::open(path.into())?; 
        Ok(Self {
            identifiers: SledEventTree::new(db.open_tree(b"iids")?),
            events: SledEventTreeVec::new(db.open_tree(b"evts")?),
            datetime_stamps: SledEventTreeVec::new(db.open_tree(b"dtss")?),
            signatures: SledEventTreeVec::new(db.open_tree(b"sigs")?),
            receipts_nt: SledEventTreeVec::new(db.open_tree(b"rcts")?),
            escrowed_receipts_nt: SledEventTreeVec::new(db.open_tree(b"ures")?),
            receipts_t: SledEventTreeVec::new(db.open_tree(b"vrcs")?),
            escrowed_receipts_t: SledEventTreeVec::new(db.open_tree(b"vres")?),
            key_event_logs: SledEventTreeVec::new(db.open_tree(b"kels")?),
            partially_signed_events: SledEventTreeVec::new(db.open_tree(b"pses")?),
            out_of_order_events: SledEventTreeVec::new(db.open_tree(b"ooes")?),
            likely_duplicious_events: SledEventTreeVec::new(db.open_tree(b"ldes")?),
            diplicitous_events: SledEventTreeVec::new(db.open_tree(b"dels")?)
        })
    }

    fn get_identifier_id(&self, prefix: &IdentifierPrefix) -> Result<u64, Error> {
        match self.identifiers.get(prefix.to_str().as_bytes())? {
            Some(id) => Ok(serde_cbor::from_slice(&id)?),
            None => Err(Error::NotIndexedError)
        }
    }

    fn set_idendifier_id(&self, prefix: &IdentifierPrefix) -> Result<(), Error> {
        let key = prefix.to_str().as_bytes();
        let tree = self.db.open_tree(b"iids")?;
        if tree.contains_key(key)? { return Err(Error::IdentifierPresentError); }

        let next_id = match tree.last()? {
            Some((max, _)) => {
                let c_max: u64 = serde_cbor::from_slice(&max)?;
                c_max + 1u64
            },
            None => 0u64
        };

        match tree.insert(key, serde_cbor::to_vec(&next_id)?)? {
            Some(_) => Ok(()),
            None => Err(Error::IdentifierPresentError)
        }
    }
}



impl EventDatabase for SledEventDatabase {
    type Error = Error;

    fn last_event_at_sn(
        &self,
        pref: &IdentifierPrefix,
        sn: u64) 
            -> Result<Option<Vec<u8>>, Self::Error> {
        // open kels tree
        let kels = self.db.open_tree(b"kels")?;
        let id = self.get_identifier_id(pref)?;
        // get entry with `sn` key
        match kels.get(key_bytes(id))? { todo!();
            Some(value) => {
                let sap: SelfAddressingPrefix = serde_cbor::from_slice(&value)?;
                let dig_index = format!("{}.{}", pref.to_str(), &sap).as_bytes();
                let events = self.db.open_tree(b"evts")?;
                match events.get(&dig_index)? {
                    Some(event) => Ok(Some(serde_cbor::from_slice(&event)?)),
                    None => Ok(None)
                }
            },
            None => Ok(None)
        }
    }

    fn get_kerl(&self, id: &IdentifierPrefix) -> Result<Option<Vec<u8>>, Self::Error> {
        todo!()
    }

    fn log_event(
        &self,
        prefix: &IdentifierPrefix,
        dig: &SelfAddressingPrefix,
        raw: &[u8],
        sigs: &[AttachedSignaturePrefix],
    ) -> Result<(), Self::Error> {
        let key = key_bytes(self.get_identifier_id(prefix)?);
        let dts = self.db.open_tree(b"dtss")?;
        dts.insert(key, Local::now().to_rfc3339().as_bytes())?;
        let sdb = self.db.open_tree(b"sigs")?;
        sigs.iter().map(|sig| sdb.insert(key, sig))
    }

    fn finalise_event(
        &self,
        prefix: &IdentifierPrefix,
        sn: u64,
        dig: &SelfAddressingPrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn escrow_partially_signed_event(
        &self,
        pref: &IdentifierPrefix,
        sn: u64,
        dig: &SelfAddressingPrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn escrow_out_of_order_event(
        &self,
        pref: &IdentifierPrefix,
        sn: u64,
        dig: &SelfAddressingPrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn likely_duplicitous_event(
        &self,
        pref: &IdentifierPrefix,
        sn: u64,
        dig: &SelfAddressingPrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn duplicitous_event(
        &self,
        pref: &IdentifierPrefix,
        sn: u64,
        dig: &SelfAddressingPrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn add_nt_receipt_for_event(
        &self,
        pref: &IdentifierPrefix,
        dig: &SelfAddressingPrefix,
        signer: &BasicPrefix,
        sig: &SelfSigningPrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn add_t_receipt_for_event(
        &self,
        pref: &IdentifierPrefix,
        dig: &SelfAddressingPrefix,
        signer: &IdentifierPrefix,
        sig: &AttachedSignaturePrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn escrow_nt_receipt(
        &self,
        pref: &IdentifierPrefix,
        dig: &SelfAddressingPrefix,
        signer: &BasicPrefix,
        sig: &SelfSigningPrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn escrow_t_receipt(
        &self,
        pref: &IdentifierPrefix,
        dig: &SelfAddressingPrefix,
        signer: &IdentifierPrefix,
        sig: &AttachedSignaturePrefix,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn has_receipt(
        &self,
        pref: &IdentifierPrefix,
        sn: u64,
        validator: &IdentifierPrefix,
    ) -> Result<bool, Self::Error> {
        todo!()
    }
}