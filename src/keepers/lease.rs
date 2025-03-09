use std::borrow::Borrow;

#[derive(Clone)]
pub struct Lease {
    pub(crate) group_id: String,
    pub(crate) ttl: u64,
}

impl Lease {
    pub fn new(group_id: &str, ttl: u64) -> Self {
        Lease {
            group_id: group_id.into(),
            ttl,
        }
    }
}

impl PartialEq for Lease {
    fn eq(&self, other: &Self) -> bool {
        self.group_id == other.group_id
    }
}

impl PartialEq<str> for Lease {
    fn eq(&self, other: &str) -> bool {
        self.group_id == other
    }
}

impl Borrow<str> for Lease {
    fn borrow(&self) -> &str {
        &self.group_id
    }
}

// Required to use Lease in a HashMap
impl Eq for Lease {}

impl std::hash::Hash for Lease {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.group_id.hash(state);
    }
}
