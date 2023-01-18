#![allow(unused)]

use crate::todo::TodoList;
use anyhow::{anyhow, Ok, Result};
use std::collections::BTreeMap;
use surrealdb::sql::{thing, Datetime, Object, Thing, Value};
use surrealdb::{Datastore, Response, Session};

// convenience type
pub type DB = (Datastore, Session);

pub async fn get_datastore_session() -> Result<DB> {
    // Start by creating a datastore and session with
    // namespace and db name
    Ok((
        Datastore::new("memory").await?,
        Session::for_db("", "my_db"),
    ))
}

pub async fn create_task((ds, ses): &DB, title: &str, priority: i32) -> Result<String> {
    let sql = "CREATE task CONTENT $data";

    let data_map: BTreeMap<String, Value> = [
        ("title".into(), title.into()),
        ("priority".into(), priority.into()),
    ]
    .into();

    let vars: BTreeMap<String, Value> = [("data".into(), data_map.into())].into();

    // setting `strict` param to `false`
    // because we want to create table/column
    // that might not be pre-defined in the db
    let ress = ds.execute(sql, ses, Some(vars), false).await?;
    into_iter_objects(ress)?
        .next()
        .transpose()?
        .and_then(|obj| obj.get("id").map(|id| id.to_string()))
        .ok_or_else(|| anyhow!("No id returned"))
}

fn into_iter_objects(ress: Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>> {
    let res = ress.into_iter().next().map(|rp| rp.result).transpose()?;

    match res {
        Some(Value::Array(arr)) => {
            let it = arr.into_iter().map(|v| match v {
                Value::Object(object) => Ok(object),
                _ => Err(anyhow!("A record was not an Object")),
            });
            Ok(it)
        }
        _ => Err(anyhow!("No records founds")),
    }
}

pub async fn update_task((ds, ses): &DB, task_id: String) -> Result<String> {
    let sql = "UPDATE $th MERGE $data RETURN id";
    let data_map: BTreeMap<String, Value> = [
        ("title".into(), "Task 02 UPDATED".into()),
        ("done".into(), true.into()),
    ]
    .into();
    let vars: BTreeMap<String, Value> = [
        // `thing` will parse task_id
        // and ensure its well formatted
        ("th".into(), thing(&task_id)?.into()),
        ("data".into(), data_map.into()),
    ]
    .into();

    // we set `strict` to true
    // because we want to ensure
    // fields exist before updating
    ds.execute(sql, ses, Some(vars), true).await?;
    Ok(task_id)
}

pub async fn delete_task((ds, ses): &DB, task_id: String) -> Result<String> {
    // --- Delete
    let sql = "DELETE $th";
    let vars: BTreeMap<String, Value> = [("th".into(), thing(&task_id)?.into())].into();
    ds.execute(sql, ses, Some(vars), true).await?;
    Ok(task_id)
}

pub async fn get_all_tasks((ds, ses): &DB) -> Result<()> {
    // --- Select
    let sql = "SELECT * from task";
    let ress = ds.execute(sql, ses, None, false).await?;
    for object in into_iter_objects(ress)? {
        let task = object?;
        // let key_value: BTreeMap<String, _> = task.into();

        println!("record {}", task);
    }
    Ok(())
}
