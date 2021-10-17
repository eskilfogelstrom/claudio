use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::str;


use hyper::Client;
use hyper::{Body, Method, Request, Uri};
use hyper::body::HttpBody;
use tokio::io::{stdout, AsyncWriteExt as _};
use anyhow::Result;

use crate::util::Mode;

pub async fn run(mode: &Mode, address: &str) -> Result<String> {

    let remote_addr = match mode {
        Mode::Send => {
            let client = Client::new();

            let req = Request::builder()
                .method(Method::POST)
                .uri("http://localhost:3000/connections/")
                .header("content-type", "application/json")
                .body(Body::from(
                    format!(r#"{{
                    "id": "test",
                    "remoteId": "test2",
                    "address": "{}"
                    }}"#, address)
                ))?; 
            
            let mut resp = client.request(req).await?;

            let mut addr = String::from("");

            while let Some(chunk) = resp.body_mut().data().await {
                addr.push_str(str::from_utf8(&chunk?)?);
            }

            addr
        },
        Mode::Return => {
            let client = Client::new();

            let req = Request::builder()
                .method(Method::POST)
                .uri("http://localhost:3000/connections/test2/ack/")
                .header("content-type", "application/json")
                .body(Body::from(
                    format!(r#"{{
                    "remoteAddress": "{}"
                    }}"#, address)
                ))?; 
            
            let mut resp = client.request(req).await?;

            let mut addr = String::from("");

            while let Some(chunk) = resp.body_mut().data().await {
                addr.push_str(str::from_utf8(&chunk?)?);
            }

            addr
        }
    };

    println!("{}", remote_addr);

    Ok(remote_addr)
}