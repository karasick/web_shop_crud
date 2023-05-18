use std::env;
use mongodb::{Database, Client, error::Error as MongodbError};

pub async fn get_mongodb_connection() -> Result<Database, MongodbError> {
    let uri = match env::var("MONGODB_URI") {
        Ok(v) => v.to_string(),
        Err(_) => panic!("Error loading MONGODB_URI env variable"),
    };
    let db_name = match env::var("MONGODB_DATABASE") {
        Ok(v) => v.to_string(),
        Err(_) => panic!("Error loading MONGODB_DATABASE env variable"),
    };


    println!("Connecting to MongoDB");
    let client = Client::with_uri_str(uri).await?;

    println!("List of databases:");
    let database_names = client.list_database_names(None, None).await?;
    let db_list = match database_names.is_empty() {
        false => database_names.join(", "),
        true => String::from("0 databases available"),
    };
    println!("{}", db_list);

    println!("Accessing {db_name} database");
    let db = client.database(db_name.as_str());
    println!("Connected to {db_name}");

    println!("List of collections:");
    let collection_names = db.list_collection_names(None).await?;
    let col_list = match collection_names.is_empty() {
        false => collection_names.join(", "),
        true => String::from("0 collections available"),
    };
    println!("{}", col_list);

    Ok(db)
}