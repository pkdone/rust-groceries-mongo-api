use std::env;
use std::error::Error;
use std::net::Ipv4Addr;
use std::process::exit;
use warp::{http, Filter};


mod data;
pub use crate::data::{GroceryId, GroceryItem, GroceryMgr};


const LISTEN_ADDRESS: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const LISTEN_PORT: u16 = 8080;
const RSC_NAME: &str = "groceries";
const PAYLOAD_LIMIT: u64 = 1024 * 16;


// Main app which starts Groceries REST API listener
//
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = get_url_arg_or_exit();
    let groceries = GroceryMgr::new(&url).await?;
    let groceries_ref = warp::any().map(move || groceries.clone());
    let api_path_filter_chain = warp::path("v1")
        .and(warp::path(RSC_NAME))
        .and(warp::path::end());
    let upsert_filter_chain = api_path_filter_chain
        .and(capture_grocery_json())
        .and(groceries_ref.clone())
        .and_then(upsert_grocery_list);
    // POST filter chain
    let add_items = warp::post()
        .and(upsert_filter_chain.clone());
    // PUT filter chain
    let update_item = warp::put()
        .and(upsert_filter_chain.clone());
    // GET filter chain
    let get_items = warp::get()
        .and(api_path_filter_chain)
        .and(groceries_ref.clone())
        .and_then(get_grocery_list);
    // DELETE filter chain
    let delete_item = warp::delete()
        .and(api_path_filter_chain)
        .and(capture_grocery_id_json())
        .and(groceries_ref.clone())
        .and_then(delete_grocery_list);
    let routes = add_items.or(get_items).or(delete_item).or(update_item);
    warp::serve(routes).run((LISTEN_ADDRESS, LISTEN_PORT)).await;
    Ok(())
}


// Extract the URL parameter passed on the command line or exit if not provided
//
fn get_url_arg_or_exit() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("\nERROR: A MongoDB URL needs to provided as an argument to this tool\n");
        exit(1);
    }

    args[1].to_string()
}


// Insert/update grocery item record in back-end DB
//
async fn upsert_grocery_list(item: GroceryItem, groceries: GroceryMgr)
                             -> Result<impl warp::Reply, warp::Rejection> {
    match groceries.db_upsert_groceries(item).await {
        Ok(_) => Ok(warp::reply::with_status("Added items to the grocery list",
                    http::StatusCode::CREATED)),
        Err(e) => {
            eprintln!("Error upserting data: {:#?}", e);
            Err(warp::reject())
        }
    }

}


// Find all grocery item records from back-end DB
//
async fn get_grocery_list(groceries: GroceryMgr)
                          -> Result<impl warp::Reply, warp::Rejection> {
    match groceries.db_find_groceries().await {
        Ok(result) => Ok(warp::reply::json(&result)),
        Err(e) => {
            eprintln!("Error deleting data: {}", e.to_string());
            Err(warp::reject())
        }
    }
}


// Delete specific grocery item record from back-end DB
//
async fn delete_grocery_list(id: GroceryId, groceries: GroceryMgr)
                             -> Result<impl warp::Reply, warp::Rejection> {
    match groceries.db_delete_groceries(id).await {
        Ok(_) => Ok(warp::reply::with_status("Removed item from grocery list",
                    http::StatusCode::OK)),
        Err(e) => {
            eprintln!("Error deleting data: {}", e.to_string());
            Err(warp::reject())
        }
    }
}


// Capture Grocery Item full record JSON request payload JSON content
//
fn capture_grocery_json() -> impl Filter<Extract = (GroceryItem,),
                                         Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(PAYLOAD_LIMIT).and(warp::body::json())
}


// Capture Grocery Item Id JSON request payload JSON content
//
fn capture_grocery_id_json() -> impl Filter<Extract = (GroceryId,),
                                            Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(PAYLOAD_LIMIT).and(warp::body::json())
}
