use mongodb::{
    bson::doc,
    options::{AuthMechanism, ClientOptions, Credential},
    Client, Collection,
};
mod schema;
mod scrape;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[clap(short, long, env = "MONGO_URI", default_value = "mongodb://localhost:27017/")]
    uri: String,
    #[clap(long, env = "MONGO_USER", default_value = "admin")]
    user: String,
    #[clap(long, env = "MONGO_PASS", default_value = "admin")]
    pass: String,
    #[clap(short, long, env = "MONGO_DB", default_value = "iconic")]
    database: String,
    #[clap(short, long, env = "MONGO_COLL", default_value = "domains")]
    collection: String,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let args = Args::parse();

    let uri = args.uri.to_string();
    let db_name = args.database.to_string();
    let db_coll = args.collection.to_string();

    let db_cred = Credential::builder()
        .username(args.user)
        .password(args.pass)
        .mechanism(AuthMechanism::ScramSha256)
        .source("admin".to_string())
        .build();

    let mut db_opts = ClientOptions::parse_async(uri).await?;
    db_opts.credential = Some(db_cred);
    db_opts.app_name = Some("iconic".to_string());

    let client = Client::with_options(db_opts)?;

    let database = client.database(db_name.as_str());
    let domains: Collection<schema::Domain> = database.collection(db_coll.as_str());

    let a_domain = domains.find_one(doc! {}, None).await?;
    // Print the document
    println!("Found a domain:\n{:#?}", a_domain);

    // scrape it
    let icons = scrape::scrape_one(a_domain.unwrap(), Some(true)).await;
    println!("Found icons:\n{:#?}", icons);

    Ok(())
}
