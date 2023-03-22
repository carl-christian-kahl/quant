use tonic::{transport::Server, Request, Response, Status};

use bookstore::{GetBookRequest, bookstore_client::BookstoreClient};

use std::time::{Duration, Instant};

mod bookstore {
    include!("bookstore.rs");
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BookstoreClient::connect("http://[::1]:50051").await?;
    let vector: Vec<i32> = vec![15];

    let request = tonic::Request::new(GetBookRequest {
        id: vector,
    });
    let start = Instant::now();
    let response = client.get_book(request).await?;
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);   
    println!("Got: '{:?}' from service", response.into_inner().id);
    Ok(())
}