#![cfg(all(test, feature = "test_e2e"))]
#[macro_use]
extern crate log;

use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::container::PublicAccess;
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::*;
use std::collections::HashMap;

#[tokio::test]
async fn put_append_blob() {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let client = client::with_access_key(&account, &master_key);

    let blob_name: &'static str = "append_blob.txt";
    let container_name: &'static str = "rust-upload-test";
    let data = b"abcdef";

    if client
        .list_containers()
        .finalize()
        .await
        .unwrap()
        .incomplete_vector
        .iter()
        .find(|x| x.name == container_name)
        .is_none()
    {
        client
            .create_container()
            .with_container_name(container_name)
            .with_public_access(PublicAccess::Blob)
            .finalize()
            .await
            .unwrap();
    }

    let mut metadata = HashMap::new();
    metadata.insert("attrib", "value");
    metadata.insert("second", "something");

    // calculate md5 too!
    let _digest = md5::compute(&data[..]);

    client
        .put_append_blob()
        .with_container_name(&container_name)
        .with_blob_name(&blob_name)
        .with_content_type("text/plain")
        .with_metadata(&metadata)
        .finalize()
        .await
        .unwrap();

    trace!("created {:?}", blob_name);
}
