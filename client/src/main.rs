use tonic::{transport::Server, Request, Response, Status};

use bookstore::{GetBookRequest, bookstore_client::BookstoreClient};


mod bookstore {
    include!("bookstore.rs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BookstoreClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(GetBookRequest {
        id: "1".to_string(),
    });
    let response = client.get_book(request).await?;
    println!("Got: '{}' from service", response.into_inner().year);
    Ok(())
}