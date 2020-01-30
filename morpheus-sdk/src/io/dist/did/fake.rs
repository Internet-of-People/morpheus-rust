use std::collections::HashMap;

use async_trait::async_trait;
use failure::{err_msg, Fallible};

use super::*;
use crate::crypto::hash::ContentId;
use crate::data::{did::Did, diddoc::DidDocument};

pub struct FakeDidLedger {
    demo_docs: HashMap<Did, DidDocument>,
}

impl FakeDidLedger {
    pub fn new() -> Self {
        let did1 = "did:morpheus:ezbeWGSY2dqcUBqT8K7R14xr".parse().unwrap();
        let doc1 = DidDocument::implicit(&did1);
        let did2 = "did:morpheus:ez25N5WZ1Q6TQpgpyYgiu9gTX".parse().unwrap();
        let doc2 = DidDocument::implicit(&did2);
        let mut demo_docs = HashMap::default();
        demo_docs.insert(did1, doc1);
        demo_docs.insert(did2, doc2);
        Self { demo_docs }
    }
}

#[async_trait(?Send)]
impl LedgerQueries for FakeDidLedger {
    async fn before_proof(&self, _content: &ContentId) -> Fallible<Option<BlockHeight>> {
        todo!()
    }

    async fn document(&self, did: &Did) -> Fallible<DidDocument> {
        self.demo_docs.get(did).map(|doc| doc.to_owned()).ok_or_else(|| err_msg("not found"))
    }
}

#[async_trait(?Send)]
impl LedgerOperations for FakeDidLedger {
    async fn send_transaction(
        &self, _operations: &[OperationAttempt],
    ) -> Fallible<Box<dyn PooledLedgerTransaction>> {
        todo!()
    }
}
