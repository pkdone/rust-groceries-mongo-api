use futures::prelude::*;
use mongodb::{
    bson::doc,
    options::{FindOptions, UpdateOptions},
    {Client, Collection},
};
use serde::{Deserialize, Serialize};
use std::error::Error;


const DB_NAME: &str = "groceries";
const COLL_NAME: &str = "items";


#[derive(Debug, Clone)]
pub struct GroceryMgr {
    coll: Collection,
}


// Grocery item record
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GroceryItem {
    pub name: String,
    pub quantity: i32,
}



// Grocery item id
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GroceryId {
    pub name: String,
}


// Manages interaction with Grocery datbaase collection
//
impl GroceryMgr {
    // Create new instance of Grocery manager using provided MongoDB URL
    //
    pub async fn new(db_url: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let client = Client::with_uri_str(db_url).await?;
        let coll = client.database(DB_NAME).collection(COLL_NAME);
        Ok(GroceryMgr { coll })
    }


    // Query Groceries collection returning list of all grocery items & quantities
    //
    pub async fn db_find_groceries(&self) -> Result<Vec<GroceryItem>, Box<dyn Error>> {
        let mut results = vec![];
        let find_options = FindOptions::builder().sort(doc! {"name": 1}).build();
        let mut cursor = self.coll.find(doc! {}, find_options).await?;

        while let Some(record) = cursor.next().await {
            match record {
                Ok(doc) => results.push(bson::de::from_bson(bson::Bson::Document(doc.clone()))?),
                Err(e) => return Err(e.into()),
            };
        }

        Ok(results)
    }


    // Insert/update new/existing Grocery item record with adding new quantity
    //
    pub async fn db_upsert_groceries(&self, item: GroceryItem) -> Result<(), Box<dyn Error>> {
        let update_options = UpdateOptions::builder().upsert(true).build();
        self.coll
            .update_one(
                doc! {"name": item.name},
                doc! {"$inc": {"quantity": item.quantity}},
                update_options,
            )
            .await?;
        Ok(())
    }


    // Delete Grocey item record from Groceries collection which matches item name
    //
    pub async fn db_delete_groceries(&self, id: GroceryId) -> Result<(), Box<dyn Error>> {
        self.coll.delete_one(doc! {"name": id.name}, None).await?;
        Ok(())
    }
}
