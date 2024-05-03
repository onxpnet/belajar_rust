use dotenvy::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let env_test = env::var("ENV_TEST_DARI_FILE").unwrap();
    let database_url = env::var("DATABASE_URL").unwrap();
    println!("Hello, world! 123");
    println!("{}", env_test);

    // bikin koneksi ke DB
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await;

    // insert
    // let _ = sqlx::query("INSERT INTO actix_user(id, name, email) VALUES (1,'test','test')")
    //     .execute(&pool.unwrap())
    //     .await;

    // read
    #[derive(Debug)]
    struct ActixUser {
        id: i32,
        name: String,
        email: String,
    }
    let actix_users = sqlx::query_as!(ActixUser, "SELECT * FROM actix_user")
        .fetch_all(&pool.unwrap())
        .await
        .unwrap();

    println!("{}", actix_users[0].id);
    println!("{}", actix_users[0].name);
    println!("{}", actix_users[0].email);

    Ok(())
}
