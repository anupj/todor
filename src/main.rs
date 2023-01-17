#![allow(dead_code)]
#![allow(unused_imports)]

mod todo;

use rand::{self, Rng};
use std::{collections::BTreeMap, error::Error, fmt, sync::Arc, thread};
use surrealdb::{
    sql::{self, Data},
    sql::{Thing, Value},
    Datastore, Session,
};
use tokio;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let ds = Datastore::new("memory").await?;
    let ds = Datastore::new("file://test.db").await?;

    // `for_kv` - create a session with root authentication
    // `with_ns` - set the selected namespace for the sesh
    // `with_db` - set the db for the sesh
    let ses = Session::for_kv().with_ns("test").with_db("test");

    let entry_num = 2;
    for i in 0..entry_num {
        let mut vars = BTreeMap::new();
        vars.insert(
            "num".to_string(),
            sql::Value::Number(sql::Number::Int(i as i64)),
        );
        // The documentation for `execute()` is not that
        // helpful
        // The query will create a table *entry* and
        // will set the column `when` to current time
        // and the column `num` to $num
        ds.execute(
            r#"CREATE entry SET when = time::now(), num = $num;"#,
            &ses,
            Some(vars),
            false,
        )
        .await?;
    }

    let select_response = &ds
        .execute("SELECT * FROM entry;", &ses, None, false)
        .await?;

    let select_result = select_response[0].output().unwrap();

    let mut id_acc = vec![];

    if let Value::Array(rows) = select_result {
        for row in rows.iter() {
            if let Value::Object(obj) = row {
                println!("{:?}", obj);
                id_acc.push(obj.rid().unwrap());
            }
        }
    }

    println!("time taken:{}", select_response[0].speed());

    Ok(())
}
