mod narwhal;
use std::error::Error;
fn main() -> Result<(), Box<(dyn Error)>> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target");
    let mut nw = narwhal::Narwhal::new(&path.to_string_lossy());
    nw.execute("test_db", "CREATE TABLE users (id integer, name varchar)")
        .unwrap();
    nw.execute("test_db", "INSERT INTO users VALUES (1, 'Alice')")
        .unwrap();
    nw.execute("test_db", "INSERT INTO users VALUES (2, 'Bob')")
        .unwrap();
    let result = nw
        .execute("test_db", "SELECT name FROM users WHERE id = 2")
        .unwrap();
    println!("{:?}", result);
    Ok(())
}
