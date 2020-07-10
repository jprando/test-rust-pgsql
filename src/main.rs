use dotenv;
use tokio_postgres::{Error, NoTls};

#[tokio::main]
async fn main() -> Result<(), Error> {
    
    let pg_host = dotenv::var("PGHOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let pg_port = dotenv::var("PGPORT").unwrap_or_else(|_| "5432".to_owned());
    let pg_user = dotenv::var("PGUSER").unwrap_or_else(|_| "postgres".to_owned());
    let pg_pass = dotenv::var("PGPASS").unwrap_or_else(|_| "".to_owned());
    let pg_data = dotenv::var("PGDATA").unwrap_or_else(|_| "postgres".to_owned());
    
    let conn_string = format!(
        "host={} port={} user={} password={} dbname={} application_name={}",
        pg_host, pg_port, pg_user, pg_pass, pg_data, "test-rust-pgsql"
    );

    let (client, connection) = tokio_postgres::connect(conn_string.as_str(), NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprint!("connection error {}", e)
        }
    });

    let select = "SELECT name FROM USERs ORDER BY name";

    let rows = client.query(select, &[]).await?;

    for i in rows {
        let user_name: &str = i.get(0);
        println!("{}", user_name);
    }

    Ok(())
}
