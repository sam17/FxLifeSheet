use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    // Get the DATABASE_URL from the environment
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable must be set");

    // Pass the DATABASE_URL to the build script
    println!("cargo:rustc-env=DATABASE_URL={}", database_url);
}
