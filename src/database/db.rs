use mysql::prelude::*;
use mysql::*;

#[derive(Debug, PartialEq, Eq)]
struct Request {
    data: String,
}

fn connect() -> Result<mysql::PooledConn> {
    let url = "mysql://root:root@localhost:3306/db_name";

    let pool = Pool::new(url)?;

    return Ok(pool.get_conn()?);
}

pub fn create_table() -> Result<()> {
    let mut conn = connect().unwrap();

    conn.query_drop(
        r"CREATE TABLE requests (
            data text
        )",
    )?;

    Ok(())
}

pub fn insert(data: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut conn = connect().unwrap();

    let requests = Request {
        data: data.to_string(),
    };

    conn.exec_drop("INSERT INTO requests (data) VALUES (?)", (requests.data,))?;

    Ok(())
}
