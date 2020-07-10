use tokio_postgres::{Error, NoTls};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let conn_string = format!(
        "host={} port={} user={} password={} dbname={} application_name={}",
        "127.0.0.1", 5432, "postgres", "mys3cr3t!password", "mydatabase", "test-rust-pgsql"
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
