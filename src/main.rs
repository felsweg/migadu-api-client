// Copyright 2022 felsweg-iota
// SPDX-License-Identifier: Apache-2.0
use migadu::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app: Migadu = Migadu::parse();

    // print result
    println!("{}", app.run(crate::api_endpoint()).await?.text().await?);

    Ok(())
}
