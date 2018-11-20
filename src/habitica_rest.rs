
use hyper::rt::{Future, Stream};
use hyper::Client;

//use htask::HTask;

pub fn fetch_json(url: hyper::Uri) -> impl Future<Item=Vec<User>, Error=FetchError> {
    let client = Client::new();

    client
        // Fetch the url...
        .get(url)
        // And then, if we get a response back...
        .and_then(|res| {
            // asynchronously concatenate chunks of the body
            res.into_body().concat2()
        })
        .from_err::<FetchError>()
        // use the body after concatenation
        .and_then(|body| {
            // try to parse as json with serde_json
            let users = serde_json::from_slice(&body)?;

            Ok(users)
        })
        .from_err()
}

//pub fn add_task(url: hyper::Uri, task: Htask)

#[derive(Deserialize, Debug)]
pub struct User {
    id: i32,
    name: String,
}

impl User {
    pub fn get_id(&self) -> &i32{
        return &self.id;
    }
}

// Define a type so we can return multiple types of errors
pub enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}