use rusqlite::{params, Connection, Result};

#[allow(dead_code)] // This is just hear as Id is not really used yet but still should exist for a table
#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    age: i32,
}

fn main() -> Result<()> {
    // Create a connection to a new database file
    let conn = Connection::open("test.db")?;

    // Create table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            age   INTEGER
        )",
        [],
    )?;

    println!("Table created successfully.");

    // Create (Insert)
    let person = Person {
        id: 1,
        name: String::from("Alice"),
        age: 30,
    };
    conn.execute(
        "INSERT INTO person (name, age) VALUES (?1, ?2)",
        params![person.name, person.age],
    )?;
    println!("Created: {:?}", person);

    // Read
    let mut stmt = conn.prepare("SELECT id, name, age FROM person WHERE id = ?1")?;
    let person = stmt.query_row(params![1], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;
    println!("Read: {:?}", person);

    // Update
    conn.execute(
        "UPDATE person SET age = ?1 WHERE id = ?2",
        params![31, 1],
    )?;
    println!("Updated Alice's age to 31");

    // Read again to verify update
    let updated_person = stmt.query_row(params![1], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;
    println!("After update: {:?}", updated_person);

    // Delete
    conn.execute("DELETE FROM person WHERE id = ?1", params![1])?;
    println!("Deleted Alice from the database");

    // Try to read Alice again
    let result = stmt.query_row(params![1], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    });
    
    match result {
        Ok(_) => println!("Alice still exists (this shouldn't happen)"),
        Err(_) => println!("Alice no longer exists in the database"),
    }

    Ok(())
}
