use crate::azure::core::errors::AzureError;
use crate::azure::core::incompletevector::IncompleteVector;
use crate::azure::core::{date_from_headers, request_id_from_headers, RequestId};
use crate::azure::storage::blob::{incomplete_vector_from_response, Blob};
use chrono::{DateTime, Utc};
use http::HeaderMap;

#[derive(Debug, Clone, PartialEq)]
pub struct ListBlobsResponse {
    pub incomplete_vector: IncompleteVector<Blob>,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
}

impl ListBlobsResponse {
    pub(crate) fn from_response(container_name: &str, headers: &HeaderMap, body: &str) -> Result<ListBlobsResponse, AzureError> {
        let incomplete_vector = incomplete_vector_from_response(body, container_name)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;

        Ok(ListBlobsResponse {
            incomplete_vector,
            request_id,
            date,
        })
    }
}
