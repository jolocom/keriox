use crate::prefix::BasicPrefix;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Signatory {
    pub threshold: usize,
    pub signers: Vec<BasicPrefix>,
}
