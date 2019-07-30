use crate::{Tiddly, get_env_var_or_default};

use bson;
use bson::{Bson, Document};

use mongodb::{Client, ThreadedClient};
use mongodb::db::{Database,ThreadedDatabase};
use mongodb::coll::Collection;

use serde::Serialize;
use bson::ordered::OrderedDocument;

const DB_NAME: &'static str = "wiki_mongo";
const COLLECTION_NAME: &'static str = "tiddlers";

pub trait Repo<T> {
    fn save(self, db: &Database) -> T;
    fn find_one(id: String, db: &Database) -> Option<T>;
    fn delete(id: String, db: &Database);
}

impl Repo<Tiddly> for Tiddly {
    fn save(self, db: &Database) -> Tiddly {
        let coll: Collection = db.collection(COLLECTION_NAME);
        let mut doc = to_document(&self).expect("Unable to convert to document");

        // Use name as Id
        doc.insert("_id", self.name.to_owned());
        coll.insert_one(doc, None).expect("Error on insert");

        // FIXME find how to update only when isn't insert
        update(&self, db);

        return self;
    }

    fn find_one(name: String, db: &Database) -> Option<Tiddly> {
        let coll: Collection = db.collection(COLLECTION_NAME);
        let filter = doc! {"_id":name};

        let option: Option<Document> = coll.find_one(Some(filter), None).expect("Error on find");
        if let Some(document) = option {
            return Some(from_document(document));
        }
        return None;
    }

    fn delete(name: String, db: &Database) {
        let coll: Collection = db.collection(COLLECTION_NAME);
        let filter = doc! {"_id":name};
        coll.delete_one(filter, None).expect("Error on delete");
    }
}

fn update(tiddly: &Tiddly, db:&Database){
    let coll: Collection = db.collection(COLLECTION_NAME);
    let filter: Document = doc! {"_id": tiddly.name.to_owned()};
    let mut update: Document = doc! {"name":tiddly.name.to_owned()};

    if let Some(content) = tiddly.body.to_owned() {
        update.insert("content", content);
    };
    coll.update_one(filter, doc! {"$set":update}, None).expect("Error on update");
}

// FIXME there must be an automatic way to do it (doc to struct)
fn from_document(doc: OrderedDocument) -> Tiddly {
    // FIXME use Tiddly::new here (make it work)
    return Tiddly {
        name: doc.get_str("name").expect("name is required").to_string(),
        body: match doc.get_str("content") {
            Ok(value) => Some(value.to_string()),
            _ => None
        }
        // FIXME convert tags
    }
}

fn to_document<T: ?Sized>(obj:&T) -> Result<Document,Bson>
    where T: Serialize
{
    let model_bson:Bson= bson::to_bson(obj).expect("unable to convert to bson");
    return match model_bson {
        bson::Bson::Document(model_doc) => Ok(model_doc),
        other => Err(other)
    }
}

pub fn connect() -> Database {
    let mongo_uri = get_env_var_or_default("MONGODB_URI","mongodb://localhost:27017");
    let client = Client::with_uri(mongo_uri.as_str()).expect("Failed to initialize mongo client.");
    return client.db(DB_NAME);
}
