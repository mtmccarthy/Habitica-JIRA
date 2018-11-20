#![deny(warnings)]
extern crate hyper;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod habitica_rest;
mod htask;

use habitica_rest::{fetch_json, FetchError};

use hyper::rt::{self, Future};

fn main() {
    let url = "http://jsonplaceholder.typicode.com/users".parse().unwrap();

    let fut = fetch_json(url)
        // use the parsed vector
        .map(|users| {
            // print users
            println!("users: {:#?}", users);

            // print the sum of ids
            let sum = users.iter().fold(0, |acc, user| acc + user.get_id());
            println!("sum of ids: {}", sum);
        })
        // if there was an error print it
        .map_err(|e| {
            match e {
                FetchError::Http(e) => eprintln!("http error: {}", e),
                FetchError::Json(e) => eprintln!("json parsing error: {}", e),
            }
        });

    // Run the runtime with the future trying to fetch, parse and print json.
    //
    // Note that in more complicated use cases, the runtime should probably
    // run on its own, and futures should just be spawned into it.
    rt::run(fut);
}
