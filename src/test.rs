use rusqlite::{Connection, Result};

#[test]
fn test_database_state() -> Result<()> {
    // Connect to the database
    let conn = Connection::open("test.db")?;

    // Check if the table is empty (Alice should have been deleted) and exists
    let count: i32 = conn.query_row("SELECT COUNT(*) FROM person", [], |row| row.get(0))?;
    assert_eq!(count, 0, "person table should be empty");

    // Try to find Alice (should fail)
    let result = conn.query_row("SELECT * FROM person WHERE name = 'Alice'", [], |_| {
        Ok(())
    });
    assert!(result.is_err(), "Alice should not exist in the database");

    Ok(())
}
