use anyhow::{anyhow, Ok, Result};
use std::collections::BTreeMap;
use surrealdb::sql::{thing, Object, Value};
use surrealdb::{Datastore, Response, Session};

// convenience type
pub type DB = (Datastore, Session);

pub async fn get_datastore_session() -> Result<DB> {
    // Start by creating a datastore and session with
    // namespace and db name
    Ok((
        Datastore::new("memory").await?,
        Session::for_db("my_ns", "todo"),
    ))
}

pub async fn create_task(
    (ds, ses): &DB,
    title: &str,
    priority: u8,
    status: &str,
) -> Result<String> {
    let sql = "CREATE task CONTENT $data";
    let data_map: BTreeMap<String, Value> = [
        ("title".into(), title.into()),
        ("status".into(), status.into()),
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

pub async fn update_task<'a>(
    (ds, ses): &DB,
    task_id: &'a str,
    title: &'a str,
    status: &'a str,
    priority: u8,
) -> Result<&'a str> {
    let sql = "UPDATE $th MERGE $data RETURN id";
    let data_map: BTreeMap<String, Value> = [
        ("title".into(), title.into()),
        ("status".into(), status.into()),
        ("priority".into(), priority.into()),
    ]
    .into();
    let vars: BTreeMap<String, Value> = [
        // `thing` will parse task_id
        // and ensure its well formatted
        ("th".into(), thing(task_id)?.into()),
        ("data".into(), data_map.into()),
    ]
    .into();

    // we set `strict` to true
    // because we want to ensure
    // fields exist before updating
    ds.execute(sql, ses, Some(vars), true).await?;
    Ok(task_id)
}

pub async fn archive_task<'a>((ds, ses): &DB, task_id: &'a str) -> Result<&'a str> {
    let sql = "UPDATE $th MERGE $data RETURN id";
    let data_map: BTreeMap<String, Value> = [("status".into(), "Archived".into())].into();
    let vars: BTreeMap<String, Value> = [
        // `thing` will parse task_id
        // and ensure its well formatted
        ("th".into(), thing(task_id)?.into()),
        ("data".into(), data_map.into()),
    ]
    .into();

    // we set `strict` to true
    // because we want to ensure
    // fields exist before updating
    ds.execute(sql, ses, Some(vars), true).await?;
    Ok(task_id)
}

// delete it from the database
pub async fn hard_delete_task<'a>((ds, ses): &DB, task_id: &'a str) -> Result<&'a str> {
    // --- Delete
    let sql = "DELETE $th";
    let vars: BTreeMap<String, Value> = [("th".into(), thing(&task_id)?.into())].into();
    ds.execute(sql, ses, Some(vars), true).await?;
    Ok(task_id)
}

// Gets the data from db and returns a Vec of (id, title, status, priority)
pub async fn get_all_tasks((ds, ses): &DB) -> Result<Vec<(String, String, String, u8)>> {
    // --- Select
    let sql = "SELECT * from task";
    let response = ds.execute(sql, ses, None, false).await?;

    // Vec of (id, title, status, priority)
    let mut tasks: Vec<(String, String, String, u8)> = Vec::new();

    for object in into_iter_objects(response)? {
        let obj = object?;
        let id = obj.get("id").map(|id| id.to_string()).unwrap();
        let priority = obj
            .get("priority")
            .map(|p| p.to_string().parse::<u8>().unwrap())
            .unwrap();
        // Note to self
        // String values are returned from db surrounded by double quotes
        // so Archived is returned as "Archived", Bring Milk is returned as "Bring milk"
        let status = obj.get("status").map(|id| id.to_string()).unwrap();
        let title = obj.get("title").map(|id| id.to_string()).unwrap();

        tasks.push((id, title, status, priority));
    }
    Ok(tasks)
}
