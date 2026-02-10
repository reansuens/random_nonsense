use rusqlite::types::Value;
use rusqlite::{Connection, Result};
fn main() -> Result<()> {
    let mut stitches: Vec<String> = Vec::new(); //random vector. EXCUSE THE NAME.
    let strength: usize = 8;

    let data_base = Connection::open("./aerospace.db")?; //open db a normal name ???
    let mut dust = data_base.prepare( 
        "SELECT name FROM sqlite_master WHERE type='table' AND name !='sqlite_sequence';",
    )?; // WHO TF CALL THEIR QUERY DUST???? yh ig it's me woooooo
    let tables = dust.query_map([], |row| row.get::<_, String>(0))?;
    let mut query: String;
    for table in tables {
        let table = table?;
 
        let limit = if table == "fundamentals" {
            strength * 8
        } else if table == "punc" {
            strength * 2
        } else {
            strength
        };
        query = format!("SELECT * FROM {} ORDER BY RANDOM() LIMIT {}", table, limit);

        let mut stmt = data_base.prepare(&query)?;
//        let column_count = stmt.column_count();  // this is how you get column count in rusqlite ig.

        let rows = stmt.query_map([], |row| {
            let val: Value = row.get(1)?;
            if let Value::Text(txt) = val {
                Ok(Some(txt))
            } else {
                Ok(None)
            }
        })?;
        for row_output in rows {
            if let Some(text) = row_output? {
                stitches.push(text)
            }
        }
    }

    use rand::seq::SliceRandom;
    let mut rng_fut = rand::rng();
    stitches.shuffle(&mut rng_fut);       // btw it took me hours to figure out if there is a .shuffle() and turned out yh I only needed to use that crate above
    let output = stitches.join(" "); 
 //this will group up the output into a single line. also if you want you can make the output crazy long
// with the exact same energy if you put the output to be inside a for loop like this 
// for term in stitches { 
// let output = stitches.join(" ");
// println!("{output}"); }

    println!("{output}.");
    Ok(())
}
