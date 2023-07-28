/*
* Llamar a una API es una de las tareas más comunes en programación.
*
* Implementa una llamada HTTP a una API (la que tú quieras) y muestra su
* resultado a través de la terminal.
*/

use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Record {
    description: String,
    amount: u32,
    id: u32,
}

#[tokio::main]
pub async fn apicall() -> Result<(), Error> {
    let request_url = format!("http://localhost:5000/api/records");
    println!("{request_url}");
    let response = reqwest::get(&request_url).await?;
    let records: Vec<Record> = response.json().await?;
    for elem in records {
        println!("{:?}", elem);
    }
    Ok(())
}
