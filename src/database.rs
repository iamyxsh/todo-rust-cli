use postgres::{Client, NoTls, Row};

pub fn return_client() -> Client {
    let client =
        Client::connect("postgres://postgres:docker@127.0.0.1:5432/postgres", NoTls).unwrap();
    client
}

pub fn create_table(client: &mut Client) {
    client
        .batch_execute(
            "
    CREATE TABLE IF NOT EXISTS todo (
        id          SERIAL PRIMARY KEY,
        task        TEXT NOT NULL,
        completed   BOOLEAN
    )
",
        )
        .unwrap();
}

pub fn get_all_todos(client: &mut Client) -> Vec<Row> {
    let todos = client
        .query("SELECT id, task, completed from todo", &[])
        .unwrap();
    todos
}

pub fn add_todo(client: &mut Client, todo: String) -> bool {
    let tx = client.query(
        "INSERT INTO todo (task, completed) VALUES ($1, $2)",
        &[&todo, &false],
    );

    tx.is_ok()
}

pub fn remove_todo(client: &mut Client, id: i32) -> bool {
    let tx = client.query("DELETE FROM todo WHERE id = $1", &[&id]);
    tx.is_ok()
}

pub fn mark_completed(client: &mut Client, id: i32) -> bool {
    let tx = client.query("UPDATE todo SET completed = true WHERE id = $1", &[&id]);
    tx.is_ok()
}

pub fn mark_uncompleted(client: &mut Client, id: i32) -> bool {
    let tx = client.query("UPDATE todo SET completed = false WHERE id = $1", &[&id]);
    tx.is_ok()
}
