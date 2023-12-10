#[allow(unused_imports)]
use futures::future::ok;
#[allow(unused_imports)]
use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

pub struct DBDATA {
    docs: mongodb::bson::Document
}

impl DBDATA {
    fn new() -> Self {
        Self {
            docs: doc! { "username":"pretham"},
        }
    }
}

#[allow(unused_variables)]
#[tokio::main]
pub async fn dbconnect() -> mongodb::error::Result<()> {
    let uri = "mongodb+srv://@cluster0.2rcpjkw.mongodb.net/?retryWrites=true&w=majority";
    let client = Client::with_uri_str(uri).await?;
    println!("connection established");
    let db = client.database("Secure-cypher");
    let coll: Collection<Document> = db.collection("usersavedpasswords");
    // let docs = doc! {"title":"1999"};
    coll.insert_one(DBDATA::new().docs, None).await?;
    println!("data inserted");
    Ok(())
}
