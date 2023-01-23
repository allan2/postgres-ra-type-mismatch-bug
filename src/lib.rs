use postgres::Client;
use postgres_types::ToSql;

/// Inserts multiple records in a single INSERT statement.
pub fn insert_multirow(mut conn: Client) -> Result<u64, postgres::Error> {
    let statement = "INSERT INTO foo (a, b) VALUES ($1, $2), ($3, $4)";

    let mut params = Vec::<&(dyn ToSql + Sync)>::with_capacity(4);
    
    let ids = vec![1, 2];
    for id in &ids {
        params.push(id);
        params.push(&"s");
    }

    conn.execute(statement, &params)
}