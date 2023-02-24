/*
List blob containers in a storage account
cargo run --package azure_svc_blobstorage --example list_containers $STORAGE_ACCOUNT_NAME

This is similar to `az storage container list --account-name $STORAGE_ACCOUNT_NAME`
https://docs.microsoft.com/cli/azure/storage/container?view=azure-cli-latest#az-storage-container-list
*/

use azure_svc_communicationidentity

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let acs_connection_string = std::env::args().nth(1).expect("please specify acs connection string");

    let client = azure_svc_blobstorage::Client::builder(credential)
        .endpoint(endpoint)
        .scopes(scopes)
        .build();

    let mut count = 0;
    let mut pages = client.service_client().list_containers_segment().into_stream();
    while let Some(Ok(page)) = pages.next().await {
        if let Some(containers) = page.containers {
            count += containers.items.len();
            for container in containers.items {
                println!("{}", container.name);
            }
        }
    }
    println!("# of containers {}", count);

    Ok(())
}
