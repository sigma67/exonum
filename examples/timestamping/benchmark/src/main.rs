// Copyright 2019 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
#![warn(unused_extern_crates)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate exonum_derive;

use exonum::{
    crypto::{gen_keypair, Hash},
    messages::{to_hex_string},
};

mod proto;
mod schema;
mod transactions;
mod api;

use transactions::{ TxTimestamp };
use schema::Timestamp;

fn main() {
    // Create testkit for network with four validators.
    // Create few transactions.
    let keypair = gen_keypair();
    let content = Timestamp::new(&Hash::zero(), "metadata");
    let tx = TxTimestamp::sign(&keypair.0, content, &keypair.1);
    let data = to_hex_string(&tx);
    let url = String::from("localhost");
    api::post(&url, &data);

//    let tx_info: TransactionResponse = api
//        .public(ApiKind::Explorer)
//        .query(&json!({ "tx_body": data }))
//        .post("v1/transactions")
//        .unwrap();

}
