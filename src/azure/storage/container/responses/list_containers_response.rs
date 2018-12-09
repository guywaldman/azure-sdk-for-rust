use crate::azure::core::incompletevector::IncompleteVector;
use crate::azure::core::RequestId;
use crate::azure::storage::container::Container;

#[derive(Debug, Clone)]
pub struct ListContainersResponse {
    pub incomplete_vector: IncompleteVector<Container>,
    pub request_id: RequestId,
}

impl ListContainersResponse {
    pub fn is_complete(&self) -> bool {
        self.incomplete_vector.is_complete()
    }
}
