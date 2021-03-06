mod expiration;
mod schema;

pub use expiration::*;
pub use schema::*;

use super::*;

pub trait SubtreePolicy {
    fn validate(
        &self, state: &State, policy_domain: &Domain, domain_after_op: &Domain,
    ) -> Result<()>;
}

impl<T: SubtreePolicy> SubtreePolicy for Option<T> {
    fn validate(
        &self, state: &State, policy_domain: &Domain, domain_after_op: &Domain,
    ) -> Result<()> {
        if let Some(p) = self {
            p.validate(state, policy_domain, domain_after_op)
        } else {
            Ok(())
        }
    }
}

impl SubtreePolicy for SubtreePolicies {
    fn validate(
        &self, state: &State, policy_domain: &Domain, domain_after_op: &Domain,
    ) -> Result<()> {
        self.expiration.validate(state, policy_domain, domain_after_op)?;
        self.schema.validate(state, policy_domain, domain_after_op)?;
        Ok(())
    }
}
